use std::fs;

pub fn solution(input: &str) -> (String, String) {
    let file = fs::read_to_string(input).expect("Could not open input file");

    let bits: Vec<Vec<u32>> = file
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).expect("Epic parse fail"))
                .collect()
        })
        .collect();

    let bitvec_len = bits[0].len();
    let mut bitvec = vec![(0, 0); bitvec_len];
    for column in 0..bitvec_len {
        for row in &bits {
            match row[column] {
                0 => bitvec[column].0 += 1,
                1 => bitvec[column].1 += 1,
                _ => panic!("youve really fucked up here. use bools dickhead"),
            }
        }
    }

    let gamma_string = bitvec
        .iter()
        .map(|(zeros, ones)| if zeros > ones { '0' } else { '1' })
        .collect::<String>();
    let gamma = u32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon_string = bitvec
        .iter()
        .map(|(zeros, ones)| if zeros > ones { '1' } else { '0' })
        .collect::<String>();
    let epsilon = u32::from_str_radix(&epsilon_string, 2).unwrap();
    let part1: String = (gamma * epsilon).to_string();

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

    let part2 = (u32::from_str_radix(
        &oxygen_rating[0]
            .iter()
            .map(|bit| char::from_digit(*bit, 10).unwrap())
            .collect::<String>(),
        2,
    )
    .unwrap()
        * u32::from_str_radix(
            &co2_rating[0]
                .iter()
                .map(|bit| char::from_digit(*bit, 10).unwrap())
                .collect::<String>(),
            2,
        )
        .unwrap())
    .to_string();
    (part1, part2)
}

fn oxygen_bit_criteria(bits: &Vec<Vec<u32>>, column_n: usize) -> u32 {
    let column = bits.iter().map(|row| row[column_n]);
    let mut total = (0, 0);
    for bit in column {
        match bit {
            0 => total.0 += 1,
            1 => total.1 += 1,
            _ => panic!("fucked up"),
        }
    }
    if total.1 >= total.0 {
        1
    } else {
        0
    }
}

fn co2_bit_criteria(bits: &Vec<Vec<u32>>, column_n: usize) -> u32 {
    let column = bits.iter().map(|row| row[column_n]);
    let mut total = (0, 0);
    for bit in column {
        match bit {
            0 => total.0 += 1,
            1 => total.1 += 1,
            _ => panic!("fucked up"),
        }
    }
    if total.0 <= total.1 {
        0
    } else {
        1
    }
}
