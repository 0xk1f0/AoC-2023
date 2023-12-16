use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    OUTSIDE,
}

fn direct_beam(beam: &mut (usize, usize, Direction)) {}

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a matrix
    let mut matrix: Vec<Vec<char>> = Vec::new();
    // all beams
    let mut beams_list: Vec<(usize, usize, Direction)> = Vec::new();
    // all activated elements corsd
    let mut activated_list: HashSet<(usize, usize)> = HashSet::new();
    // iterate lines and add to matrix
    for line in lines {
        matrix.push(line.chars().collect());
    }
    // instantiate a new beam
    beams_list.push((0, 0, Direction::DOWN));
    activated_list.insert((0, 0));
    // loop protect
    let mut loop_protect = 0;
    // walk the beams
    while loop_protect < 800 && beams_list.len() != 0 {
        loop_protect += 1;
        let mut new_beams: Vec<(usize, usize, Direction)> = Vec::new();
        beams_list
            .iter_mut()
            .map(|beam| match beam.2 {
                Direction::UP => {
                    if (beam.1 - 1) > 1000 {
                        *beam = (beam.0, beam.1, Direction::OUTSIDE);
                        return;
                    }
                    *beam = (beam.0, beam.1 - 1, beam.2);
                    match matrix[beam.1][beam.0] {
                        '.' => {
                            *beam = (beam.0, beam.1, beam.2);
                        }
                        '-' => {
                            *beam = (beam.0, beam.1, Direction::RIGHT);
                            new_beams.push((beam.0, beam.1, Direction::LEFT));
                        }
                        '|' => *beam = (beam.0, beam.1, beam.2),
                        '/' => {
                            *beam = (beam.0, beam.1, Direction::RIGHT);
                        }
                        '\\' => {
                            *beam = (beam.0, beam.1, Direction::LEFT);
                        }
                        _ => panic!("Unexpected Direction"),
                    }
                    activated_list.insert((beam.0, beam.1));
                }
                Direction::RIGHT => {
                    if (beam.0 + 1) > matrix[beam.1].len() - 1 {
                        *beam = (beam.0, beam.1, Direction::OUTSIDE);
                        return;
                    }
                    *beam = (beam.0 + 1, beam.1, beam.2);
                    match matrix[beam.1][beam.0] {
                        '.' => {
                            *beam = (beam.0, beam.1, beam.2);
                        }
                        '-' => *beam = (beam.0, beam.1, beam.2),
                        '|' => {
                            *beam = (beam.0, beam.1, Direction::UP);
                            new_beams.push((beam.0, beam.1, Direction::DOWN));
                        }
                        '/' => {
                            *beam = (beam.0, beam.1, Direction::UP);
                        }
                        '\\' => {
                            *beam = (beam.0, beam.1, Direction::DOWN);
                        }
                        _ => panic!("Unexpected Direction"),
                    }
                    activated_list.insert((beam.0, beam.1));
                }
                Direction::DOWN => {
                    if (beam.1 + 1) > matrix.len() - 1 {
                        *beam = (beam.0, beam.1, Direction::OUTSIDE);
                        return;
                    }
                    *beam = (beam.0, beam.1 + 1, beam.2);
                    match matrix[beam.1][beam.0] {
                        '.' => {
                            *beam = (beam.0, beam.1, beam.2);
                        }
                        '-' => {
                            *beam = (beam.0, beam.1, Direction::LEFT);
                            new_beams.push((beam.0, beam.1, Direction::RIGHT));
                        }
                        '|' => *beam = (beam.0, beam.1, beam.2),
                        '/' => {
                            *beam = (beam.0, beam.1, Direction::LEFT);
                        }
                        '\\' => {
                            *beam = (beam.0, beam.1, Direction::RIGHT);
                        }
                        _ => panic!("Unexpected Direction"),
                    }
                    activated_list.insert((beam.0, beam.1));
                }
                Direction::LEFT => {
                    if (beam.0 - 1) > 1000 {
                        *beam = (beam.0, beam.1, Direction::OUTSIDE);
                        return;
                    }
                    *beam = (beam.0 - 1, beam.1, beam.2);
                    match matrix[beam.1][beam.0] {
                        '.' => {
                            *beam = (beam.0, beam.1, beam.2);
                        }
                        '-' => *beam = (beam.0, beam.1, beam.2),
                        '|' => {
                            *beam = (beam.0, beam.1, Direction::UP);
                            new_beams.push((beam.0, beam.1, Direction::DOWN));
                        }
                        '/' => {
                            *beam = (beam.0, beam.1, Direction::DOWN);
                        }
                        '\\' => {
                            *beam = (beam.0, beam.1, Direction::UP);
                        }
                        _ => panic!("Unexpected Direction"),
                    }
                    activated_list.insert((beam.0, beam.1));
                }
                Direction::OUTSIDE => {}
            })
            .count();
        // add the new beam
        if new_beams.len() > 0 {
            for beam in new_beams {
                beams_list.push(beam);
            }
        }
        beams_list.retain(|beam| beam.2 != Direction::OUTSIDE);
    }
    activated_list.len()
}

fn main() {
    println!("Part 1> The Sum of Energized Elements is: {}", part_one());
}
