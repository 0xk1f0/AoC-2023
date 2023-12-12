use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::max;

fn part_one() -> u16 {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // define a sum
    let mut sum: u16 = 0;

    // iterate over the lines of the file
    for line in reader.lines() {
        let current_line = line.unwrap();
        // invalidity bool for breaking early
        let mut invalid: bool = false;
        // split into parts and remove seperators
        let parts: Vec<String> = current_line.split(' ').map(|m| m.replace(&[':', ';', ','], "")).collect();
        // deifne the current game id
        let id: u16 = parts[1].parse().unwrap();
        // iterate over ever second instance
        for i in (2..parts.len()).step_by(2) {
            let num: u16 = parts[i].parse().unwrap();
            // assume that every instance of i+1 is a color
            // if over threshhold, break early and set invalid
            if num > 12 && parts[i+1] == "red" {
                invalid = true;
                break;
            } else if num > 13 && parts[i+1] == "green" {
                invalid = true;
                break;
            } else if num > 14 && parts[i+1] == "blue" {
                invalid = true;
                break;
            }
        }
        // check for invalidity, else add to sum
        if !invalid {
            sum += id;
        }
    }

    return sum;
}

fn part_two() -> u32 {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // define a sum
    let mut sum: u32 = 0;

    // iterate over the lines of the file
    for line in reader.lines() {
        let current_line = line.unwrap();
        // split into parts and remove seperators
        let parts: Vec<String> = current_line.split(' ').map(|m| m.replace(&[':', ';', ','], "")).collect();
        // define maximum reached values for each color
        let (mut max_r, mut max_g, mut max_b) = (0,0,0);
        // iterate over ever second instance
        for i in (2..parts.len()).step_by(2) {
            let num: u32 = parts[i].parse().unwrap();
            // compare each value to their current max value to check for higher
            if parts[i+1] == "red" {
                max_r = max(max_r, num);
            } else if parts[i+1] == "green" {
                max_g = max(max_g, num);
            } else if parts[i+1] == "blue" {
                max_b = max(max_b, num);
            }
        }
        // get the power and add to sum
        sum += max_r * max_g * max_b;
    }

    return sum;
}

fn main() {
    println!("Part 1> The Sum of all game IDs is: {}", part_one());
    println!("Part 1> The Sum of all powers is: {}", part_two());
}
