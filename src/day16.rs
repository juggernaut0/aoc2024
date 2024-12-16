use aoc::{search, Dir, Grid, Point, Searchable};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let maze = Maze {
            map: input.parse().unwrap(),
        };
        let score = -search(&maze).unwrap().1;
        score.to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let maze = Maze {
            map: input.parse().unwrap(),
        };
        let points: HashSet<_> = search_multi(&maze)
            .into_iter()
            .flat_map(|s| s.path.into_iter().map(|(p, _)| p))
            .collect();
        points.len().to_string()
    }
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

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Start => 'S',
            Tile::End => 'E',
        };
        write!(f, "{c}")
    }
}

struct Maze {
    map: Grid<Tile>,
}

#[derive(Debug)]
struct MazeState {
    pos: Point,
    dir: Dir,
    score: i32,
    path: Vec<(Point, i32)>,
}

impl Searchable for Maze {
    type State = MazeState;
    type Key = (Point, Dir);
    type Value = i32;

    fn initial_state(&self) -> Self::State {
        let start = self
            .map
            .points_with_item()
            .find_map(|(p, &t)| if t == Tile::Start { Some(p) } else { None })
            .unwrap();
        MazeState {
            pos: start,
            dir: Dir::E,
            score: 0,
            path: vec![(start, 0)],
        }
    }

    fn successors(&self, state: Self::State) -> Vec<Self::State> {
        let mut res = Vec::new();
        res.push(MazeState {
            pos: state.pos,
            dir: state.dir.turn_left(),
            score: state.score + 1000,
            path: state.path.clone(),
        });
        res.push(MazeState {
            pos: state.pos,
            dir: state.dir.turn_right(),
            score: state.score + 1000,
            path: state.path.clone(),
        });
        let forward = state.pos + state.dir.diff();
        if self.map[forward] != Tile::Wall {
            let mut path = state.path.clone();
            path.push((forward, state.score + 1));
            res.push(MazeState {
                pos: forward,
                dir: state.dir,
                score: state.score + 1,
                path,
            });
        }
        res
    }

    fn key(&self, state: &Self::State) -> Self::Key {
        (state.pos, state.dir)
    }

    fn value(&self, state: &Self::State) -> Self::Value {
        -state.score
    }

    fn value_estimate(&self, state: &Self::State) -> Self::Value {
        -state.score
    }

    fn is_goal(&self, state: &Self::State) -> bool {
        self.map[state.pos] == Tile::End
    }
}

fn search_multi(maze: &Maze) -> Vec<MazeState> {
    let mut res: Vec<MazeState> = Vec::new();
    let mut winning_scores: HashMap<Point, i32> = HashMap::new();
    let mut visited: HashMap<(Point, Dir), i32> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(maze.initial_state());
    while let Some(state) = queue.pop_front() {
        if winning_scores.get(&state.pos) == Some(&state.score) {
            for (p, s) in &state.path {
                winning_scores.insert(*p, *s);
            }
            res.push(state);
            continue;
        }

        if maze.is_goal(&state) {
            let best = res.first().map_or(i32::MAX, |s| s.score);
            if state.score < best {
                winning_scores.clear();
                for (p, s) in &state.path {
                    winning_scores.insert(*p, *s);
                }
                res.clear();
                res.push(state);
            }
            continue;
        }

        for succ in maze.successors(state) {
            let key = maze.key(&succ);
            let value = maze.value(&succ);
            if let Some(&old_value) = visited.get(&key) {
                if old_value > value {
                    continue;
                }
            }
            visited.insert(key, value);
            queue.push_back(succ);
        }
    }
    res
}
