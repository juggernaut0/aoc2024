use aoc::{Dir, DirSet, Grid, Point};
use std::collections::HashSet;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let map: Grid<Tile> = input.parse().unwrap();
        simulate(&map).unwrap().len().to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let map: Grid<Tile> = input.parse().unwrap();
        let visited = simulate(&map).unwrap();
        let mut result = 0;
        for p in visited {
            if let Some(Tile::Start) = map.get(p) {
                continue;
            }
            let mut new_map = map.clone();
            new_map.set(p, Tile::Wall);
            if simulate(&new_map).is_none() {
                result += 1;
            }
        }
        result.to_string()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '^' => Tile::Start,
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            _ => panic!("Invalid tile: {value}"),
        }
    }
}

fn simulate(map: &Grid<Tile>) -> Option<HashSet<Point>> {
    let mut pos = map
        .points_with_item()
        .find_map(|(p, &t)| if t == Tile::Start { Some(p) } else { None })
        .unwrap();
    let mut dir = Dir::N;
    let mut visited_state = Grid::from_elem(map.width(), map.height(), DirSet::new());
    while map.contains_point(pos) {
        if !visited_state[pos].insert(dir) {
            return None;
        }
        let forward = pos + dir.diff();
        if let Some(Tile::Wall) = map.get(forward) {
            dir = dir.turn_right();
        } else {
            pos = forward;
        }
    }
    Some(
        visited_state
            .points_with_item()
            .filter_map(|(p, ds)| if ds.is_empty() { None } else { Some(p) })
            .collect(),
    )
}
