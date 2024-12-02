use aoc::parse_lines_with;
use std::collections::HashSet;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        parse_lines_with(&input, parse_line)
            .filter(|report| is_safe(report))
            .count()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        parse_lines_with(&input, parse_line)
            .filter(|report| is_safe_2(report))
            .count()
            .to_string()
    }
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let diffs: HashSet<_> = report.windows(2).map(|w| w[1] - w[0]).collect();
    let grad_incr = diffs.iter().all(|diff| (1..=3).contains(diff));
    let grad_decr = diffs.iter().all(|diff| (-3..=-1).contains(diff));
    grad_incr || grad_decr
}

fn is_safe_2(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut report = report.to_vec();
        report.remove(i);
        if is_safe(&report) {
            return true;
        }
    }
    false
}
