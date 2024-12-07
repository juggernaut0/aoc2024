use aoc::parse_lines_with;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        solve(&input, false)
    }

    fn solve_2(&self, input: String) -> String {
        solve(&input, true)
    }
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let (test_str, nums_str) = line.split_once(": ").unwrap();
    let test = test_str.parse().unwrap();
    let nums = nums_str
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    (test, nums)
}

fn solve(input: &str, concat: bool) -> String {
    parse_lines_with(input, parse_line)
        .filter_map(|(test, nums)| {
            log::trace!("checking {test}: {nums:?}");
            if is_valid(test, &nums, concat) {
                log::info!("{test}: {nums:?}");
                Some(test)
            } else {
                None
            }
        })
        .sum::<u64>()
        .to_string()
}

fn is_valid(test: u64, nums: &[u64], concat: bool) -> bool {
    log::trace!("is_valid called with test: {test}, nums: {nums:?}");
    if nums.len() == 1 {
        return nums[0] == test;
    }
    if nums[0] > test {
        return false;
    }
    let last = *nums.last().unwrap();
    if test % last == 0 && is_valid(test / last, &nums[0..nums.len() - 1], concat) {
        return true;
    }
    if test > last && is_valid(test - last, &nums[0..nums.len() - 1], concat) {
        return true;
    }
    if concat {
        let test_string = test.to_string();
        let last_str = last.to_string();
        if test_string.len() > last_str.len() && test_string.to_string().ends_with(&last_str) {
            let new_test = test_string[..test_string.len() - last_str.len()]
                .parse()
                .unwrap();
            return is_valid(new_test, &nums[0..nums.len() - 1], concat);
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::{init_test_logging, Solution};

    #[test]
    fn part_1() {
        init_test_logging();

        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
        .to_string();
        let res = Solution.solve_1(input);
        assert_eq!(res, "3749");
    }

    #[test]
    fn part_2() {
        init_test_logging();

        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
        .to_string();
        let res = Solution.solve_2(input);
        assert_eq!(res, "11387");
    }
}
