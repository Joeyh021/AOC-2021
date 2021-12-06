use crate::Answer;

pub fn solution(input: String) -> Answer<usize, usize> {
    let day0: Vec<u32> = input.split(',').map(|s| s.parse::<u64>().unwrap).collect();
    for i in 0..80 {
        breed(day0);
    }
}

fn breed(fish: &mut Vec<u64>) -> Vec<u64> {
    for f in fish {
        if f == 0 {
            f = 6;
            next.push(8);
        } else {
            f = f - 1;
        }
    }
}
