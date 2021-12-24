use std::env::args;
use std::fmt::Display;
use std::time::Instant;
mod days;

pub struct Answer<S: Display, T: Display> {
    part1: Option<S>,
    part2: Option<T>,
    start: Instant,
    parsed: Option<Instant>,
    time1: Option<Instant>,
    time2: Option<Instant>,
}
impl<S: Display, T: Display> Answer<S, T> {
    fn start() -> Self {
        Answer {
            part1: None,
            part2: None,
            start: Instant::now(),
            parsed: None,
            time1: None,
            time2: None,
        }
    }
    fn parsed(self) -> Self {
        Answer {
            parsed: Some(Instant::now()),
            ..self
        }
    }
    fn part1(self, p1: S) -> Self {
        Answer {
            part1: Some(p1),
            time1: Some(Instant::now()),
            ..self
        }
    }
    fn part2(self, p2: S) -> Self {
        Answer {
            part1: Some(p2),
            time2: Some(Instant::now()),
            ..self
        }
    }
    fn untimed(p1: S, p2: T) -> Self {
        Answer {
            part1: Some(p1),
            part2: Some(p2),
            ..Self::start()
        }
    }
}

type Solution<S, T> = fn(String) -> Answer<S, T>;

fn main() {
    let day: u32 = args().collect::<Vec<_>>()[1]
        .parse()
        .expect("Please enter a day to solve");
    println!("Solutions for day {}:", day);
    days::solve_day(day).expect("Could not open input file");
}
