use crate::Answer;

fn median(arr: &mut [i64]) -> i64 {
    arr.sort_unstable();
    if arr.len() % 2 == 0 {
        arr[arr.len() / 2]
    } else {
        (arr[arr.len() / 2 - 1] + arr[arr.len() / 2]) / 2
    }
}

pub fn solution(input: String) -> Answer<i64, i64> {
    let mut crabs = input
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let med: i64 = median(&mut crabs);
    let mean: i64 = crabs.iter().sum::<i64>() / crabs.len() as i64;
    let part1: i64 = crabs.iter().map(|pos| i64::abs(pos - med)).sum();
    let part2: i64 = crabs
        .iter()
        .map(|pos| {
            let n = i64::abs(pos - mean);
            (n * (n + 1)) / 2
        })
        .sum();
    Answer(part1, part2)
}

#[test]
fn test_1() {
    let input = String::from("16,1,2,0,4,2,7,1,2,14");
    assert_eq!(solution(input).0, 37);
}
#[test]
fn test_2() {
    let input = String::from("16,1,2,0,4,2,7,1,2,14");
    assert_eq!(solution(input).1, 168);
}
