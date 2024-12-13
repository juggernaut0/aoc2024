use aoc::Point;
use std::str::FromStr;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        input
            .split("\n\n")
            .map(|s| s.parse::<Machine>().unwrap())
            .filter_map(|machine| machine.solve().map(|point| point.0 * 3 + point.1))
            .sum::<i64>()
            .to_string()
    }

    #[allow(clippy::unreadable_literal)]
    fn solve_2(&self, input: String) -> String {
        input
            .split("\n\n")
            .map(|s| {
                let mut machine = s.parse::<Machine>().unwrap();
                machine.prize += Point(10000000000000, 10000000000000);
                machine
            })
            .filter_map(|machine| machine.solve().map(|point| point.0 * 3 + point.1))
            .sum::<i64>()
            .to_string()
    }
}

#[derive(Debug)]
struct Machine {
    a: Point<i64>,
    b: Point<i64>,
    prize: Point<i64>,
}

impl FromStr for Machine {
    type Err = String;

    #[allow(clippy::similar_names)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let a_str = &lines.next().ok_or("Missing A")?[10..];
        let (a_x_str, a_y_str) = a_str.split_once(", ").ok_or("Invalid A")?;
        let a = Point(
            a_x_str[2..]
                .parse()
                .map_err(|_| format!("Invalid A X: {s}"))?,
            a_y_str[2..]
                .parse()
                .map_err(|_| format!("Invalid A Y: {s}"))?,
        );

        let b_str = &lines.next().ok_or("Missing B")?[10..];
        let (b_x_str, b_y_str) = b_str.split_once(", ").ok_or("Invalid B")?;
        let b = Point(
            b_x_str[2..]
                .parse()
                .map_err(|_| format!("Invalid B X: {s}"))?,
            b_y_str[2..]
                .parse()
                .map_err(|_| format!("Invalid B Y: {s}"))?,
        );

        let prize_str = &lines.next().ok_or("Missing Prize")?[7..];
        let (prize_x_str, prize_y_str) = prize_str.split_once(", ").ok_or("Invalid Prize")?;
        let prize = Point(
            prize_x_str[2..]
                .parse()
                .map_err(|_| format!("Invalid Prize X: {s}"))?,
            prize_y_str[2..]
                .parse()
                .map_err(|_| format!("Invalid Prize Y: {s}"))?,
        );

        Ok(Machine { a, b, prize })
    }
}

impl Machine {
    fn solve(&self) -> Option<Point<i64>> {
        let matrix = [[self.a.0, self.b.0], [self.a.1, self.b.1]];
        let inv_matrix = invert(matrix);
        log::debug!("{:?}", inv_matrix);
        let [an, bn] = multiply(inv_matrix, [self.prize.0, self.prize.1])?;
        if an < 0 || bn < 0 {
            None
        } else {
            Some(Point(an, bn))
        }
    }
}

fn invert(matrix: [[i64; 2]; 2]) -> ([[i64; 2]; 2], i64) {
    let det = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    log::debug!("det: {}", det);
    (
        [[matrix[1][1], -matrix[0][1]], [-matrix[1][0], matrix[0][0]]],
        det,
    )
}

fn multiply((matrix, det): ([[i64; 2]; 2], i64), vector: [i64; 2]) -> Option<[i64; 2]> {
    let a = matrix[0][0] * vector[0] + matrix[0][1] * vector[1];
    if a % det != 0 {
        return None;
    }
    let b = matrix[1][0] * vector[0] + matrix[1][1] * vector[1];
    if b % det != 0 {
        return None;
    }
    Some([a / det, b / det])
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::init_test_logging;

    #[test]
    fn example() {
        init_test_logging();

        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";

        let machine: Machine = input.parse().unwrap();
        assert_eq!(machine.solve(), Some(Point(80, 40)));

        let input = "Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";
        let machine: Machine = input.parse().unwrap();
        assert_eq!(machine.solve(), None);
    }
}
