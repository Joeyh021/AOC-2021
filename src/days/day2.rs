use regex::Regex;
use std::{fs, path::Path};
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn solution<T: AsRef<Path>>(input: T) -> (String, String) {
    let re = Regex::new(r"(forward |up |down )(\d+)").expect("regex not valid");

    let file = fs::read_to_string(input).expect("Could not open input file");

    let data: Vec<Direction> = re
        .captures_iter(&file)
        .map(|line| {
            let n: i32 = line[2].parse().expect("Regex fail");
            match &line[1] {
                "forward " => Direction::Forward(n),
                "down " => Direction::Down(n),
                "up " => Direction::Up(n),
                _ => panic!("Epic Regex Fail"),
            }
        })
        .collect();

    let part1: (i32, i32) = data.iter().fold((0, 0), |(pos, depth), item| match item {
        Direction::Down(n) => (pos, depth + n),
        Direction::Up(n) => (pos, depth - n),
        Direction::Forward(n) => (pos + n, depth),
    });

    let part2: (i32, i32, i32) =
        data.iter()
            .fold((0, 0, 0), |(pos, depth, aim), item| match item {
                Direction::Down(n) => (pos, depth, aim + n),
                Direction::Up(n) => (pos, depth, aim - n),
                Direction::Forward(n) => (pos + n, depth + aim * n, aim),
            });

    (
        (part1.0 * part1.1).to_string(),
        (part2.0 * part2.1).to_string(),
    )
}
