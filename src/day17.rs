pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let mut computer = parse_input(&input);
        computer.run();
        let mut output = String::new();
        for (i, n) in computer.output.into_iter().enumerate() {
            if i != 0 {
                output.push(',');
            }
            output.push_str(&n.to_string());
        }
        output
    }

    fn solve_2(&self, input: String) -> String {
        let computer = parse_input(&input);
        let target = computer.program;
        let n = find_digit(&target, target.len() - 1, 0).unwrap();
        n.to_string()
    }
}

fn parse_input(s: &str) -> Computer {
    let mut lines = s.lines();
    let a = lines.next().unwrap()[12..].parse().unwrap();
    let b = lines.next().unwrap()[12..].parse().unwrap();
    let c = lines.next().unwrap()[12..].parse().unwrap();
    lines.next().unwrap(); // skip empty line
    let program = lines.next().unwrap()[9..]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    Computer {
        a,
        b,
        c,
        program,
        pc: 0,
        output: Vec::new(),
    }
}

struct Computer {
    a: u32,
    b: u32,
    c: u32,
    program: Vec<u8>,
    pc: usize,
    output: Vec<u8>,
}

impl Computer {
    fn run(&mut self) {
        while self.pc < self.program.len() {
            self.run_instruction();
        }
    }

    fn run_instruction(&mut self) {
        let opcode = self.program[self.pc];
        self.pc += 1;
        match opcode {
            0 => {
                let denom = 2_u32.pow(self.read_combo_operand());
                self.a /= denom;
            }
            1 => {
                self.b ^= self.read_literal_operand();
            }
            2 => {
                self.b = self.read_combo_operand() % 8;
            }
            3 => {
                if self.a != 0 {
                    self.pc = self.read_literal_operand() as usize;
                    return;
                }
            }
            4 => {
                self.b ^= self.c;
            }
            5 => {
                self.output.push((self.read_combo_operand() % 8) as u8);
            }
            6 => {
                let denom = 2_u32.pow(self.read_combo_operand());
                self.b = self.a / denom;
            }
            7 => {
                let denom = 2_u32.pow(self.read_combo_operand());
                self.c = self.a / denom;
            }
            _ => {
                panic!("Invalid opcode {opcode}");
            }
        }
        self.pc += 1;
    }

    fn read_literal_operand(&self) -> u32 {
        self.program[self.pc].into()
    }

    fn read_combo_operand(&self) -> u32 {
        let operand = self.program[self.pc];
        match operand {
            0..=3 => operand.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid combo operand {operand}"),
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
fn f(a: u64) -> u8 {
    // TODO make this generic across inputs?
    let x = 5; // program[3]
    let y = 6; // program[7]
    (((((a % 8) ^ x) ^ y) ^ (a / 2_u64.pow(((a % 8) ^ x) as u32))) % 8) as u8
}

fn find_digit(target: &[u8], i: usize, n: u64) -> Option<u64> {
    log::info!("target: {} n: {n:#o}", target[i]);
    for d in 0..8 {
        if f(n + d) == target[i] {
            if i == 0 {
                return Some(n + d);
            }
            if let Some(m) = find_digit(target, i - 1, (n + d) * 8) {
                return Some(m);
            }
        }
    }
    None
}
