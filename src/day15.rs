use aoc::{Dir, Grid, Point};
use std::fmt::Debug;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (mut world, moves) = parse_input(&input);
        for dir in moves {
            world.move_robot(dir);
        }
        world.gps_score().to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let wide_input = input
            .replace('#', "##")
            .replace('O', "[]")
            .replace('.', "..")
            .replace('@', "@.");
        let (mut world, moves) = parse_input(&wide_input);
        for dir in moves {
            #[cfg(not(test))]
            if log::max_level() >= log::Level::Debug {
                let mut debug_map = world.map.clone();
                debug_map[world.robot] = Tile::Start;
                log::debug!("dir: {dir:?}\n{:?}", debug_map);
                std::io::stdin().read_line(&mut String::new()).unwrap();
            }
            world.move_robot(dir);
        }
        world.gps_score().to_string()
    }
}

struct World {
    map: Grid<Tile>,
    robot: Point,
}

fn parse_input(input: &str) -> (World, Vec<Dir>) {
    let (grid_str, moves_str) = input.trim().split_once("\n\n").unwrap();
    let mut grid: Grid<Tile> = grid_str.parse().unwrap();
    let moves = moves_str
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => Dir::N,
            'v' => Dir::S,
            '<' => Dir::W,
            '>' => Dir::E,
            _ => panic!("Invalid move: {c}"),
        })
        .collect();
    let robot = grid
        .points_with_item()
        .find_map(|(p, &t)| if t == Tile::Start { Some(p) } else { None })
        .unwrap();
    grid[robot] = Tile::Empty;
    let world = World { map: grid, robot };
    (world, moves)
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Start,
    Crate,
    BigCrateLeft,
    BigCrateRight,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            '@' => Tile::Start,
            'O' => Tile::Crate,
            '[' => Tile::BigCrateLeft,
            ']' => Tile::BigCrateRight,
            _ => panic!("Invalid tile: {value}"),
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Start => '@',
            Tile::Crate => 'O',
            Tile::BigCrateLeft => '[',
            Tile::BigCrateRight => ']',
        };
        write!(f, "{c}")
    }
}

impl World {
    fn move_robot(&mut self, dir: Dir) {
        let next = self.robot + dir.diff();
        let next_tile = self.map[next];
        match next_tile {
            Tile::Empty => {
                self.robot = next;
            }
            Tile::Crate => {
                // this would be incorrect if crates and big crates were on the same map
                let mut last_next = next;
                while let Some(Tile::Crate) = self.map.get(last_next) {
                    last_next += dir.diff();
                }
                if let Some(Tile::Empty) = self.map.get(last_next) {
                    self.map[last_next] = Tile::Crate;
                    self.map[next] = Tile::Empty;
                    self.robot = next;
                }
            }
            Tile::BigCrateLeft => {
                if self.try_move_big_crate(next, dir) {
                    self.robot = next;
                }
            }
            Tile::BigCrateRight => {
                if self.try_move_big_crate(next + Dir::W.diff(), dir) {
                    self.robot = next;
                }
            }
            _ => {}
        }
    }

    fn can_move_big_crate(&self, pos: Point, dir: Dir) -> bool {
        if let Some(Tile::BigCrateLeft) = self.map.get(pos) {
            let right_pos = pos + Dir::E.diff();
            match dir {
                Dir::N | Dir::S => {
                    let next_left = pos + dir.diff();
                    let next_right = right_pos + dir.diff();
                    match (self.map[next_left], self.map[next_right]) {
                        (Tile::Empty, Tile::Empty) => true,
                        (Tile::BigCrateLeft, Tile::BigCrateRight) => {
                            self.can_move_big_crate(next_left, dir)
                        }
                        (Tile::BigCrateRight, Tile::Empty) => {
                            self.can_move_big_crate(next_left + Dir::W.diff(), dir)
                        }
                        (Tile::Empty, Tile::BigCrateLeft) => {
                            self.can_move_big_crate(next_right, dir)
                        }
                        (Tile::BigCrateRight, Tile::BigCrateLeft) => {
                            self.can_move_big_crate(next_left + Dir::W.diff(), dir)
                                && self.can_move_big_crate(next_right, dir)
                        }
                        _ => false,
                    }
                }
                Dir::E => {
                    let next = pos + Dir::E.diff() * 2;
                    match self.map[next] {
                        Tile::Empty => true,
                        Tile::BigCrateLeft => self.can_move_big_crate(next, dir),
                        _ => false,
                    }
                }
                Dir::W => match self.map[pos + Dir::W.diff()] {
                    Tile::Empty => true,
                    Tile::BigCrateRight => self.can_move_big_crate(pos + Dir::W.diff() * 2, dir),
                    _ => false,
                },
            }
        } else {
            panic!("called can_move_big_crate on non-BigCrateLeft tile");
        }
    }

    // returns whether the crate was moved
    fn try_move_big_crate(&mut self, pos: Point, dir: Dir) -> bool {
        if let Some(Tile::BigCrateLeft) = self.map.get(pos) {
            let right_pos = pos + Dir::E.diff();
            match dir {
                Dir::N | Dir::S => {
                    let next_left = pos + dir.diff();
                    let next_right = right_pos + dir.diff();
                    match (self.map[next_left], self.map[next_right]) {
                        (Tile::Empty, Tile::Empty) => {
                            self.move_big_crate(pos, dir);
                            true
                        }
                        (Tile::BigCrateLeft, Tile::BigCrateRight) => {
                            let could_move = self.try_move_big_crate(next_left, dir);
                            if could_move {
                                self.move_big_crate(pos, dir);
                            }
                            could_move
                        }
                        (Tile::BigCrateRight, Tile::Empty) => {
                            let could_move =
                                self.try_move_big_crate(next_left + Dir::W.diff(), dir);
                            if could_move {
                                self.move_big_crate(pos, dir);
                            }
                            could_move
                        }
                        (Tile::Empty, Tile::BigCrateLeft) => {
                            let could_move = self.try_move_big_crate(next_right, dir);
                            if could_move {
                                self.move_big_crate(pos, dir);
                            }
                            could_move
                        }
                        (Tile::BigCrateRight, Tile::BigCrateLeft) => {
                            let can_move_left =
                                self.can_move_big_crate(next_left + Dir::W.diff(), dir);
                            let can_move_right = self.can_move_big_crate(next_right, dir);
                            if can_move_left && can_move_right {
                                self.try_move_big_crate(next_left + Dir::W.diff(), dir);
                                self.try_move_big_crate(next_right, dir);
                                self.move_big_crate(pos, dir);
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    }
                }
                Dir::E => {
                    let next = pos + Dir::E.diff() * 2;
                    match self.map[next] {
                        Tile::Empty => {
                            self.move_big_crate(pos, dir);
                            true
                        }
                        Tile::BigCrateLeft => {
                            let could_move = self.try_move_big_crate(next, dir);
                            if could_move {
                                self.move_big_crate(pos, dir);
                            }
                            could_move
                        }
                        _ => false,
                    }
                }
                Dir::W => match self.map[pos + Dir::W.diff()] {
                    Tile::Empty => {
                        self.move_big_crate(pos, dir);
                        true
                    }
                    Tile::BigCrateRight => {
                        let could_move = self.try_move_big_crate(pos + Dir::W.diff() * 2, dir);
                        if could_move {
                            self.move_big_crate(pos, dir);
                        }
                        could_move
                    }
                    _ => false,
                },
            }
        } else {
            self.map[self.robot] = Tile::Start;
            log::error!("robot: {:?}\n{:?}", self.robot, self.map);
            panic!("called try_move_big_crate on non-BigCrateLeft tile");
        }
    }

    // only to be called from try_move_big_crate!!!!
    // does no checks!
    fn move_big_crate(&mut self, pos: Point, dir: Dir) {
        debug_assert!(self.map[pos] == Tile::BigCrateLeft);
        let right_pos = pos + Dir::E.diff();
        let next_left = pos + dir.diff();
        let next_right = right_pos + dir.diff();

        self.map[pos] = Tile::Empty;
        self.map[right_pos] = Tile::Empty;
        self.map[next_left] = Tile::BigCrateLeft;
        self.map[next_right] = Tile::BigCrateRight;
    }

    fn gps_score(&self) -> i32 {
        self.map
            .points_with_item()
            .map(|(p, &t)| match t {
                Tile::Crate | Tile::BigCrateLeft => p.0 + p.1 * 100,
                _ => 0,
            })
            .sum()
    }
}
