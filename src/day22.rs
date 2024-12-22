use aoc::{parse_lines, Counter};
use std::collections::HashSet;
use std::iter::once;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        parse_lines(&input)
            .map(|v| {
                let mut secret = Secret(v);
                secret.nth(1999).unwrap()
            })
            .sum::<i64>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let all_prices: Vec<Vec<i8>> = parse_lines(&input)
            .map(|v| {
                let secret = Secret(v);
                once(v)
                    .chain(secret.take(2000))
                    .map(|s| (s % 10) as i8)
                    .collect()
            })
            .collect();

        let all_diffs: Vec<Vec<i8>> = all_prices
            .iter()
            .map(|p| p.windows(2).map(|w| w[1] - w[0]).collect())
            .collect();

        let mut scores_for_diff = Counter::new();

        for (price_history, diffs) in all_prices.iter().zip(all_diffs.iter()) {
            let mut seen = HashSet::new();
            for (t, window) in diffs.windows(4).enumerate() {
                if seen.insert(window) {
                    scores_for_diff.count_n(window, i32::from(price_history[t + 4]));
                }
            }
        }

        scores_for_diff
            .into_iter()
            .map(|(_, v)| v)
            .max()
            .unwrap()
            .to_string()
    }
}

struct Secret(i64);

impl Secret {
    fn mix(&mut self, mixin: i64) {
        self.0 ^= mixin;
    }

    fn prune(&mut self) {
        self.0 %= 16_777_216;
    }
}

impl Iterator for Secret {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.mix(self.0 * 64);
        self.prune();

        self.mix(self.0 / 32);
        self.prune();

        self.mix(self.0 * 2048);
        self.prune();

        Some(self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let mut secret = Secret(123);
        assert_eq!(secret.next(), Some(15887950));
        assert_eq!(secret.next(), Some(16495136));
        assert_eq!(secret.next(), Some(527345));
        assert_eq!(secret.next(), Some(704524));
        assert_eq!(secret.next(), Some(1553684));
        assert_eq!(secret.next(), Some(12683156));
        assert_eq!(secret.next(), Some(11100544));
        assert_eq!(secret.next(), Some(12249484));
        assert_eq!(secret.next(), Some(7753432));
        assert_eq!(secret.next(), Some(5908254));
    }
}
