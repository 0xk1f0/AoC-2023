use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // a matrix for the galaxy
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    // a lookup table for galaxies
    let mut galaxies: HashMap<usize, (usize, usize)> = HashMap::new();
    // iterate each line
    for line in lines {
        // define a x vector and find possible starting point
        let x_pos: Vec<char> = Vec::from_iter(line.chars());
        // push x values to matrix
        matrix.push(x_pos);
    }
    // expand horizontally
    let mut expand_rows: Vec<usize> = Vec::new();
    let mut temp: Vec<char> = Vec::new();
    for i in 0..matrix.len() {
        if matrix[i].iter().any(|&c| c != '.') {
            continue;
        } else {
            temp = matrix[i].clone();
            expand_rows.push(i + 1);
        }
    }
    // insert new rows
    for (c, x) in expand_rows.iter().enumerate() {
        matrix.insert(c + x, temp.clone());
    }
    // expand vertically
    let mut expand_colls: Vec<usize> = Vec::new();
    for i in 0..matrix[0].len() {
        let mut breaked: bool = false;
        for x in 0..matrix.len() {
            if matrix[x][i] != '.' {
                breaked = true;
                break;
            }
        }
        if breaked {
            continue;
        } else {
            expand_colls.push(i + 1);
        }
    }
    // insert new columns
    for (c, x) in expand_colls.iter().enumerate() {
        for i in 0..matrix.len() {
            matrix[i].insert(c + x, '.');
        }
    }
    // discover galaxies
    let mut counter: usize = 1;
    for i in 0..matrix.len() {
        let _ = matrix[i]
            .iter()
            .enumerate()
            .map(|(n, &c)| {
                if c == '#' {
                    galaxies.insert(counter, (n, i));
                    counter += 1;
                }
            })
            .count();
    }
    // new hasmap for paths and sums
    let mut sum_map: HashMap<String, usize> = HashMap::new();
    // find path costs in all keys
    for i in galaxies.keys() {
        for j in galaxies.keys() {
            let this = galaxies.get(&j).unwrap();
            let that = galaxies.get(&i).unwrap();
            let key = format!("{}-{}", i.max(j), j.min(i));
            let x_steps: usize;
            if this.0 > that.0 {
                x_steps = this.0 - that.0;
            } else {
                x_steps = that.0 - this.0;
            }
            let y_steps: usize;
            if this.1 > that.1 {
                y_steps = this.1 - that.1;
            } else {
                y_steps = that.1 - this.1;
            }
            let result = x_steps + y_steps;
            // do not duplicate paths
            if i != j {
                sum_map.insert(key, result);
            }
        }
    }
    // return the all up sum
    return sum_map.values().sum();
}

fn main() {
    println!("Part 1> The pathlength sum is: {}", part_one());
}
