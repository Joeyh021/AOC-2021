use std::env::args;
use std::fmt::Display;
mod days;
pub struct Answer<S: Display, T: Display>(S, T);

fn main() {
    let day: u32 = args().collect::<Vec<_>>()[1]
        .parse()
        .expect("Please enter a day to solve");
    days::solve_day(day);
}
