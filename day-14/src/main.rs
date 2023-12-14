use std::fs::File;
use std::io::{BufRead, BufReader};

fn rotate_matrix(m: &mut Vec<Vec<char>>, ccw: bool) {
    *m = (1..m[0].len() + 1)
        .map(|oi| {
            if ccw {
                (1..m.len() + 1)
                    .map(|ii| m[ii - 1][m[0].len() - oi])
                    .collect()
            } else {
                (1..m.len() + 1).map(|ii| m[m.len() - ii][oi - 1]).collect()
            }
        })
        .collect();
}

fn shift_load_matrix(m: &mut Vec<Vec<char>>) -> usize {
    m.iter_mut()
        .map(|l| {
            // find positions of rocks
            let positions: Vec<usize> = l
                .iter()
                .enumerate()
                .filter(|(_i, &c)| c == 'O')
                .map(|(i, _c)| i)
                .collect();
            // iterate positions
            positions
                .iter()
                .map(|&p| {
                    let mut pos: isize = p.clone() as isize;
                    // check how far left we can move
                    while pos - 1 >= 0
                        && l[(pos - 1) as usize] != 'O'
                        && l[(pos - 1) as usize] != '#'
                    {
                        pos -= 1;
                    }
                    // insert at new positon
                    l.insert(pos as usize, 'O');
                    // remove the old
                    l.remove(p + 1);
                    // calculate the load (offset)
                    l.len() - pos as usize
                })
                .sum::<usize>()
        })
        .sum()
}

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: BufReader<File> = BufReader::new(input);
    // define a matrix
    let mut matrix: Vec<Vec<char>> = lines
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    // rotate the matrix
    rotate_matrix(&mut matrix, true);
    // get the total sum of loads
    return shift_load_matrix(&mut matrix);
}

fn part_two() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: BufReader<File> = BufReader::new(input);
    // define a matrix
    let mut matrix: Vec<Vec<char>> = lines
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    // ! We can see that after 1000 cycles, the result
    // ! is the same as after 1000000000
    for _round in 0..1000 {
        // rotate the matrix to north
        rotate_matrix(&mut matrix, true);
        shift_load_matrix(&mut matrix);
        // rotate the matrix to west
        rotate_matrix(&mut matrix, false);
        shift_load_matrix(&mut matrix);
        // rotate the matrix to south
        rotate_matrix(&mut matrix, false);
        shift_load_matrix(&mut matrix);
        // rotate the matrix to east
        rotate_matrix(&mut matrix, false);
        shift_load_matrix(&mut matrix);
        // rotate the matrix back to initial
        rotate_matrix(&mut matrix, false);
        rotate_matrix(&mut matrix, false);
    }
    // calc final sum
    rotate_matrix(&mut matrix, true);
    matrix.iter_mut()
    .map(|l| {
        // only find load of rocks
        l.iter()
            .enumerate()
            .filter(|(_i, &c)| c == 'O')
            .map(|(i, _c)| l.len() - i)
            .sum::<usize>()
    })
    .sum()
}

fn main() {
    println!("Part 1> The Total Load is: {}", part_one());
    println!("Part 2> The Total Load is: {}", part_two());
}
