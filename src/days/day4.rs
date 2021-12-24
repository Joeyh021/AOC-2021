use crate::Answer;
type Board = [(u32, bool); 25];

fn is_winner(board: &Board) -> bool {
    let bools = board.map(|(_, b)| b);
    //check each row
    for row in 0..5 {
        if bools[row] && bools[row + 1] && bools[row + 2] && bools[row + 3] && bools[row + 4] {
            return true;
        }
    }
    for column in (0..5).map(|i| i * 5) {
        if bools[column]
            && bools[column + 1]
            && bools[column + 2]
            && bools[column + 3]
            && bools[column + 4]
        {
            return true;
        }
    }
    false
}

pub fn solution(input: String) -> Answer<u32, u32> {
    let mut lines = input.split_whitespace();
    let sequence = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap());

    let nums = lines.map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();

    let mut boards: Vec<Board> = nums
        .chunks(25)
        .map(|board_slice| {
            let mut board_arr = [(0, false); 25];
            for (i, n) in board_slice.iter().enumerate() {
                board_arr[i] = (*n, false);
            }
            board_arr
        })
        .collect();

    let mut part1: Option<u32> = None;
    let mut part2: Option<u32> = None;
    for n in sequence {
        //blot number on each board
        for b in boards.iter_mut() {
            for number in b.iter_mut() {
                if n == number.0 {
                    number.1 = true;
                }
            }
            //if that board is then a winner
            if is_winner(b) && part1.is_none() {
                //sum of unmarked numbers
                part1 = Some(b.map(|(i, b)| if b { 0 } else { i }).iter().sum::<u32>() * n);
            }
        }
        //remove all winners
        boards = boards
            .into_iter()
            .filter(|board| !is_winner(board))
            .collect();
        dbg!(&boards.len());

        if boards.len() == 2 {
            part2 = Some(
                boards[0]
                    .map(|(i, b)| if b { 0 } else { i })
                    .iter()
                    .sum::<u32>()
                    * n,
            );
        }
    }
    Answer::untimed(part1.unwrap(), part2.unwrap())
}
