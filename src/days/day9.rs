use crate::Answer;

pub fn solution(input: String) -> Answer<u32, usize> {
    let mut nums = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let height = nums.len();
    let width = nums[0].len();

    let mut mins: Vec<u32> = Vec::new();
    //find local minima
    for (i, row) in nums.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            let up = if i == 0 { true } else { nums[i - 1][j] > *x };
            let down = if i == height - 1 {
                true
            } else {
                nums[i + 1][j] > *x
            };
            let left = if j == 0 { true } else { nums[i][j - 1] > *x };
            let right = if j == width - 1 {
                true
            } else {
                nums[i][j + 1] > *x
            };
            if up && down && left && right {
                mins.push(nums[i][j])
            }
        }
    }

    let part1 = mins.iter().map(|x| x + 1).sum();
    //get the size of each basin
    let mut size_list: Vec<usize> = Vec::new();
    for i in 0..nums.len() {
        for j in 0..nums[0].len() {
            let s = basin_size(&mut nums, i, j);
            size_list.push(s);
        }
    }
    size_list.sort_unstable();
    let part2 = size_list.iter().rev().take(3).product();
    Answer(part1, part2)
}

fn basin_size(tiles: &mut Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    if tiles[i][j] == 9 {
        return 0;
    }

    tiles[i][j] = 9;
    let up = if i != 0 {
        basin_size(tiles, i - 1, j)
    } else {
        0
    };

    let down = if i != tiles.len() - 1 {
        basin_size(tiles, i + 1, j)
    } else {
        0
    };

    let left = if j != 0 {
        basin_size(tiles, i, j - 1)
    } else {
        0
    };
    let right = if j != tiles[0].len() - 1 {
        basin_size(tiles, i, j + 1)
    } else {
        0
    };
    1 + up + down + left + right
}

#[allow(dead_code)]
fn print_square(list: &[Vec<u32>]) {
    let v: Vec<Vec<&str>> = list
        .iter()
        .map(|v| {
            v.iter()
                .map(|n| if n == &9u32 { "*" } else { " " })
                .collect()
        })
        .collect();

    for _ in 0..&v[0].len() * 2 + 2 {
        print!("-")
    }
    println!();
    for row in &v {
        print!("|");
        for item in row {
            print!(" {}", item);
        }
        print!("|");
        println!();
    }
    for _ in 0..&v[0].len() * 2 + 2 {
        print!("-")
    }
    println!();
}

#[test]
fn test() {
    let test_input = String::from("2199943210\n3987894921\n9856789892\n8767896789\n9899965678");
    let soln = solution(test_input);
    assert_eq!(soln.0, 15);
    assert_eq!(soln.1, 1134);
}
