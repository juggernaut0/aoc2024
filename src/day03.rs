pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let mut sum = 0;
        let mut i = 0;
        let s = input.chars().collect::<Vec<char>>();
        loop {
            let (next_i, mul) = is_mul(&s, i);
            if let Some(mul) = mul {
                sum += mul;
            }
            if next_i >= s.len() {
                break;
            }
            i = next_i;
        }
        sum.to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let mut sum = 0;
        let mut i = 0;
        let s = input.chars().collect::<Vec<char>>();
        let mut enabled = true;
        loop {
            if is_do(&s, i) {
                enabled = true;
                i += 4;
            } else if is_dont(&s, i) {
                enabled = false;
                i += 7;
            } else {
                let (next_i, mul) = is_mul(&s, i);
                if let Some(mul) = mul {
                    if enabled {
                        sum += mul;
                    }
                }
                if next_i >= s.len() {
                    break;
                }
                i = next_i;
            }
        }
        sum.to_string()
    }
}

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn is_mul(s: &[char], start: usize) -> (usize, Option<i32>) {
    if start + 8 >= s.len() {
        return (start + 8, None);
    }
    if s[start..start + 4] != ['m', 'u', 'l', '('] {
        return (start + 1, None);
    }
    let mut i = start + 4;
    let mut count = 0;
    let mut second = false;
    let mut a_str = String::new();
    let mut b_str = String::new();
    while i < s.len() {
        if NUMBERS.contains(&s[i]) {
            if second {
                b_str.push(s[i]);
            } else {
                a_str.push(s[i]);
            }
            count += 1;
        } else if count == 0 {
            return (i + 1, None);
        } else if !second && s[i] == ',' {
            second = true;
            count = 0;
        } else if second && s[i] == ')' {
            let a = a_str.parse::<i32>().unwrap();
            let b = b_str.parse::<i32>().unwrap();
            return (i + 1, Some(a * b));
        } else {
            return (i + 1, None);
        }
        i += 1;
    }
    (i, None)
}

fn is_do(s: &[char], start: usize) -> bool {
    if start + 4 >= s.len() {
        return false;
    }
    s[start..start + 4] == ['d', 'o', '(', ')']
}

fn is_dont(s: &[char], start: usize) -> bool {
    if start + 7 >= s.len() {
        return false;
    }
    s[start..start + 7] == ['d', 'o', 'n', '\'', 't', '(', ')']
}
