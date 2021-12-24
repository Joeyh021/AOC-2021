use crate::Answer;

pub fn solution(input: String) -> Answer<u64, u64> {
    let init_counts =
        input
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .fold([0; 9], |mut counts, f| {
                counts[f as usize] += 1;
                counts
            });

    Answer::untimed(
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
