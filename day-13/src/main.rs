use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // find splits
    let mut all_splits: Vec<usize> = Vec::new();
    all_splits.push(0);
    let _: Vec<_> = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            if l.len() == 0 {
                all_splits.push(i + 1);
            }
        })
        .collect();
    all_splits.push(lines.len());

    // define a allup sum
    let mut sum: usize = 0;

    // flip flop row col
    let mut type_row: bool = false;
    let mut inner_iterate: usize = 0;
    for i in 0..lines.len() {
        if all_splits.contains(&i) {
            let mut current_lines: Vec<String> = lines.clone();
            current_lines = current_lines.into_iter()
                .skip(all_splits[inner_iterate])
                .take(all_splits[inner_iterate + 1] - 1)
                .collect();
            if type_row {
                let _: Vec<_> = current_lines
                    .iter()
                    .enumerate()
                    .map(|(i, l)| {
                        if i < current_lines.len() - 1 && l == &current_lines[i + 1] {
                            println!("RMATCH at {}", i + 1);
                            sum += (i + 1) * 100;
                        }
                    })
                    .collect();
            } else {
                for col in 0..current_lines[0].len() {
                    let mut broken: bool = false;
                    for row in 0..current_lines.len() + 1 {
                        if row < current_lines.len() - 1 {
                            let this_line: Vec<char> = current_lines[col].chars().collect();
                            let that_line: Vec<char> = current_lines[col].chars().collect();
                            if this_line[row] != that_line[row + 1] {
                                broken = true;
                                break;
                            } else {
                                if row > 0 && this_line[row - 1] != that_line[row + 2] {
                                    broken = true;
                                    break;
                                }
                            }
                        }
                    }
                    if broken {
                        println!("VMATCH at {}", col);
                        sum += col;
                        break;
                    }
                }
            }
            type_row = !type_row;
            inner_iterate += 1;
        }
    }

    return sum;
}

fn main() {
    println!("Part 1> Row Column Sum: {}", part_one());
}
