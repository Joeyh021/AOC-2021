use crate::Answer;

pub fn solution(input: String) -> Answer<u64, u64> {
    let mut ans = Answer::start();

    let file: Vec<u64> = input
        .split_whitespace()
        .map(|a| a.parse::<u64>().unwrap())
        .collect();

    ans.parsed();

    let v = (file).iter();
    let v_1 = (file).iter().skip(1);

    let mut part1 = 0;

    for (i, j) in v.zip(v_1) {
        if j > i {
            part1 += 1;
        }
    }
    ans.part1(part1);

    let mut v1 = file.iter().skip(1);
    let mut v2 = file.iter().skip(2);

    let m = file
        .iter()
        .map(|a| a + v1.next().unwrap_or(&0) + v2.next().unwrap_or(&0))
        .collect::<Vec<_>>();

    let t = m.iter();
    let t_1 = m.iter().skip(1);

    let mut part2 = 0;
    for (i, j) in t.zip(t_1) {
        if j > i {
            part2 += 1;
        }
    }

    ans.part2(part2);

    //part 2
    ans
}
