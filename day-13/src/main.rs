use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // collector vector for preventing duplicates
    let mut collector: Vec<usize> = Vec::new();
    // define a allup sum
    let mut sum: usize = 0;
    // offset for keeping track of position
    let mut offset: usize = 0;
    // another iterator for duplicate prevention
    let mut side_iter: usize = 0;
    // sum for number of emptylines for col matching
    let mut empty_lines: usize = 0;
    // for rows
    for (i, line) in lines.iter().enumerate() {
        if line.len() > 0 {
            if i > offset && line == &lines[i - 1] {
                let mut match_offset: usize = 1;
                let mut broken: bool = true;
                while i + match_offset < lines.len() && &lines[i + match_offset].len() != &0 && &lines[i - (match_offset + 1)].len() != &0 {
                    if &lines[i + match_offset] != &lines[i - (match_offset + 1)] {
                        broken = true;
                        break;
                    }
                    broken = false;
                    match_offset += 1;
                }
                if !broken {
                    if !collector.contains(&side_iter) {
                        collector.push(side_iter);
                        sum += i - offset;
                    }
                    continue;
                }else if &lines[i + match_offset].len() == &0 || &lines[i - (match_offset + 1)].len() == &0 {
                    if !collector.contains(&side_iter) {
                        collector.push(side_iter);
                        sum += i - offset;
                    }
                    sum += i - offset;
                    continue;
                }
            }
        } else {
            empty_lines += 1;
            offset = i;
            side_iter = i + 1;
            continue;
        }
    }
    // for columns
    offset = 0;
    let mut iterator: usize = 0;
    for _x in 0..empty_lines + 1 {
        let mut matrix: Vec<String> = Vec::new();
        while lines[iterator].len() > 0 {
            for i in 0..lines[iterator].len() {
                if matrix.len() < i + 1 {
                    matrix.push(String::from(lines[iterator].chars().nth(i).unwrap()));
                } else {
                    let mut curr_string = matrix[i].clone();
                    curr_string.push(lines[iterator].chars().nth(i).unwrap());
                    matrix[i] = curr_string;
                }
            }
            if iterator < lines.len() - 1 {
                iterator += 1;
            } else {
                break;
            }
        }
        for (i, line) in matrix.iter().enumerate() {
            if i > 0 && line == &matrix[i - 1] {
                let mut match_offset: usize = 1;
                let mut broken: bool = true;
                while i > match_offset && i < (matrix.len() - (match_offset + 1)) {
                    if &matrix[i + match_offset] != &matrix[i - (match_offset + 1)] {
                        broken = true;
                        break;
                    }
                    broken = false;
                    match_offset += 1;
                }
                if !broken {
                    if !collector.contains(&offset) {
                        collector.push(offset);
                        sum += (i - 1) * 100;
                    }
                    break;
                }else if i < match_offset || i >= (matrix.len() - (match_offset + 1)) {
                    if !collector.contains(&offset) {
                        collector.push(offset);
                        sum += (i - 1) * 100;
                    }
                    break;
                }
            }
        }
        offset += matrix[0].len() + 1;
        iterator += 1;
    }

    return sum;
}

fn main() {
    println!("Part 1> Row Column Sum: {}", part_one());
}
