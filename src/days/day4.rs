use crate::Answer;
type Board = [[(u32, bool); 5]; 5];

fn is_winner(board: &Board) -> bool {
    for row in board {
        if row.map(|(_, b)| b).iter().all(|b| *b) {
            return true;
        }
    }

    for colunn_n in 0..5 {
        if board[0][colunn_n].1
            && board[1][colunn_n].1
            && board[2][colunn_n].1
            && board[3][colunn_n].1
            && board[4][colunn_n].1
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
            let mut board_arr = [[(0, false); 5]; 5];
            for (i, n) in board_slice.iter().enumerate() {
                let row = i / 5;
                let column = i % 5;
                board_arr[row as usize][column as usize] = (*n, false);
            }
            board_arr
        })
        .collect();

    let mut part1: Option<u32> = None;
    for n in sequence {
        //blot number on each board
        for b in boards.iter_mut() {
            for row in b.iter_mut() {
                for number in row.iter_mut() {
                    if n == number.0 {
                        number.1 = true;
                    }
                }
            }
            //if that board is then a winner
            if is_winner(b) {
                //sum of unmarked numbers
                part1 = Some(
                    b.concat()
                        .into_iter()
                        .map(|(i, b)| if b { 0 } else { i })
                        .sum::<u32>()
                        * n,
                );
                dbg!(b);
                break;
            }
        }
        if part1.is_some() {
            break;
        }
    }

    Answer(part1.expect("didnt find winner"), 1u32)
}
