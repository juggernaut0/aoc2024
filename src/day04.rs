use aoc::{Grid, Point};

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let ws: Grid<char> = input.parse().unwrap();
        let mut count = 0;
        for y in 0..ws.height() {
            for x in 0..ws.width() {
                for dir in Direction::all() {
                    if is_word("XMAS", &ws, Point(x, y), dir) {
                        count += 1;
                    }
                }
            }
        }
        count.to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let ws: Grid<char> = input.parse().unwrap();
        let mut count = 0;
        for y in 1..ws.height() {
            for x in 1..ws.width() {
                if is_cross_mas(&ws, Point(x, y)) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }
}

#[derive(Copy, Clone)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn diff(self) -> Point {
        match self {
            Direction::N => Point(0, -1),
            Direction::NE => Point(1, -1),
            Direction::E => Point(1, 0),
            Direction::SE => Point(1, 1),
            Direction::S => Point(0, 1),
            Direction::SW => Point(-1, 1),
            Direction::W => Point(-1, 0),
            Direction::NW => Point(-1, -1),
        }
    }

    fn all() -> impl Iterator<Item = Direction> {
        #[allow(clippy::enum_glob_use)]
        use Direction::*;
        [N, NE, E, SE, S, SW, W, NW].iter().copied()
    }
}

fn is_word(word: &str, grid: &Grid<char>, start: Point, dir: Direction) -> bool {
    if word.is_empty() {
        return true;
    }
    if grid.get(start).copied().unwrap_or(' ') != word.chars().next().unwrap() {
        return false;
    }
    is_word(&word[1..], grid, start + dir.diff(), dir)
}

fn is_cross_mas(grid: &Grid<char>, start: Point) -> bool {
    if grid.get(start).copied() != Some('A') {
        return false;
    }
    let is_mas = |p1: Point, p2: Point| -> bool {
        grid.get(p1).copied() == Some('M') && grid.get(p2).copied() == Some('S')
            || grid.get(p1).copied() == Some('S') && grid.get(p2).copied() == Some('M')
    };
    let mas1 = is_mas(start + Direction::NE.diff(), start + Direction::SW.diff());
    let mas2 = is_mas(start + Direction::NW.diff(), start + Direction::SE.diff());
    mas1 && mas2
}
