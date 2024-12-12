use aoc::{Dir, DirSet, Grid, Point};
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let map: Grid<char> = input.parse().unwrap();
        let regions = find_regions(&map);
        regions
            .iter()
            .map(|r| r.len() * perimeter(r))
            .sum::<usize>()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let map: Grid<char> = input.parse().unwrap();
        let regions = find_regions(&map);
        regions
            .iter()
            .map(|r| r.len() * sides(r))
            .sum::<usize>()
            .to_string()
    }
}

fn find_regions(map: &Grid<char>) -> Vec<HashSet<Point>> {
    let mut visited = Grid::from_elem(map.width(), map.height(), false);
    let mut regions = Vec::new();
    for (p, &c) in map.points_with_item() {
        if visited[p] {
            continue;
        }
        let mut region = HashSet::new();
        let mut stack = vec![p];
        while let Some(p) = stack.pop() {
            if visited[p] {
                continue;
            }
            visited[p] = true;
            region.insert(p);
            for n in p.adj() {
                if map.contains_point(n) && map[n] == c {
                    stack.push(n);
                }
            }
        }
        regions.push(region);
    }
    regions
}

fn perimeter(region: &HashSet<Point>) -> usize {
    region
        .iter()
        .map(|&p| p.adj().iter().filter(|&n| !region.contains(n)).count())
        .sum()
}

fn sides(region: &HashSet<Point>) -> usize {
    let mut sides = 0;
    let mut border_map = HashMap::new();
    for &p in region {
        let mut borders = DirSet::new();
        for dir in Dir::all() {
            let n = p + dir.diff();
            if !region.contains(&n) {
                borders.insert(dir);
            }
        }
        if !borders.is_empty() {
            border_map.insert(p, borders);
        }
    }
    for d in Dir::all() {
        let border_points: HashSet<_> = border_map
            .iter()
            .filter(|(_, b)| b.contains(d))
            .map(|(p, _)| *p)
            .collect();
        let mut visited = HashSet::new();
        for &p in &border_points {
            if !visited.insert(p) {
                continue;
            }
            sides += 1;
            let left = d.turn_left();
            for i in 1.. {
                let n = p + left.diff() * i;
                if border_points.contains(&n) {
                    visited.insert(n);
                } else {
                    break;
                }
            }
            let right = d.turn_right();
            for i in 1.. {
                let n = p + right.diff() * i;
                if border_points.contains(&n) {
                    visited.insert(n);
                } else {
                    break;
                }
            }
        }
    }
    sides
}
