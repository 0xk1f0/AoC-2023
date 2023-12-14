use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn rotate_matrix(m: &mut Vec<Vec<char>>) {
    *m = (0..m[0].len())
        .map(|oi| (0..m.len()).map(|ii| m[ii][oi]).collect())
        .collect();
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

            // test reversed direction
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

fn main() {
    println!("Part 1> The Row/Column Sum is: {}", part_one());
}
