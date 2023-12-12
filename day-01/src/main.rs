use aho_corasick::AhoCorasick;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> u32 {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // a regex that matches only digits
    let regex = Regex::new(r"\d").unwrap();
    // define a sum
    let mut sum: u32 = 0;

    // iterate over the lines of the file
    for line in reader.lines() {
        let current_line = line.unwrap();

        // use find_iter for all occurence of a digit
        let matched_vector: Vec<&str> =
            regex.find_iter(&current_line).map(|m| m.as_str()).collect();

        // define first and last char from the matched digits
        let first_char = matched_vector.first().unwrap_or(&"0");
        let last_char = matched_vector.last().unwrap_or(&"0");

        // combine both char_nums and parse to u32
        let number: u32 = format!("{}{}", first_char, last_char).parse().unwrap();

        // add to sum
        sum = sum + number;
    }

    return sum;
}

fn part_two() -> u32 {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // create a new ac pattern
    let ac = AhoCorasick::new(&[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ])
    .unwrap();
    // define a sum
    let mut sum: u32 = 0;

    // iterate over the lines of the file
    for line in reader.lines() {
        let current_line = line.unwrap();

        // use find_iter for all occurence of a digit and written digit
        let matched_vector: Vec<&str> = ac
            .find_overlapping_iter(&current_line)
            .map(|m| match &current_line[m.start()..m.end()] {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => &current_line[m.start()..m.end()],
            })
            .collect();

        // define first and last char from the matched digits
        let first_char = matched_vector.first().unwrap_or(&"0");
        let last_char = matched_vector.last().unwrap_or(&"0");

        // combine both char_nums and parse to u32
        let number: u32 = format!("{}{}", first_char, last_char).parse().unwrap();

        // add to sum
        sum = sum + number;
    }

    return sum;
}

fn main() {
    println!("Part 1> The Sum of all numbers is: {}", part_one());
    println!("Part 2> The Sum of all numbers is: {}", part_two());
}
