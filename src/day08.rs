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

enum Tile {
    Empty,
    Antenna(char),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            c => Tile::Antenna(c),
        }
    }
}

fn solve(input: &str, part2: bool) -> String {
    let map: Grid<Tile> = input.parse().unwrap();
    let unique_freqs = unique_freqs(&map);
    let mut antinodes = HashSet::new();
    for freq in unique_freqs {
        let antennae = find_antennae(&map, freq);
        for i in 0..antennae.len() {
            for j in i + 1..antennae.len() {
                for antinode in calculate_antinodes(antennae[i], antennae[j], &map, part2) {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len().to_string()
}

fn unique_freqs(map: &Grid<Tile>) -> HashSet<char> {
    let mut unique_freqs = HashSet::new();
    for (_, t) in map.points_with_item() {
        if let Tile::Antenna(c) = t {
            unique_freqs.insert(*c);
        }
    }
    unique_freqs
}

fn find_antennae(map: &Grid<Tile>, freq: char) -> Vec<Point> {
    map.points_with_item()
        .filter_map(|(p, t)| {
            if let Tile::Antenna(c) = t {
                if *c == freq {
                    Some(p)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn calculate_antinodes(p1: Point, p2: Point, map: &Grid<Tile>, part2: bool) -> Vec<Point> {
    let Point(dx, dy) = p2 - p1;
    let mut res = Vec::new();
    if part2 {
        res.push(p1);
        res.push(p2);
    }
    for i in 1.. {
        let a1 = p2 + Point(dx, dy) * i;
        let a2 = p1 + Point(-dx, -dy) * i;
        if map.contains_point(a1) {
            res.push(a1);
        }
        if map.contains_point(a2) {
            res.push(a2);
        }
        if !part2 || (!map.contains_point(a1) && !map.contains_point(a2)) {
            break;
        }
    }
    res
}
