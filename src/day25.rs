use aoc::{Grid, Point};

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (keys, locks) = parse_input(&input);
        keys.iter()
            .flat_map(|key| locks.iter().map(move |lock| (key, lock)))
            .filter(|(key, lock)| check(key, lock))
            .count()
            .to_string()
    }

    fn solve_2(&self, _input: String) -> String {
        "Merry Christmas!".to_string()
    }
}

#[allow(clippy::cast_sign_loss)]
fn parse_input(input: &str) -> (Vec<[i32; 5]>, Vec<[i32; 5]>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for diagram in input.split("\n\n") {
        let diagram: Grid<char> = diagram.parse().unwrap();
        let mut repr = [0; 5];
        if (0..5).all(|x| diagram[Point(x, 0)] == '#') {
            for x in 0..5 {
                for y in 1..diagram.height() {
                    if diagram[Point(x, y)] == '#' {
                        repr[x as usize] = y;
                    } else {
                        break;
                    }
                }
            }
            locks.push(repr);
        } else {
            for x in 0..5 {
                for y in 1..diagram.height() {
                    if diagram[Point(x, diagram.height() - y - 1)] == '#' {
                        repr[x as usize] = y;
                    } else {
                        break;
                    }
                }
            }
            keys.push(repr);
        }
    }
    (keys, locks)
}

fn check(key: &[i32; 5], lock: &[i32; 5]) -> bool {
    key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5)
}
