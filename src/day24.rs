use aoc::{parse_lines, parse_lines_with};
use std::collections::HashMap;
use std::mem::{replace, take};
use std::str::FromStr;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (mut values, mut gates) = parse_input(&input);
        while !gates.is_empty() {
            let gate = gates
                .iter()
                .position(|gate| {
                    values.contains_key(&gate.input_1) && values.contains_key(&gate.input_2)
                })
                .expect("No gate can be evaluated");
            let gate = gates.remove(gate);
            gate.eval(&mut values);
        }
        let mut z_keys = values
            .keys()
            .filter(|key| key.starts_with('z'))
            .collect::<Vec<_>>();
        z_keys.sort();
        z_keys.reverse();
        let mut res = 0;
        for key in z_keys {
            log::debug!("processing {key} = {}", values[key]);
            res <<= 1;
            res |= u64::from(values[key]);
        }
        log::debug!("res = {res:#b}");
        res.to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let (_, gates) = parse_input(&input);

        let mut carry_in = gates
            .iter()
            .find(|gate| gate.matches("x00", "y00", Operation::And))
            .unwrap()
            .output
            .as_str();
        for bit_i in 1..44 {
            carry_in = verify_full_adder(&gates, bit_i, carry_in);
        }

        "Finished by hand".to_string()
    }
}

fn parse_input(input: &str) -> (HashMap<String, bool>, Vec<Gate>) {
    let (initials_str, gates_str) = input.split_once("\n\n").unwrap();
    let initials = parse_lines_with(initials_str, |line| {
        let (name, value) = line.split_once(": ").unwrap();
        (name.to_string(), value == "1")
    })
    .collect();
    let gates = parse_lines(gates_str).collect();
    (initials, gates)
}

struct Gate {
    input_1: String,
    input_2: String,
    output: String,
    operation: Operation,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (inputs, output) = s.split_once(" -> ").unwrap();
        let mut input_parts = inputs.split_ascii_whitespace();
        let input_1 = input_parts.next().unwrap();
        let operation = match input_parts.next().unwrap() {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => panic!("Invalid operation"),
        };
        let input_2 = input_parts.next().unwrap();
        Ok(Gate {
            input_1: input_1.to_string(),
            input_2: input_2.to_string(),
            output: output.to_string(),
            operation,
        })
    }
}

impl Gate {
    fn eval(&self, values: &mut HashMap<String, bool>) {
        values.insert(
            self.output.clone(),
            match self.operation {
                Operation::And => values[&self.input_1] & values[&self.input_2],
                Operation::Or => values[&self.input_1] | values[&self.input_2],
                Operation::Xor => values[&self.input_1] ^ values[&self.input_2],
            },
        );
    }

    fn matches(&self, a: &str, b: &str, op: Operation) -> bool {
        let has_inputs =
            self.input_1 == a && self.input_2 == b || self.input_1 == b && self.input_2 == a;
        has_inputs && self.operation == op
    }
}

fn verify_full_adder<'a>(gates: &'a [Gate], bit_i: u8, carry_in: &str) -> &'a str {
    let xnn = format!("x{bit_i:02}");
    let ynn = format!("y{bit_i:02}");
    let first_xor = gates
        .iter()
        .find(|gate| gate.matches(&xnn, &ynn, Operation::Xor))
        .unwrap();
    let first_and = gates
        .iter()
        .find(|gate| gate.matches(&xnn, &ynn, Operation::And))
        .unwrap();
    let Some(second_xor) = gates
        .iter()
        .find(|gate| gate.matches(&first_xor.output, carry_in, Operation::Xor))
    else {
        panic!(
            "could not find second xor at bit {bit_i}: {} XOR {carry_in}",
            &first_xor.output
        );
    };
    let znn = format!("z{bit_i:02}");
    assert_eq!(
        second_xor.output, znn,
        "second xor output at bit {bit_i} was {} instead of expected {}",
        &second_xor.output, znn
    );
    let Some(second_and) = gates
        .iter()
        .find(|gate| gate.matches(&first_xor.output, carry_in, Operation::And))
    else {
        panic!(
            "could not find second and at bit {bit_i}: {} AND {carry_in}",
            &first_xor.output
        );
    };
    let Some(or) = gates
        .iter()
        .find(|gate| gate.matches(&first_and.output, &second_and.output, Operation::Or))
    else {
        panic!(
            "could not find or at bit {bit_i}: {} OR {}",
            &first_xor.output, &second_and.output
        );
    };
    &or.output
}

#[allow(dead_code)] // used for solving by hand
fn swap_outputs(gates: &mut [Gate], a: &str, b: &str) {
    let first = gates.iter_mut().find(|gate| gate.output == a).unwrap();
    let a = take(&mut first.output);

    let second = gates.iter_mut().find(|gate| gate.output == b).unwrap();
    let b = replace(&mut second.output, a);

    let third = gates
        .iter_mut()
        .find(|gate| gate.output.is_empty())
        .unwrap();
    third.output = b;
}

/*
given a bit_i "nn" and an unverified carry_in
find xnn XOR ynn -> t (it has to exist) t is unverified
find a XOR/AND pair for carry_in and t. If found, carry_in and t are verified
    If not found, find an XOR/AND pair for carry_in and some "other", carry_in is verified and t is swapped with other
    If not found, so same for t and some "other".
    If pair not found for either carry_in or t, all hope is lost. They are both swapped but we don't know with who
    If output of XOR is not znn, it has been swapped with znn
    Call output of AND c1. c1 is unverified
find xnn AND ynn -> c2 (it has to exist) c2 is unverified
find c1 OR c2 -> carry_out
    If found, c1 and c2 are verified. carry_out is unverified. Move onto next bit_i
    If not found, either c1 or c2 is swapped but we don't know which. All hope is lost

 */
