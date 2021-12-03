use std::env::args;
mod days;

fn main() {
    let day: u32 = args().collect::<Vec<_>>()[1]
        .parse()
        .expect("Please enter a day to solve");
    days::solve_day(day);
}
