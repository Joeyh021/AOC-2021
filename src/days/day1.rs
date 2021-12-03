use std::{fs, path::Path};

use crate::Answer;

pub fn solution<T: AsRef<Path>>(input: T) -> crate::Answer<u64, u64> {
    let file: Vec<u64> = fs::read_to_string(input)
        .expect("Could not open input file")
        .split_whitespace()
        .map(|a| {
            dbg!(a);
            a.parse::<u64>().unwrap()
        })
        .collect();

    let v = (file).iter();
    let v_1 = (file).iter().skip(1);

    let mut count = 0;

    for (i, j) in v.zip(v_1) {
        if j > i {
            count += 1;
        }
    }

    let mut v1 = file.iter().skip(1);
    let mut v2 = file.iter().skip(2);

    let m = file
        .iter()
        .map(|a| a + v1.next().unwrap_or(&0) + v2.next().unwrap_or(&0))
        .collect::<Vec<_>>();

    let t = m.iter();
    let t_1 = m.iter().skip(1);

    let mut count2 = 0;
    for (i, j) in t.zip(t_1) {
        if j > i {
            count2 += 1;
        }
    }
    //part 2
    Answer(count, count2)
}
