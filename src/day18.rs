use aoc::{parse_lines_with, Grid, Point};

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let points = parse_lines_with(&input, parse_point);
        let mut grid = Grid::from_elem(71, 71, false);
        for p in points.take(1024) {
            grid[p] = true;
        }
        bfs(&grid).unwrap().to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let points: Vec<_> = parse_lines_with(&input, parse_point).collect();
        let mut hi = points.len();
        let mut lo = 0;
        while hi != lo {
            let mid = (hi + lo) / 2;
            let mut grid = Grid::from_elem(71, 71, false);
            for &p in points.iter().take(mid) {
                grid[p] = true;
            }
            if bfs(&grid).is_none() {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        let p = points[lo - 1];
        format!("{},{}", p.0, p.1)
    }
}

fn parse_point(s: &str) -> Point {
    let (x, y) = s.split_once(',').unwrap();
    Point(x.parse().unwrap(), y.parse().unwrap())
}

fn bfs(grid: &Grid<bool>) -> Option<i32> {
    let mut q = std::collections::VecDeque::new();
    let mut seen = std::collections::HashSet::new();
    q.push_back((Point::zero(), 0));
    while let Some((p, d)) = q.pop_front() {
        if p == Point(grid.width() - 1, grid.height() - 1) {
            return Some(d);
        }
        for adj in p.adj() {
            if grid.contains_point(adj) && !grid[adj] && seen.insert(adj) {
                q.push_back((adj, d + 1));
            }
        }
    }
    None
}
