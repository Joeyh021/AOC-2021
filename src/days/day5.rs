use std::vec;

use crate::Answer;
use regex::Regex;

fn gen_points_straight((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> Vec<(u32, u32)> {
    if x1 == x2 && y1 == y2 {
        vec![(x1, y1)]
    }
    //line lies in y axis ->
    else if x1 == x2 && y2 > y1 {
        (y1..=y2).map(|y| (x1, y)).collect()
    }
    //line lies in y axis <-
    else if x1 == x2 && y1 > y2 {
        (y2..=y1).map(|y| (x1, y)).collect()
    }
    //line lies in x axis up
    else if y1 == y2 && x2 > x1 {
        (x1..=x2).map(|x| (x, y1)).collect()
    }
    //line lies in y axis down
    else if y1 == y2 && x1 > x2 {
        (x2..=x1).map(|x| (x, y1)).collect()
    } else {
        vec![]
    }
}

fn gen_points_all((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> Vec<(u32, u32)> {
    if x1 > x2 && y1 > y2 {
        (x2..=x1).zip(y2..=y1).collect()
    } else if x2 > x1 && y2 > y1 {
        (x1..=x2).zip(y1..=y2).collect()
    } else if x1 > x2 && y2 > y1 {
        (x2..=x1).zip((y1..=y2).rev()).collect()
    } else if x2 > x1 && y1 > y2 {
        (x1..=x2).rev().zip(y2..=y1).collect()
    } else {
        gen_points_straight((x1, y1), (x2, y2))
    }
}

pub fn solution(input: String) -> Answer<usize, usize> {
    let re = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)").expect("could not compile regex");
    let mut space1 = vec![vec![0u32; 1000]; 1000];
    let mut space2 = vec![vec![0u32; 1000]; 1000];
    for cap in re.captures_iter(&input) {
        let start: (u32, u32) = (
            cap[1].parse().expect("regex fail"),
            cap[2].parse().expect("regex fail"),
        );
        let end: (u32, u32) = (
            cap[3].parse().expect("regex fail"),
            cap[4].parse().expect("regex fail"),
        );

        for (x, y) in gen_points_straight(start, end) {
            space1[x as usize][y as usize] += 1;
        }
        for (x, y) in gen_points_all(start, end) {
            space2[x as usize][y as usize] += 1;
        }
    }
    let part1 = space1.concat().into_iter().filter(|i| *i > 1).count();
    let part2 = space2.concat().into_iter().filter(|i| *i > 1).count();
    //have list of every point
    Answer::untimed(part1, part2)
}
