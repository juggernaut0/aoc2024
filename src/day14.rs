use aoc::{parse_lines, Point};
use std::str::FromStr;

pub struct Solution {
    pub width: i32,
    pub height: i32,
}

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let mut robots: Vec<Robot> = parse_lines(&input).collect();
        for robot in &mut robots {
            robot.move_robot(100, self.width, self.height);
        }
        let mut quadrants = [0i64; 4];
        log::info!("Robots: {robots:?}");
        for robot in robots {
            if let Some(q) = robot.quadrant(self.width, self.height) {
                quadrants[q] += 1;
            }
        }
        log::info!("Quadrants: {quadrants:?}");
        quadrants.iter().product::<i64>().to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let mut robots: Vec<Robot> = parse_lines(&input).collect();
        // I'm guessing that 2 and 103 are values specific to my input
        for robot in &mut robots {
            robot.move_robot(2, self.width, self.height);
        }
        for i in 0.. {
            println!("Second: {}", (i * 103) + 2);
            display_robots(&robots, self.width, self.height);
            for robot in &mut robots {
                robot.move_robot(103, self.width, self.height);
            }

            std::io::stdin().read_line(&mut String::new()).unwrap();
        }
        unreachable!();
    }
}

#[derive(Debug)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos_str, vel_str) = s.split_once(' ').ok_or(())?;
        let pos_parts = pos_str[2..].split_once(',').ok_or(())?;
        let pos = Point(
            pos_parts.0.parse().map_err(|_| ())?,
            pos_parts.1.parse().map_err(|_| ())?,
        );
        let vel_parts = vel_str[2..].split_once(',').ok_or(())?;
        let vel = Point(
            vel_parts.0.parse().map_err(|_| ())?,
            vel_parts.1.parse().map_err(|_| ())?,
        );
        Ok(Robot { pos, vel })
    }
}

impl Robot {
    fn move_robot(&mut self, seconds: i32, width: i32, height: i32) {
        self.pos += self.vel * seconds;
        self.pos.0 %= width;
        if self.pos.0 < 0 {
            self.pos.0 += width;
        }
        self.pos.1 %= height;
        if self.pos.1 < 0 {
            self.pos.1 += height;
        }
    }

    fn quadrant(&self, width: i32, height: i32) -> Option<usize> {
        let left = self.pos.0 < width / 2;
        let right = self.pos.0 > width / 2;
        let top = self.pos.1 < height / 2;
        let bottom = self.pos.1 > height / 2;

        if left {
            if top {
                Some(0)
            } else if bottom {
                Some(1)
            } else {
                None
            }
        } else if right {
            if top {
                Some(2)
            } else if bottom {
                Some(3)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[allow(clippy::cast_sign_loss)]
fn display_robots(robots: &[Robot], width: i32, height: i32) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    for robot in robots {
        grid[robot.pos.1 as usize][robot.pos.0 as usize] = '#';
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use aoc::{init_test_logging, Solution};

    #[test]
    fn example() {
        init_test_logging();

        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
            .to_string();

        let res = super::Solution {
            width: 11,
            height: 7,
        }
        .solve_1(input);
        assert_eq!(res, "12");
    }
}
