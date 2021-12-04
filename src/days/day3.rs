use core::panic;

use crate::Answer;

pub fn solution(input: String) -> Answer<u32, u32> {
    let bits: Vec<Vec<u32>> = input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).expect("Epic parse fail"))
                .collect()
        })
        .collect();
    let bitvec_len = bits[0].len();

    let column_counts = (0..bitvec_len).map(|i| count_column(&bits, i));

    let gamma = bitvec_to_num(
        column_counts
            .clone()
            .map(|(zeros, ones)| if zeros > ones { 0 } else { 1 }),
    );

    let epsilon =
        bitvec_to_num(column_counts.map(|(zeros, ones)| if zeros > ones { 1 } else { 0 }));

    let part1 = gamma * epsilon;

    let mut oxygen_rating: Vec<Vec<u32>> = bits.clone();
    let mut column_n: usize = 0;
    while oxygen_rating.len() != 1 {
        let most_common = oxygen_bit_criteria(&oxygen_rating, column_n);
        oxygen_rating = oxygen_rating
            .into_iter()
            .filter(|bitvec| bitvec[column_n] == most_common)
            .collect();
        column_n += 1;
    }

    let mut co2_rating: Vec<Vec<u32>> = bits.clone();
    let mut column_n: usize = 0;
    while co2_rating.len() != 1 {
        let most_common = co2_bit_criteria(&co2_rating, column_n);
        co2_rating = co2_rating
            .into_iter()
            .filter(|bitvec| bitvec[column_n] == most_common)
            .collect();
        column_n += 1;
    }

    let part2 = bitvec_to_num(oxygen_rating[0].clone()) * bitvec_to_num(co2_rating[0].clone());

    Answer(part1, part2)
}

fn count_bits(bits: impl IntoIterator<Item = u32>) -> (u32, u32) {
    bits.into_iter().fold((0, 0), |acc, elem| match elem {
        0 => (acc.0 + 1, acc.1),
        1 => (acc.0, acc.1 + 1),
        _ => panic!(),
    })
}

fn count_column(bits: &[Vec<u32>], column_n: usize) -> (u32, u32) {
    count_bits(bits.iter().map(|row| row[column_n]))
}

fn oxygen_bit_criteria(bits: &[Vec<u32>], column_n: usize) -> u32 {
    let total = count_column(bits, column_n);
    if total.1 >= total.0 {
        1
    } else {
        0
    }
}

fn co2_bit_criteria(bits: &[Vec<u32>], column_n: usize) -> u32 {
    let total = count_column(bits, column_n);
    if total.0 <= total.1 {
        0
    } else {
        1
    }
}

fn bitvec_to_num<T: IntoIterator<Item = u32>>(bits: T) -> u32 {
    u32::from_str_radix(
        &bits
            .into_iter()
            .map(|bit| char::from_digit(bit, 10).unwrap())
            .collect::<String>(),
        2,
    )
    .unwrap()
}
