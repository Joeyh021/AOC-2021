use crate::Answer;
use std::collections::HashSet;

struct Display {
    input: [Digit; 10],
    output: [Digit; 4],
}
impl Default for Display {
    fn default() -> Self {
        Display {
            input: [
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
            output: [
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ],
        }
    }
}

type Digit = HashSet<char>;

fn calc_segments(digits: &[Digit]) -> [&Digit; 10] {
    //there are for sure more efficient ways to get these but oh well
    let one = digits
        .iter()
        .find(|s| s.len() == 2)
        .expect("Could not find one");
    let seven = digits
        .iter()
        .find(|s| s.len() == 3)
        .expect("Could not find seven");
    let four = digits
        .iter()
        .find(|s| s.len() == 4)
        .expect("Could not find four");
    let eight = digits
        .iter()
        .find(|s| s.len() == 7)
        .expect("Could not find eight");
    //len(3-1) == 3
    let three = digits
        .iter()
        .find(|s| s.len() == 5 && s.difference(one).count() == 3)
        .expect("Could not find three");
    //len(2) == 5 && len(2-4) ==3
    let two = digits
        .iter()
        .find(|s| s.len() == 5 && s.difference(four).count() == 3)
        .expect("Could not find two");
    //len(5-4) == 2 && len(5)  == 5
    let five = digits
        .iter()
        .find(|s| s.len() == 5 && s != &two && s != &three)
        .expect("Could not find five");
    let six = digits
        .iter()
        .find(|s| s.len() == 6 && s.difference(one).count() == 5)
        .expect("Could not find six");
    let nine = digits
        .iter()
        .find(|s| s.len() == 6 && s.difference(five).count() == 1)
        .expect("Could not find nine");
    let zero = digits
        .iter()
        .find(|s| s.len() == 6 && s != &nine && s != &six)
        .expect("Could not find zero");
    [one, two, three, four, five, six, seven, eight, nine, zero]
}

pub fn solution(input: String) -> Answer<usize, usize> {
    let mut displays: Vec<Display> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut display = Display::default();
        for (i, part) in parts.iter().enumerate().take(10) {
            display.input[i] = HashSet::from_iter(part.chars());
        }

        for (i, part) in parts.iter().enumerate().take(15).skip(11) {
            display.output[i - 11] = HashSet::from_iter(part.chars());
        }
        displays.push(display);
    }
    let mappings = displays.iter().map(|d| calc_segments(&d.input));
    let part2: usize = displays
        .iter()
        .map(|d| &d.output)
        .zip(mappings)
        .map::<usize, _>(|d| {
            d.0.iter()
                .map(|digit| d.1.iter().position(|x| x == &digit).unwrap())
                .sum()
        })
        .sum();

    let part1 = displays
        .iter()
        .flat_map(|d| &d.output)
        .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
        .count();

    Answer::untimed(part1, part2)
}
