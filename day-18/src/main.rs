use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a movements
    let mut movements: Vec<(isize, isize)> = Vec::new();
    // max length of motion
    let (mut curr_down, mut curr_right): (usize, usize) = (0,0);
    let (mut max_down, mut max_right): (usize, usize) = (0,0);
    // empty matrix
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines {
        // direction
        let dir = line.split(" ").nth(0).unwrap();
        // move range
        let range: isize = line.split(" ").nth(1).unwrap().parse().unwrap();
        // final tuple
        movements.insert(0, match dir {
            "U" => {
                curr_down -= range as usize;
                max_down = max_down.max(curr_down);
                (0, range * -1)
            },
            "D" => {
                curr_down += range as usize;
                max_down = max_down.max(curr_down);
                (0, range)
            },
            "L" => {
                curr_right -= range as usize;
                max_right = max_right.max(curr_right);
                (range * -1, 0)
            },
            "R" => {
                curr_right += range as usize;
                max_right = max_right.max(curr_right);
                (range, 0)
            },
            _ => panic!("Unexpected Direction")
        })
    }

    println!("Movements: {:?}", movements);

    println!("Matrix will be: {}x{}", max_right + 1, max_down + 1);

    // fill the new matrix
    for x in 0..max_down + 1 {
        matrix.push(vec!['.']);
        for _y in 0..max_right {
            matrix[x].push('.');
        }
    }

    println!("Matrix: {:?}", matrix);

    let mut start: (usize, usize) = (0,0);
    while movements.len() > 0 {
        let current_move = movements.last().unwrap();
        if current_move.1 == 0 {
            for tile in 0..current_move.0 + 1 {
                matrix[start.1][start.0 + tile as usize] = '#';
            }
            start = (start.0 + current_move.0 as usize, start.1);
        } else {
            for tile in 0..current_move.1 + 1 {
                matrix[start.1 + tile as usize][start.0] = '#';
            }
            start = (start.0, start.1 + current_move.1 as usize);
        }
        movements.pop();
    }

    println!("Matrix: {:?}", matrix);

    0
}

fn main() {
    println!("Part 1> The Area is: {}", part_one());
}
