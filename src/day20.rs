use aoc::{Counter, Grid, Point};
use std::collections::VecDeque;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        solve(&input, 2)
    }

    fn solve_2(&self, input: String) -> String {
        solve(&input, 20)
    }
}

fn solve(input: &str, cheat_time: i32) -> String {
    let map = input.parse().unwrap();
    let race = Race { map, cheat_time };
    log::debug!("finding base dists");
    let base_dists = find_base_dists(&race);
    log::debug!("finding cheats");
    let cheats = find_cheat_dists(&race, &base_dists);
    cheats
        .into_iter()
        .filter_map(|(save, count)| if save >= 100 { Some(count) } else { None })
        .sum::<u64>()
        .to_string()
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => unreachable!(),
        }
    }
}

struct Race {
    map: Grid<Tile>,
    cheat_time: i32,
}

struct RaceState {
    pos: Point,
    dist: i32,
}

fn find_base_dists(race: &Race) -> Grid<i32> {
    let mut res = Grid::from_elem(race.map.width(), race.map.height(), i32::MAX);
    let mut q = VecDeque::new();
    let start = race
        .map
        .points_with_item()
        .find_map(|(p, &tile)| if tile == Tile::Start { Some(p) } else { None })
        .unwrap();
    q.push_back(RaceState {
        pos: start,
        dist: 0,
    });
    while let Some(state) = q.pop_front() {
        if res[state.pos] <= state.dist {
            continue;
        }
        res[state.pos] = state.dist;
        for a in state.pos.adj() {
            if race.map[a] == Tile::Wall {
                continue;
            }
            let succ = RaceState {
                pos: a,
                dist: state.dist + 1,
            };
            q.push_back(succ);
        }
    }
    res
}

fn spaces_within_distance(
    start: Point,
    d: i32,
    map: &Grid<Tile>,
) -> impl Iterator<Item = (Point, i32)> + '_ {
    (0..=d)
        .flat_map(move |i| {
            (0..=d - i).flat_map(move |j| {
                Some((start + Point(i, j), i + j))
                    .into_iter()
                    .chain(Some((start + Point(i, -j), i + j)).filter(|_| j > 0))
                    .chain(Some((start + Point(-i, j), i + j)).filter(|_| i > 0))
                    .chain(Some((start + Point(-i, -j), i + j)).filter(|_| i > 0 && j > 0))
            })
        })
        .filter(|(p, _)| matches!(map.get(*p), Some(t) if *t != Tile::Wall))
}

fn find_cheat_dists(race: &Race, base_dists: &Grid<i32>) -> Counter<i32> {
    race.map
        .points_with_item()
        .filter_map(|(p, &t)| if t == Tile::Wall { None } else { Some(p) })
        .flat_map(|start| {
            let base_dist = base_dists[start];
            spaces_within_distance(start, race.cheat_time, &race.map).filter_map(move |(end, d)| {
                let old_dist = base_dists[end];
                let new_dist = base_dist + d;
                if new_dist < old_dist {
                    Some(old_dist - new_dist)
                } else {
                    None
                }
            })
        })
        .collect()
}
