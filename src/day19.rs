use std::collections::HashMap;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (towels, designs) = parse_input(&input);

        let mut memo = HashMap::new();
        memo.insert("", 1);

        designs
            .into_iter()
            .filter(|design| get_matches(&towels, design, &mut memo) > 0)
            .count()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let (towels, designs) = parse_input(&input);

        let mut memo = HashMap::new();
        memo.insert("", 1);

        designs
            .into_iter()
            .map(|design| get_matches(&towels, design, &mut memo))
            .sum::<u64>()
            .to_string()
    }
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels_str, designs_str) = input.split_once("\n\n").unwrap();
    let towels = towels_str.split(", ").collect();
    let designs = designs_str.lines().collect();
    (towels, designs)
}

fn get_matches<'a>(towels: &[&str], design: &'a str, memo: &mut HashMap<&'a str, u64>) -> u64 {
    if let Some(&result) = memo.get(design) {
        return result;
    }

    let all_results = towels
        .iter()
        .filter_map(|towel| design.strip_prefix(towel))
        .map(|rest| get_matches(towels, rest, memo))
        .sum();

    memo.insert(design, all_results);
    all_results
}
