use aoc::Counter;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        solve(&input, 25)
    }

    fn solve_2(&self, input: String) -> String {
        solve(&input, 75)
    }
}

fn solve(input: &str, times: usize) -> String {
    let mut stones: Counter<_> = input.split_ascii_whitespace().map(str::to_string).collect();
    for _ in 0..times {
        blink(&mut stones);
    }
    stones.total().to_string()
}

fn blink(stones: &mut Counter<String>) {
    let mut new_stones = Counter::new();
    for (stone, &count) in stones.iter() {
        if stone == "0" {
            new_stones.count_n("1".to_string(), count);
        } else if stone.len() % 2 == 0 {
            let half = stone.len() / 2;
            let left = stone[..half].to_string();
            let right = stone[half..].parse::<u32>().unwrap().to_string();
            new_stones.count_n(left, count);
            new_stones.count_n(right, count);
        } else {
            let v: u64 = stone.parse().unwrap();
            new_stones.count_n((v * 2024).to_string(), count);
        }
    }
    *stones = new_stones;
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::{init_test_logging, Solution};

    #[test]
    fn part_1() {
        init_test_logging();

        let res = Solution.solve_1("125 17".to_string());
        assert_eq!("55312", res);
    }
}
