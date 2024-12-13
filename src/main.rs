#![warn(clippy::pedantic)]

use aoc::Solution;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

const SOLUTIONS: [&dyn Solution; 25] = [
    &day01::Solution,
    &day02::Solution,
    &day03::Solution,
    &day04::Solution,
    &day05::Solution,
    &day06::Solution,
    &day07::Solution,
    &day08::Solution,
    &day09::Solution,
    &day10::Solution,
    &day11::Solution,
    &day12::Solution,
    &day13::Solution,
    &day14::Solution,
    &day15::Solution,
    &day16::Solution,
    &day17::Solution,
    &day18::Solution,
    &day19::Solution,
    &day20::Solution,
    &day21::Solution,
    &day22::Solution,
    &day23::Solution,
    &day24::Solution,
    &day25::Solution,
];

fn main() {
    aoc::run("2024", SOLUTIONS);
}

#[cfg(test)]
aoc::generate_answer_tests!(SOLUTIONS);
