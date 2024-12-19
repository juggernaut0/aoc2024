use std::collections::HashMap;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (towels, designs) = parse_input(&input);

        designs
            .into_iter()
            .filter(|design| {
                let mut memo = HashMap::new();
                memo.insert("", 1);
                get_matches(&towels, design, &mut memo) > 0
            })
            .count()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let (towels, designs) = parse_input(&input);

        designs
            .into_iter()
            .map(|design| {
                let mut memo = HashMap::new();
                memo.insert("", 1);
                get_matches(&towels, design, &mut memo)
            })
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
    let mut all_results = 0;
    for towel in towels {
        if let Some(rest) = design.strip_prefix(towel) {
            all_results += get_matches(towels, rest, memo);
        }
    }
    memo.insert(design, all_results);
    all_results
}
