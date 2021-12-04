use crate::Answer;
use regex::Regex;
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn solution(input: String) -> Answer<i32, i32> {
    let re = Regex::new(r"(forward |up |down )(\d+)").expect("regex not valid");

    let data: Vec<Direction> = re
        .captures_iter(&input)
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

    Answer(part1.0 * part1.1, part2.0 * part2.1)
}
