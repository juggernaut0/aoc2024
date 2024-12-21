use aoc::{Counter, Point};
use std::fmt::Debug;
use std::mem::take;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        solve(&input, 2)
    }

    fn solve_2(&self, input: String) -> String {
        solve(&input, 25)
    }
}

fn solve(input: &str, nested: u32) -> String {
    let mut total = 0;
    for line in input.lines() {
        let mut robot = KeypadRobot {
            robot: DoorRobot::new(),
            nested,
        };
        let mut full_inputs = Counter::new();
        for c in line.chars() {
            log::info!("starting to press {c}");
            full_inputs.extend(robot.press(c));
        }
        let numeric: u64 = line[0..3].parse().unwrap();
        log::debug!("full inputs {full_inputs:?}");
        log::info!("complexity is {} * {numeric}", full_inputs.total());
        total += numeric * full_inputs.total();
    }
    total.to_string()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum KeyInput {
    U,
    D,
    L,
    R,
    A,
}

impl From<KeyInput> for char {
    fn from(value: KeyInput) -> Self {
        match value {
            KeyInput::U => '^',
            KeyInput::D => 'v',
            KeyInput::L => '<',
            KeyInput::R => '>',
            KeyInput::A => 'A',
        }
    }
}

impl Debug for KeyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<char>::into(*self))
    }
}

fn to_debug_string(input: &[KeyInput]) -> String {
    input.iter().map(|ki| Into::<char>::into(*ki)).collect()
}

struct DoorRobot {
    pointer: Point,
}

impl DoorRobot {
    fn new() -> Self {
        DoorRobot {
            pointer: Point(2, 3),
        }
    }
}

impl DoorRobot {
    fn press(&mut self, c: char) -> Counter<(KeyInput, KeyInput)> {
        let mut inputs = Vec::new();
        inputs.push(KeyInput::A);
        let target = match c {
            '7' => Point(0, 0),
            '8' => Point(1, 0),
            '9' => Point(2, 0),
            '4' => Point(0, 1),
            '5' => Point(1, 1),
            '6' => Point(2, 1),
            '1' => Point(0, 2),
            '2' => Point(1, 2),
            '3' => Point(2, 2),
            '0' => Point(1, 3),
            'A' => Point(2, 3),
            _ => panic!("Invalid input"),
        };
        if self.pointer.1 == 3 && target.0 == 0 {
            // up first, then left
            inputs.extend((0..(self.pointer.1 - target.1)).map(|_| KeyInput::U));
            inputs.extend((0..(self.pointer.0 - target.0)).map(|_| KeyInput::L));
        } else if target.1 == 3 && self.pointer.0 == 0 {
            // right then down
            inputs.extend((0..(target.0 - self.pointer.0)).map(|_| KeyInput::R));
            inputs.extend((0..(target.1 - self.pointer.1)).map(|_| KeyInput::D));
        } else if self.pointer.0 > target.0 {
            inputs.extend((0..(self.pointer.0 - target.0)).map(|_| KeyInput::L));
            if self.pointer.1 > target.1 {
                inputs.extend((0..(self.pointer.1 - target.1)).map(|_| KeyInput::U));
            } else {
                inputs.extend((0..(target.1 - self.pointer.1)).map(|_| KeyInput::D));
            }
        } else {
            if self.pointer.1 > target.1 {
                inputs.extend((0..(self.pointer.1 - target.1)).map(|_| KeyInput::U));
            } else {
                inputs.extend((0..(target.1 - self.pointer.1)).map(|_| KeyInput::D));
            }
            inputs.extend((0..(target.0 - self.pointer.0)).map(|_| KeyInput::R));
        }
        self.pointer = target;
        inputs.push(KeyInput::A);
        log::debug!("for input {c} inputs are {}", to_debug_string(&inputs));
        inputs.windows(2).map(|w| (w[0], w[1])).collect()
    }
}

struct KeypadRobot {
    robot: DoorRobot,
    nested: u32,
}

impl KeypadRobot {
    #[allow(clippy::too_many_lines)]
    fn press(&mut self, c: char) -> Counter<(KeyInput, KeyInput)> {
        let mut res = self.robot.press(c);
        for _ in 0..self.nested {
            let old = take(&mut res);
            for (pe, c) in old {
                #[allow(clippy::match_same_arms)]
                match pe {
                    (KeyInput::A, KeyInput::A) => {
                        res.count_n((KeyInput::A, KeyInput::A), c);
                    }
                    (KeyInput::A, KeyInput::U) => {
                        res.count_n((KeyInput::A, KeyInput::L), c);
                        res.count_n((KeyInput::L, KeyInput::A), c);
                    }
                    (KeyInput::A, KeyInput::D) => {
                        res.count_n((KeyInput::A, KeyInput::L), c);
                        res.count_n((KeyInput::L, KeyInput::D), c);
                        res.count_n((KeyInput::D, KeyInput::A), c);
                    }
                    (KeyInput::A, KeyInput::L) => {
                        res.count_n((KeyInput::A, KeyInput::D), c);
                        res.count_n((KeyInput::D, KeyInput::L), c);
                        res.count_n((KeyInput::L, KeyInput::L), c);
                        res.count_n((KeyInput::L, KeyInput::A), c);
                    }
                    (KeyInput::A, KeyInput::R) => {
                        res.count_n((KeyInput::A, KeyInput::D), c);
                        res.count_n((KeyInput::D, KeyInput::A), c);
                    }

                    (KeyInput::U, KeyInput::A) => {
                        res.count_n((KeyInput::A, KeyInput::R), c);
                        res.count_n((KeyInput::R, KeyInput::A), c);
                    }
                    (KeyInput::U, KeyInput::U) => {
                        res.count_n((KeyInput::A, KeyInput::A), c);
                    }
                    (KeyInput::U, KeyInput::D) => {
                        res.count_n((KeyInput::A, KeyInput::D), c);
                        res.count_n((KeyInput::D, KeyInput::A), c);
                    }
                    (KeyInput::U, KeyInput::L) => {
                        res.count_n((KeyInput::A, KeyInput::D), c);
                        res.count_n((KeyInput::D, KeyInput::L), c);
                        res.count_n((KeyInput::L, KeyInput::A), c);
                    }
                    (KeyInput::U, KeyInput::R) => {
                        res.count_n((KeyInput::A, KeyInput::D), c);
                        res.count_n((KeyInput::D, KeyInput::R), c);
                        res.count_n((KeyInput::R, KeyInput::A), c);
                    }

                    (KeyInput::D, KeyInput::A) => {
                        res.count_n((KeyInput::A, KeyInput::U), c);
                        res.count_n((KeyInput::U, KeyInput::R), c);
                        res.count_n((KeyInput::R, KeyInput::A), c);
                    }
                    (KeyInput::D, KeyInput::U) => {
                        res.count_n((KeyInput::A, KeyInput::U), c);
                        res.count_n((KeyInput::U, KeyInput::A), c);
                    }
                    (KeyInput::D, KeyInput::D) => {
                        res.count_n((KeyInput::A, KeyInput::A), c);
                    }
                    (KeyInput::D, KeyInput::L) => {
                        res.count_n((KeyInput::A, KeyInput::L), c);
                        res.count_n((KeyInput::L, KeyInput::A), c);
                    }
                    (KeyInput::D, KeyInput::R) => {
                        res.count_n((KeyInput::A, KeyInput::R), c);
                        res.count_n((KeyInput::R, KeyInput::A), c);
                    }

                    (KeyInput::L, KeyInput::A) => {
                        res.count_n((KeyInput::A, KeyInput::R), c);
                        res.count_n((KeyInput::R, KeyInput::R), c);
                        res.count_n((KeyInput::R, KeyInput::U), c);
                        res.count_n((KeyInput::U, KeyInput::A), c);
                    }
                    (KeyInput::L, KeyInput::U) => {
                        res.count_n((KeyInput::A, KeyInput::R), c);
                        res.count_n((KeyInput::R, KeyInput::U), c);
                        res.count_n((KeyInput::U, KeyInput::A), c);
                    }
                    (KeyInput::L, KeyInput::D) => {
                        res.count_n((KeyInput::A, KeyInput::R), c);
                        res.count_n((KeyInput::R, KeyInput::A), c);
                    }
                    (KeyInput::L, KeyInput::L) => {
                        res.count_n((KeyInput::A, KeyInput::A), c);
                    }
                    (KeyInput::L, KeyInput::R) => {
                        res.count_n((KeyInput::A, KeyInput::R), c);
                        res.count_n((KeyInput::R, KeyInput::R), c);
                        res.count_n((KeyInput::R, KeyInput::A), c);
                    }

                    (KeyInput::R, KeyInput::A) => {
                        res.count_n((KeyInput::A, KeyInput::U), c);
                        res.count_n((KeyInput::U, KeyInput::A), c);
                    }
                    (KeyInput::R, KeyInput::U) => {
                        res.count_n((KeyInput::A, KeyInput::L), c);
                        res.count_n((KeyInput::L, KeyInput::U), c);
                        res.count_n((KeyInput::U, KeyInput::A), c);
                    }
                    (KeyInput::R, KeyInput::D) => {
                        res.count_n((KeyInput::A, KeyInput::L), c);
                        res.count_n((KeyInput::L, KeyInput::A), c);
                    }
                    (KeyInput::R, KeyInput::L) => {
                        res.count_n((KeyInput::A, KeyInput::L), c);
                        res.count_n((KeyInput::L, KeyInput::L), c);
                        res.count_n((KeyInput::L, KeyInput::A), c);
                    }
                    (KeyInput::R, KeyInput::R) => {
                        res.count_n((KeyInput::A, KeyInput::A), c);
                    }
                };
            }
        }
        res
    }
}
