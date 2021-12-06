use crate::Answer;

pub fn solution(input: String) -> Answer<u64, u64> {
    let fish: Vec<u64> = input
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut init_counts: [u64; 9] = [0; 9];
    for f in fish {
        init_counts[f as usize] += 1;
    }

    Answer(
        (0..80)
            .fold(init_counts, |prev, _| breed(prev))
            .iter()
            .sum(),
        (0..256)
            .fold(init_counts, |prev, _| breed(prev))
            .iter()
            .sum(),
    )
}

fn breed(mut counts: [u64; 9]) -> [u64; 9] {
    counts.rotate_left(1);
    counts[6] += counts[8];
    counts
}
