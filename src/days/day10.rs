use std::panic;

use crate::Answer;

pub fn solution(input: String) -> Answer<u64, u64> {
    let lines: Vec<&str> = input.lines().collect();
    let mut part1 = 0;
    let mut part2_scores: Vec<u64> = vec![];
    for line in lines {
        let r = parse_line(&line.chars().collect::<Vec<_>>());
        //all lines should contain errors
        assert!(r.is_err());
        part1 += match r.clone().unwrap_err() {
            ParseError::CorruptChunk((')', _)) => 3,
            ParseError::CorruptChunk((']', _)) => 57,
            ParseError::CorruptChunk(('}', _)) => 1197,
            ParseError::CorruptChunk(('>', _)) => 25137,
            ParseError::ExtraCloser(_) => panic!(),
            _ => 0,
        };
        if let Err(ParseError::Incomplete(_)) = r {
            let end = complete_line(&line.chars().collect::<Vec<_>>());
            part2_scores.push(
                end.chars()
                    .map(|ch| match ch {
                        ')' => 1,
                        ']' => 2,
                        '>' => 3,
                        '}' => 4,
                        _ => panic!("bad char"),
                    })
                    .fold(0, |acc, i| acc * 5 + i),
            )
        }
    }
    part2_scores.sort_unstable();
    let part2 = part2_scores[part2_scores.len() / 2];
    Answer(part1, part2)
}

#[derive(Clone)]
enum ParseError {
    //wrong charachter char at position usize
    CorruptChunk((char, usize)),
    ExtraCloser((char, usize)),
    Incomplete((char, usize)),
}

//unnecessarily detailed part 1 parsing function
fn parse_line(line: &[char]) -> Result<(), ParseError> {
    let mut stack: Vec<char> = vec![];
    for (i, ch) in line.iter().enumerate() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(*ch),
            ')' | ']' | '}' | '>' => match stack.pop() {
                None => return Err(ParseError::ExtraCloser((*ch, i))),
                Some('(') if *ch != ')' => return Err(ParseError::CorruptChunk((*ch, i))),
                Some('[') if *ch != ']' => return Err(ParseError::CorruptChunk((*ch, i))),
                Some('{') if *ch != '}' => return Err(ParseError::CorruptChunk((*ch, i))),
                Some('<') if *ch != '>' => return Err(ParseError::CorruptChunk((*ch, i))),
                Some('(') | Some('[') | Some('{') | Some('<') => (),
                _ => panic!(),
            },
            _ => panic!("bad char in input"),
        }
    }
    if !stack.is_empty() {
        Err(ParseError::Incomplete((
            *line.last().unwrap(),
            line.len() - 1,
        )))
    } else {
        Ok(())
    }
}
//we already know stack is gonna
fn complete_line(line: &[char]) -> String {
    let mut stack: Vec<char> = vec![];
    for ch in line {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(*ch),
            ')' | ']' | '}' | '>' => {
                stack.pop().unwrap();
            }
            _ => panic!("bad char in input"),
        }
    }

    if !stack.is_empty() {
        return stack
            .iter()
            .rev()
            .map(|ch| match ch {
                '(' => ')',
                '[' => ']',
                '<' => '>',
                '{' => '}',
                _ => panic!("bad char"),
            })
            .collect::<String>();
    } else {
        //stack should be non empty
        panic!();
    }
}
