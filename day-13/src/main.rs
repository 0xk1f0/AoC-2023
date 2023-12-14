use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn rotate_matrix(m: &mut Vec<Vec<char>>) {
    *m = (0..m[0].len())
        .map(|oi| (1..m.len() + 1).map(|ii| m[m.len() - ii][oi]).collect())
        .collect();
}

fn find_occurence(m: &mut Vec<Vec<char>>) -> usize {
    for x in 1..m.len() {
        if m[0..x].iter().rev().zip(m[x..].iter()).all(|(a, b)| a == b) {
            return x;
        }
    }
    0
}

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a matrix
    let mut matrix_list: Vec<Vec<Vec<char>>> = vec![vec![]];
    // iterate lines and add matrixes
    for line in lines {
        if line.len() == 0 {
            matrix_list.push(vec![]);
        } else {
            matrix_list.last_mut().unwrap().push(line.chars().collect());
        }
    }
    // iterate each matrix and get total sum
    matrix_list
        .iter_mut()
        .map(|matrix| {
            // test normal direction
            for x in 1..matrix.len() {
                if matrix[0..x]
                    .iter()
                    .rev()
                    .zip(matrix[x..].iter())
                    .all(|(a, b)| a == b)
                {
                    return x * 100;
                }
            }

            // rotate
            rotate_matrix(matrix);

            // test rotated direction
            for y in 1..matrix.len() {
                if matrix[0..y]
                    .iter()
                    .rev()
                    .zip(matrix[y..].iter())
                    .all(|(a, b)| a == b)
                {
                    return y;
                }
            }

            0
        })
        .sum()
}

fn part_two() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a matrix
    let mut matrix_list: Vec<Vec<Vec<char>>> = vec![vec![]];
    // iterate lines and add matrixes
    for line in lines {
        if line.len() == 0 {
            matrix_list.push(vec![]);
        } else {
            matrix_list.last_mut().unwrap().push(line.chars().collect());
        }
    }
    // iterate each matrix and get total sum
    matrix_list
        .iter_mut()
        .map(|matrix| {
            // define original value
            let mut base: usize;
            // test normal direction
            base = find_occurence(matrix) * 100;
            // define a second comparison
            let mut base_before = base.clone();
            // brute force exchange
            'oi: for row in 0..matrix.len() {
                for entry in 0..matrix[row].len() {
                    // match current and switch
                    match matrix[row][entry] {
                        '#' => {
                            matrix[row][entry] = '.';
                            // retest
                            let new = find_occurence(matrix) * 100;
                            if base != new && new != 0 {
                                base = new;
                                break 'oi;
                            }
                            matrix[row][entry] = '#';
                        }
                        '.' => {
                            matrix[row][entry] = '#';
                            // retest
                            let new = find_occurence(matrix) * 100;
                            if base != new && new != 0 {
                                base = new;
                                break 'oi;
                            }
                            matrix[row][entry] = '.';
                        }
                        _ => panic!("should not happen"),
                    }
                }
            }
            // return if non zero and non equal
            if base != 0 && base != base_before {
                return base;
            }
            // else rotate
            matrix.reverse();
            rotate_matrix(matrix);
            // and test rotated direction
            // reset if needed
            base = find_occurence(matrix);
            base_before = base.clone();

            // brute force exchange
            'oi: for row in 0..matrix.len() {
                for entry in 0..matrix[row].len() {
                    // match current and switch
                    match matrix[row][entry] {
                        '#' => {
                            matrix[row][entry] = '.';
                            // retest
                            let new = find_occurence(matrix);
                            if base != new && new != 0 {
                                base = new;
                                break 'oi;
                            }
                            matrix[row][entry] = '#';
                        }
                        '.' => {
                            matrix[row][entry] = '#';
                            // retest
                            let new = find_occurence(matrix);
                            if base != new && new != 0 {
                                base = new;
                                break 'oi;
                            }
                            matrix[row][entry] = '.';
                        }
                        _ => panic!("should not happen"),
                    }
                }
            }

            // return if non zero and non equal
            if base != 0 && base != base_before {
                return base;
            }
            0
        })
        .sum()
}

fn main() {
    println!("Part 1> The Row/Column Sum is: {}", part_one());
    println!("Part 2> The Row/Column Sum is: {}", part_two());
}
