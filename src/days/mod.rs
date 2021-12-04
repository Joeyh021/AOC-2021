mod day1;
mod day2;
mod day3;

pub fn solve_day(day: u32) {
    let answer: (String, String) = match day {
        1 => day1::solution("input/01.txt"),
        2 => day2::solution("input/02.txt"),
        3 => day3::solution("input/03.txt"),
        _ => panic!("Not done this day yet"),
    };
    println!("Day {} solution:", day);
    println!("Part 1: {}", answer.0);
    println!("Part 2: {}", answer.1);
}
