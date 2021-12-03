mod day1;
mod day2;

pub fn solve_day(day: u32) {
    let answer: (String, String) = match day {
        1 => day1::solution("input/01"),
        2 => day2::solution("input/02"),
        _ => panic!("Not done this day yet"),
    };
    println!("Day {} solution:", day);
    println!("Part 1: {}", answer.0);
    println!("Part 2: {}", answer.1);
}
