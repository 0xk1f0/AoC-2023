use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a matrix for the loop field
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    // define a cursor for navigating
    let mut cursor: (char, usize, usize, i8, i8) = ('S', 0, 0, 1, 1);
    // iterate each line
    for line in lines {
        // define a x vector and find possible starting point
        let x_pos: Vec<char> = Vec::from_iter(line.chars().enumerate().map(|(i, c)| {
            if c == 'S' {
                cursor = ('S', i, matrix.len(), 0, 0);
                c
            } else {
                c
            }
        }));
        // push x values to matrix
        matrix.push(x_pos);
    }

    // move to first loop
    // @TODO: This is hardcoded because I am too lazy to implement
    // a proper searcher logic for it, so adjust as needed
    cursor = (matrix[cursor.2 + 1][cursor.1], cursor.1, cursor.2 + 1, 0, 1);
    // define a step counter
    let mut step: usize = 1;
    // navigate the cursor
    while cursor.0 != 'S' {
        match cursor.0 {
            '|' => {
                if cursor.4 == -1 {
                    cursor = (
                        matrix[cursor.2 - 1][cursor.1],
                        cursor.1,
                        cursor.2 - 1,
                        0,
                        -1,
                    );
                } else {
                    cursor = (matrix[cursor.2 + 1][cursor.1], cursor.1, cursor.2 + 1, 0, 1);
                }
            }
            '-' => {
                if cursor.3 == -1 {
                    cursor = (
                        matrix[cursor.2][cursor.1 - 1],
                        cursor.1 - 1,
                        cursor.2,
                        -1,
                        0,
                    );
                } else {
                    cursor = (matrix[cursor.2][cursor.1 + 1], cursor.1 + 1, cursor.2, 1, 0);
                }
            }
            'L' => {
                if cursor.3 == -1 {
                    cursor = (
                        matrix[cursor.2 - 1][cursor.1],
                        cursor.1,
                        cursor.2 - 1,
                        0,
                        -1,
                    );
                } else {
                    cursor = (matrix[cursor.2][cursor.1 + 1], cursor.1 + 1, cursor.2, 1, 0);
                }
            }
            'J' => {
                if cursor.3 == 1 {
                    cursor = (
                        matrix[cursor.2 - 1][cursor.1],
                        cursor.1,
                        cursor.2 - 1,
                        0,
                        -1,
                    );
                } else {
                    cursor = (
                        matrix[cursor.2][cursor.1 - 1],
                        cursor.1 - 1,
                        cursor.2,
                        -1,
                        0,
                    );
                }
            }
            '7' => {
                if cursor.3 == 1 {
                    cursor = (matrix[cursor.2 + 1][cursor.1], cursor.1, cursor.2 + 1, 0, 1);
                } else {
                    cursor = (
                        matrix[cursor.2][cursor.1 - 1],
                        cursor.1 - 1,
                        cursor.2,
                        -1,
                        0,
                    );
                }
            }
            'F' => {
                if cursor.3 == -1 {
                    cursor = (matrix[cursor.2 + 1][cursor.1], cursor.1, cursor.2 + 1, 0, 1);
                } else {
                    cursor = (matrix[cursor.2][cursor.1 + 1], cursor.1 + 1, cursor.2, 1, 0);
                }
            }
            '.' => panic!("We hit a wall"),
            _ => panic!("Unexpected Direction"),
        }
        // add a step
        step += 1;
    }

    // return the all up sum
    return step / 2;
}

fn part_two() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a matrix for the loop field
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    // define a cursor for navigating
    let mut cursor: (char, usize, usize, i8, i8) = ('S', 0, 0, 1, 1);
    // iterate each line
    for line in lines {
        // define a x vector and find possible starting point
        let x_pos: Vec<char> = Vec::from_iter(line.chars().enumerate().map(|(i, c)| {
            if c == 'S' {
                cursor = ('S', i, matrix.len(), 0, 0);
                '┌' // @TODO: Attention this is hardcoded and might not work for all matrixes
            } else {
                c
            }
        }));
        // push x values to matrix
        matrix.push(x_pos);
    }
    // move to first loop
    cursor = (matrix[cursor.2 + 1][cursor.1], cursor.1, cursor.2 + 1, 0, 1);

    // define a step counter
    let mut step: usize = 1;
    // navigate the cursor
    while cursor.0 != '┌' {
        match cursor.0 {
            '|' => {
                matrix[cursor.2][cursor.1] = '│'; // replace with proper char
                if cursor.4 == -1 {
                    cursor = (
                        matrix[cursor.2 - 1][cursor.1],
                        cursor.1,
                        cursor.2 - 1,
                        0,
                        -1,
                    );
                } else {
                    cursor = (matrix[cursor.2 + 1][cursor.1], cursor.1, cursor.2 + 1, 0, 1);
                }
            }
            '-' => {
                matrix[cursor.2][cursor.1] = '─'; // replace with proper char
                if cursor.3 == -1 {
                    cursor = (
                        matrix[cursor.2][cursor.1 - 1],
                        cursor.1 - 1,
                        cursor.2,
                        -1,
                        0,
                    );
                } else {
                    cursor = (matrix[cursor.2][cursor.1 + 1], cursor.1 + 1, cursor.2, 1, 0);
                }
            }
            'L' => {
                matrix[cursor.2][cursor.1] = '└'; // replace with proper char
                if cursor.3 == -1 {
                    cursor = (
                        matrix[cursor.2 - 1][cursor.1],
                        cursor.1,
                        cursor.2 - 1,
                        0,
                        -1,
                    );
                } else {
                    cursor = (matrix[cursor.2][cursor.1 + 1], cursor.1 + 1, cursor.2, 1, 0);
                }
            }
            'J' => {
                matrix[cursor.2][cursor.1] = '┘'; // replace with proper char
                if cursor.3 == 1 {
                    cursor = (
                        matrix[cursor.2 - 1][cursor.1],
                        cursor.1,
                        cursor.2 - 1,
                        0,
                        -1,
                    );
                } else {
                    cursor = (
                        matrix[cursor.2][cursor.1 - 1],
                        cursor.1 - 1,
                        cursor.2,
                        -1,
                        0,
                    );
                }
            }
            '7' => {
                matrix[cursor.2][cursor.1] = '┐'; // replace with proper char
                if cursor.3 == 1 {
                    cursor = (matrix[cursor.2 + 1][cursor.1], cursor.1, cursor.2 + 1, 0, 1);
                } else {
                    cursor = (
                        matrix[cursor.2][cursor.1 - 1],
                        cursor.1 - 1,
                        cursor.2,
                        -1,
                        0,
                    );
                }
            }
            'F' => {
                matrix[cursor.2][cursor.1] = '┌'; // replace with proper char
                if cursor.3 == -1 {
                    cursor = (matrix[cursor.2 + 1][cursor.1], cursor.1, cursor.2 + 1, 0, 1);
                } else {
                    cursor = (matrix[cursor.2][cursor.1 + 1], cursor.1 + 1, cursor.2, 1, 0);
                }
            }
            '.' => panic!("We hit a wall"),
            _ => panic!("Unexpected Direction"),
        }
        // add a step
        step += 1;
    }
    // a sum to count all occurences of enclosed
    let mut sum: usize = 0;
    // loop all row vectors of matrix
    for i in 0..matrix.len() {
        // keep track of pipe instances
        let mut s_seen: usize = 0;
        // keep track of special char sequences
        let mut last_special: char = 'x';
        // loop all chars in matrix row
        for x in 0..matrix[i].len() {
            let current = matrix[i][x].clone();
            if current == '│' {
                // count a pipe as seen
                s_seen += 1;
            } else if current == '┌' {
                last_special = '┌';
            } else if current == '└' {
                last_special = '└';
            } else if current == '┐' {
                if last_special == '└' {
                    // we fullfilled a turn and
                    // count another pipe as seen
                    s_seen += 1;
                }
                last_special = 'x';
            } else if current == '┘' {
                if last_special == '┌' {
                    // again we fullfilled a turn and
                    // count another pipe as seen
                    s_seen += 1;
                }
                last_special = 'x';
            } else if current == '─' {
                // this is just straight and skips
                continue;
            } else {
                // should the number of seen pipes not be whole
                // the last special character not fullfilled and ended
                // and the seen value non zero
                if s_seen % 2 != 0 && s_seen != 0 && last_special == 'x' {
                    // count one to sum
                    sum += 1;
                }
            }
        }
    }
    // return the sum of all matches
    return sum;
}

fn main() {
    println!("Part 1> Steps to farthest point: {}", part_one());
    println!("Part 2> Enclosed tiles: {}", part_two());
}
