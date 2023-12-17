use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// directions for easier matching
#[derive(PartialEq, Copy, Clone, Debug, Hash, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    OUTSIDE,
}

fn trace_path(matrix: &Vec<Vec<char>>, x: usize, y: usize, direction: Direction) -> usize {
    // all beams
    let mut beams_list: Vec<(usize, usize, Direction)> = Vec::new();
    // all activated element cords
    let mut activated_list: HashSet<(usize, usize)> = HashSet::new();
    // all visited elements with direction included
    let mut traversed_list: HashSet<(usize, usize, Direction)> = HashSet::new();
    // instantiate a new beam
    beams_list.push((x, y, direction));
    // activate it
    activated_list.insert((x, y));
    // initiator vector to match first element
    let mut initiator: bool = true;
    // walk the beams until there are none left
    while beams_list.len() != 0 {
        // new beam initiator
        let mut new_beams: Vec<(usize, usize, Direction)> = Vec::new();
        beams_list
            .iter_mut()
            .map(|beam| {
                // checks for early pathing exit
                let mut activated = false;
                let mut re_check = true;
                // match beam direction
                match beam.2 {
                    Direction::UP => {
                        if (beam.1 - 1) > 1000 {
                            *beam = (beam.0, beam.1, Direction::OUTSIDE);
                            return;
                        }
                        if traversed_list.contains(&beam) {
                            activated = true;
                        }
                        if !initiator {
                            *beam = (beam.0, beam.1 - 1, beam.2);
                        } else {
                            activated = true;
                        }
                        match matrix[beam.1][beam.0] {
                            '.' => {
                                *beam = (beam.0, beam.1, beam.2);
                            }
                            '-' => {
                                *beam = (beam.0, beam.1, Direction::RIGHT);
                                new_beams.push((beam.0, beam.1, Direction::LEFT));
                                re_check = false; // dont early exit
                            }
                            '|' => {
                                *beam = (beam.0, beam.1, beam.2);
                            }
                            '/' => {
                                *beam = (beam.0, beam.1, Direction::RIGHT);
                            }
                            '\\' => {
                                *beam = (beam.0, beam.1, Direction::LEFT);
                            }
                            _ => panic!("Unexpected Direction"),
                        }
                    }
                    Direction::RIGHT => {
                        if (beam.0 + 1) > matrix[beam.1].len() - 1 {
                            *beam = (beam.0, beam.1, Direction::OUTSIDE);
                            return;
                        }
                        if traversed_list.contains(&beam) {
                            activated = true;
                        }
                        if !initiator {
                            *beam = (beam.0 + 1, beam.1, beam.2);
                        } else {
                            activated = true;
                        }
                        match matrix[beam.1][beam.0] {
                            '.' => {
                                *beam = (beam.0, beam.1, beam.2);
                            }
                            '-' => {
                                *beam = (beam.0, beam.1, beam.2);
                            }
                            '|' => {
                                *beam = (beam.0, beam.1, Direction::UP);
                                new_beams.push((beam.0, beam.1, Direction::DOWN));
                                re_check = false;
                            }
                            '/' => {
                                *beam = (beam.0, beam.1, Direction::UP);
                            }
                            '\\' => {
                                *beam = (beam.0, beam.1, Direction::DOWN);
                            }
                            _ => panic!("Unexpected Direction"),
                        }
                    }
                    Direction::DOWN => {
                        if (beam.1 + 1) > matrix.len() - 1 {
                            *beam = (beam.0, beam.1, Direction::OUTSIDE);
                            return;
                        }
                        if traversed_list.contains(&beam) {
                            activated = true;
                        }
                        if !initiator {
                            *beam = (beam.0, beam.1 + 1, beam.2);
                        } else {
                            activated = true;
                        }
                        match matrix[beam.1][beam.0] {
                            '.' => {
                                *beam = (beam.0, beam.1, beam.2);
                            }
                            '-' => {
                                *beam = (beam.0, beam.1, Direction::LEFT);
                                new_beams.push((beam.0, beam.1, Direction::RIGHT));
                                re_check = false;
                            }
                            '|' => {
                                *beam = (beam.0, beam.1, beam.2);
                            }
                            '/' => {
                                *beam = (beam.0, beam.1, Direction::LEFT);
                            }
                            '\\' => {
                                *beam = (beam.0, beam.1, Direction::RIGHT);
                            }
                            _ => panic!("Unexpected Direction"),
                        }
                    }
                    Direction::LEFT => {
                        if (beam.0 - 1) > 1000 {
                            *beam = (beam.0, beam.1, Direction::OUTSIDE);
                            return;
                        }
                        if traversed_list.contains(&beam) {
                            activated = true;
                        }
                        if !initiator {
                            *beam = (beam.0 - 1, beam.1, beam.2);
                        } else {
                            activated = true;
                        }
                        match matrix[beam.1][beam.0] {
                            '.' => {
                                *beam = (beam.0, beam.1, beam.2);
                            }
                            '-' => {
                                *beam = (beam.0, beam.1, beam.2);
                            }
                            '|' => {
                                *beam = (beam.0, beam.1, Direction::UP);
                                new_beams.push((beam.0, beam.1, Direction::DOWN));
                                re_check = false;
                            }
                            '/' => {
                                *beam = (beam.0, beam.1, Direction::DOWN);
                            }
                            '\\' => {
                                *beam = (beam.0, beam.1, Direction::UP);
                            }
                            _ => panic!("Unexpected Direction"),
                        }
                    }
                    Direction::OUTSIDE => {}
                }
                // if both past and current beam tiles are activated
                // we can early exit
                if re_check && activated && traversed_list.contains(&beam) {
                    *beam = (beam.0, beam.1, Direction::OUTSIDE);
                    return;
                } else {
                    // if we dont seem to have visited these tiles
                    // add them to traversed and activated
                    activated_list.insert((beam.0, beam.1));
                    traversed_list.insert(*beam);
                }
                // set the initator to false
                // only relevant on first run
                initiator = false;
            })
            .count();
        // add the new beam
        for beam in new_beams {
            beams_list.push(beam);
        }
        // filter out expired beams
        beams_list.retain(|beam| beam.2 != Direction::OUTSIDE);
    }
    // return the length of activated items
    activated_list.len()
}

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a matrix
    let mut matrix: Vec<Vec<char>> = Vec::new();
    // iterate lines and add to matrix
    for line in lines {
        matrix.push(line.chars().collect());
    }
    // walk the beams
    trace_path(&matrix, 0, 0, Direction::RIGHT)
}

fn part_two() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a matrix
    let mut matrix: Vec<Vec<char>> = Vec::new();
    // check all sum
    let mut sum = 0;
    // iterate lines and add to matrix
    for line in lines {
        matrix.push(line.chars().collect());
    }
    // walk start and end of each row
    for row in 0..matrix.len() {
        for column in [0, matrix[0].len() - 1] {
            if row == 0 {
                sum = sum.max(trace_path(&matrix, column, row, Direction::RIGHT));
            } else {
                sum = sum.max(trace_path(&matrix, column, row, Direction::LEFT));
            }
        }
    }
    // walk start and end of each column
    for column in 0..matrix[0].len() {
        for row in [0, matrix.len() - 1] {
            if row == 0 {
                sum = sum.max(trace_path(&matrix, column, row, Direction::DOWN));
            } else {
                sum = sum.max(trace_path(&matrix, column, row, Direction::UP));
            }
        }
    }
    // return the max sum
    sum
}

fn main() {
    println!("Part 1> The Sum of Energized Elements is: {}", part_one());
    println!("Part 2> The Highest Sum is: {}", part_two());
}
