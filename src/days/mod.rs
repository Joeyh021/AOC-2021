mod day1;
mod day2;

pub fn solve_day(day: u32) {
    let answer = match day {
        1 => day1::solution("input/01"),
        _ => panic!("Not done this day yet"),
    };
    println!("Day {} solution", day);
    println!("Part 1 solution: {}", answer.0);
    println!("Part 2 solution: {}", answer.1);
}
