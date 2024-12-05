use std::collections::HashSet;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (rules, updates) = parse_input(&input);
        updates
            .into_iter()
            .filter(|update| validate_update(update, &rules))
            .map(|update| middle_number(&update))
            .sum::<i32>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let (rules, updates) = parse_input(&input);
        updates
            .into_iter()
            .filter(|update| !validate_update(update, &rules))
            .map(|update| reorder(update, &rules))
            .map(|update| middle_number(&update))
            .sum::<i32>()
            .to_string()
    }
}

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let rules = rules_str
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .collect();

    let updates = updates_str
        .lines()
        .map(|line| line.split(',').map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn validate_update(update: &[i32], rules: &HashSet<(i32, i32)>) -> bool {
    rules.iter().all(|(a, b)| {
        let ai = update.iter().position(|n| n == a);
        let bi = update.iter().position(|n| n == b);
        match (ai, bi) {
            (Some(ai), Some(bi)) => ai < bi,
            _ => true,
        }
    })
}

fn middle_number(update: &[i32]) -> i32 {
    update[update.len() / 2]
}

fn reorder(mut update: Vec<i32>, rules: &HashSet<(i32, i32)>) -> Vec<i32> {
    update.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            std::cmp::Ordering::Less
        } else if rules.contains(&(*b, *a)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    update
}
