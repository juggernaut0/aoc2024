use aoc::Counter;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (mut left, mut right) = parse_lists(&input);
        left.sort_unstable();
        right.sort_unstable();
        left.into_iter()
            .zip(right)
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let (left, right) = parse_lists(&input);
        let right_counts: Counter<_, i32> = right.into_iter().collect();
        left.into_iter()
            .map(|n| n * right_counts.get(&n))
            .sum::<i32>()
            .to_string()
    }
}

fn parse_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|s| {
            let mut parts = s.split_ascii_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip()
}
