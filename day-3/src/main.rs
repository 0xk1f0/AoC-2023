use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> u32 {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);

    // new regex for digits
    let regex_digits = Regex::new(r"\d+").unwrap();
    // new regex for anything but digits and punctuations
    let regex_symbols = Regex::new(r"[^\d\.]").unwrap();
    // vector for matches
    let mut numbers: Vec<u32> = Vec::new();
    // previous line
    let mut prev_line: String = String::from("");

    // consume the lines
    let lines: Vec<_> = reader.lines().collect();
    // get the max length
    let max_length = lines.len();
    // iterator
    let mut iter = lines.into_iter().peekable();

    // iterate over the lines of the file
    for _ in 0..max_length {
        // set current and next line using peek
        let current_line = iter.next().unwrap().unwrap();
        let next_line: String;
        if let Some(x) = iter.peek() {
            next_line = x.as_ref().unwrap().to_owned();
        } else {
            next_line = String::from("");
        }
        // find all occurrences of digits
        let _: Vec<_> = regex_digits.find_iter(&current_line).map(|d| {
            // check for range matches in previous, current and next line
            let _: Vec<_> = regex_symbols.find_iter(&prev_line).map(|s| {
                if s.start() >= d.start().saturating_sub(1) && s.end() <= d.end() + 1 {
                    numbers.push(d.as_str().parse().unwrap());
                }
            }).collect();
            let _: Vec<_> = regex_symbols.find_iter(&current_line).map(|s| {
                if s.end() == d.start() || s.start() == d.end() {
                    numbers.push(d.as_str().parse().unwrap());
                }
            }).collect();
            let _: Vec<_> = regex_symbols.find_iter(&next_line).map(|s| {
                if s.start() >= d.start().saturating_sub(1) && s.end() <= d.end() + 1 {
                    numbers.push(d.as_str().parse().unwrap());
                }
            }).collect();
        }).collect();
        // dont forget to set current to next
        prev_line = current_line;
    }
    // sum all numbers of the vector
    return numbers.iter().sum();
}

fn part_two() -> u32 {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);

    // new regex for digits
    let regex_digits = Regex::new(r"\d+").unwrap();
    // new regex for a gear
    let regex_gear = Regex::new(r"\*").unwrap();
    // vector for matches
    let mut numbers: Vec<u32> = Vec::new();
    // previous line
    let mut prev_line: String = String::from("");

    // consume the lines
    let lines: Vec<_> = reader.lines().collect();
    // get the max length
    let max_length = lines.len();
    // iterator
    let mut iter = lines.into_iter().peekable();

    // iterate over the lines of the file
    for _ in 0..max_length {
        // set current and next line using peek
        let current_line = iter.next().unwrap().unwrap();
        let next_line: String;
        if let Some(x) = iter.peek() {
            next_line = x.as_ref().unwrap().to_owned();
        } else {
            next_line = String::from("");
        }
        // find all occurrences of gears (*)
        let _: Vec<_> = regex_gear.find_iter(&current_line).map(|g| {
            // define a new vector for ratio
            let mut gear_ratio: Vec<u32> = Vec::with_capacity(2);
            // check for range matches in previous, current and next line
            let _: Vec<_> = regex_digits.find_iter(&prev_line).map(|d| {
                if gear_ratio.len() < 2 && d.end() >= g.start() && d.start() <= g.end() {
                    gear_ratio.push(d.as_str().parse().unwrap());
                }
            }).collect();
            let _: Vec<_> = regex_digits.find_iter(&current_line).map(|d| {
                if gear_ratio.len() < 2 && d.end() == g.start() || d.start() == g.end() {
                    gear_ratio.push(d.as_str().parse().unwrap());
                }
            }).collect();
            let _: Vec<_> = regex_digits.find_iter(&next_line).map(|d| {
                if gear_ratio.len() < 2 && d.end() >= g.start() && d.start() <= g.end() {
                    gear_ratio.push(d.as_str().parse().unwrap());
                }
            }).collect();
            // if we have exactly two numbers as ratio we can multiply and add
            if gear_ratio.len() == 2 {
                numbers.push(gear_ratio[0] * gear_ratio[1])
            }
        }).collect();
        // dont forget to set current to next
        prev_line = current_line;
    }
    // sum all numbers of the vector
    return numbers.iter().sum();
}

fn main() {
    println!("Part 1> The Sum of all numbers is: {}", part_one());
    println!("Part 2> The Sum of all ratios is: {}", part_two());
}
