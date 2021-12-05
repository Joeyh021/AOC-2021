use crate::Answer;
use regex::Regex;
use std::{thread, time};
fn get_eq((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (f32, f32) {
    let m = (y2 - y1) as f32 / (x2 - x1) as f32;
    let c = (y1 as f32) - m * (x1 as f32);
    (m, c)
}

fn gen_points((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> Vec<(u32, u32)> {
    //line lies in y axis ->
    if x1 == x2 && y2 > y1 {
        (y1..=y2).map(|y| (x1, y)).collect()
    }
    //line lies in y axis <-
    else if x1 == x2 && y1 >= y2 {
        (y2..=y1).map(|y| (x1, y)).collect()
    }
    //line lies in x axis up
    else if y1 == y2 && x2 > x1 {
        (x1..=x2).map(|x| (x, y1)).collect()
    }
    //line lies in y axis down
    else if y1 == y2 && x1 >= x2 {
        (x2..=x1).map(|x| (x, y1)).collect()
    }
    //diagonals
    else {
        vec![]
    }
}

pub fn solution(input: String) -> Answer<u32, u32> {
    let re = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)").expect("could not compile regex");
    let mut space = vec![vec![0u32; 1000]; 1000];
    for cap in re.captures_iter(&input) {
        let start: (u32, u32) = (
            cap[1].parse().expect("regex fail"),
            cap[2].parse().expect("regex fail"),
        );
        let end: (u32, u32) = (
            cap[3].parse().expect("regex fail"),
            cap[4].parse().expect("regex fail"),
        );

        for (x, y) in gen_points(start, end) {
            space[x as usize][y as usize] += 1;
        }
    }
    let mut sum = 0;
    for i in space.concat() {
        if i > 1 {
            sum += 1;
        }
    }
    //have list of every point
    Answer(sum, 1u32)
}
