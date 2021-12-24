use crate::Solution;
use std::{fmt::Display, io};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn solve_day(day: u32) -> Result<(), io::Error> {
    match day {
        1 => solve(day1::solution, std::fs::read_to_string("input/01.txt")?),
        2 => solve(day2::solution, std::fs::read_to_string("input/02.txt")?),
        3 => solve(day3::solution, std::fs::read_to_string("input/03.txt")?),
        4 => solve(day4::solution, std::fs::read_to_string("input/04.txt")?),
        5 => solve(day5::solution, std::fs::read_to_string("input/05.txt")?),
        6 => solve(day6::solution, std::fs::read_to_string("input/06.txt")?),
        7 => solve(day7::solution, std::fs::read_to_string("input/07.txt")?),
        8 => solve(day8::solution, std::fs::read_to_string("input/08.txt")?),
        9 => solve(day9::solution, std::fs::read_to_string("input/09.txt")?),
        _ => panic!("Day not done yet"),
    }
    Ok(())
}

fn solve<S: Display, T: Display>(solution: Solution<S, T>, input: String) {
    let sln = solution(input);
    println!("Part 1: {}", sln.0);
    println!("Part 2: {}", sln.1);
}
