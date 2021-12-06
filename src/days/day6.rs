use crate::Answer;

pub fn solution(input: String) -> Answer<usize, usize> {
    let mut day0: Vec<u64> = input
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    for _ in 0..256 {
        breed(&mut day0);
    }
    Answer(day0.len(), 0usize)
}

fn breed(fish: &mut Vec<u64>) {
    let mut new = 0;
    for f in fish.iter_mut() {
        if *f == 0u64 {
            *f = 6u64;
            new += 1;
        } else {
            *f -= 1u64;
        }
    }
    fish.append(&mut vec![8; new]);
}
