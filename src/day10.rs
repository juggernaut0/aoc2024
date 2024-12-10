use aoc::{Grid, Point};
use std::collections::HashSet;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        solve(&input, false)
    }

    fn solve_2(&self, input: String) -> String {
        solve(&input, true)
    }
}

struct Height(u32);

impl From<char> for Height {
    fn from(c: char) -> Self {
        Height(c.to_digit(10).unwrap())
    }
}

fn solve(input: &str, part2: bool) -> String {
    let map: Grid<Height> = input.parse().unwrap();
    let mut total_score = 0;
    for (p, h) in map.points_with_item() {
        if h.0 == 0 {
            let mut visited = if part2 { None } else { Some(HashSet::new()) };
            total_score += get_scorating(&map, p, &mut visited);
        }
    }
    total_score.to_string()
}

fn get_scorating(map: &Grid<Height>, start: Point, visited: &mut Option<HashSet<Point>>) -> u32 {
    if let Some(visited) = visited.as_mut() {
        if !visited.insert(start) {
            return 0;
        }
    }

    let my_height = map[start].0;
    if my_height == 9 {
        return 1;
    }

    let mut scorating = 0;
    for a in start.adj() {
        if map.get(a).map_or(0, |it| it.0) == my_height + 1 {
            scorating += get_scorating(map, a, visited);
        }
    }
    scorating
}
