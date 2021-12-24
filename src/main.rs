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
    fn parsed(&mut self) {
        self.parsed = Some(Instant::now())
    }

    fn part1(&mut self, p1: S) {
        self.part1 = Some(p1);
        self.time1 = Some(Instant::now());
    }
    fn part2(&mut self, p2: T) {
        self.part2 = Some(p2);
        self.time2 = Some(Instant::now());
    }

    fn untimed(p1: S, p2: T) -> Self {
        Answer {
            part1: Some(p1),
            part2: Some(p2),
            ..Self::start()
        }
    }
}

impl<S: Display, T: Display> Display for Answer<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(p) = self.parsed {
            writeln!(f, "Parsing Duration: {:?}", p - self.start)?;
        };
        if let Some(a1) = &self.part1 {
            writeln!(f, "Part 1 Answer: {}", a1)?;
        };
        if let Some(t1) = self.time1 {
            writeln!(f, "Part 1 Duration: {:?}", t1 - self.parsed.unwrap())?;
        };
        if let Some(a2) = &self.part2 {
            writeln!(f, "Part 2 Answer: {}", a2)?;
        };
        if let Some(t2) = self.time2 {
            writeln!(f, "Part 2 Duration: {:?}", t2 - self.time1.unwrap())?;
        };
        Ok(())
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
