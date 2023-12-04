use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;

fn part_one() -> u32 {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // define a sum
    let mut sum: u32 = 0;
    // regex for digits
    let num_regex = Regex::new(r"\d+").unwrap();

    // iterate over the lines of the file
    for line in reader.lines() {
        let current_line = line.unwrap();
        // game multiplier
        let mut multiplier: u32 = 0;
        // define winning numbers
        let win_numbers: &str = current_line.split('|').rev().last().unwrap().split(":").last().unwrap();
        // define winning numbers
        let actual_numbers: &str = current_line.split(':').last().unwrap().split("|").last().unwrap();
        // loop with regex
        let _: Vec<_> = num_regex.find_iter(win_numbers).map(|n| {
            let _: Vec<_> = num_regex.find_iter(actual_numbers).map(|d| {
                if d.as_str() == n.as_str() {
                    if multiplier == 0 {
                        multiplier = 1;
                    } else {
                        multiplier *= 2;
                    }
                }
            }).collect();
        }).collect();

        sum += multiplier;
    }

    return sum;
}

fn part_two() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // regex for digits
    let num_regex = Regex::new(r"\d+").unwrap();
    // vector for all games
    let mut full_cards: HashMap<usize, usize> = HashMap::new();

    // iterate over the lines of the file
    for line in reader.lines() {
        let current_line = line.unwrap();
        // all numbers that match
        let mut all_matches: usize = 0;
        // deifne the current game id
        let card_id: usize = current_line.split(':').rev().last().unwrap().split(" ").last().unwrap().parse().unwrap();
        // define winning numbers
        let win_numbers: &str = current_line.split('|').rev().last().unwrap().split(":").last().unwrap();
        // define winning numbers
        let actual_numbers: &str = current_line.split(':').last().unwrap().split("|").last().unwrap();
        // loop with regex
        let _: Vec<_> = num_regex.find_iter(win_numbers).map(|n| {
            let _: Vec<_> = num_regex.find_iter(actual_numbers).map(|d| {
                if d.as_str() == n.as_str() {
                    all_matches += 1;
                }
            }).collect();
        }).collect();

        // this cards value
        // make sure to iterate by one
        let current_value: usize;
        if let Some(value) = full_cards.get(&card_id) {
            current_value = value.to_owned() + 1;
        } else {
            current_value = 1;
        }

        // check what we add
        // also make sure to add one to matches so we go through all
        for m in card_id..card_id+all_matches+1 {
            let current: usize;
            // if found is value, else is zero
            if let Some(value) = full_cards.get(&m) {
                current = value.to_owned();
            } else {
                current = 0;
            }
            // should current be equal id, iterate by one
            // else do current iterator value plus current value
            if m == card_id {
                full_cards.insert(m, current + 1);
            } else {
                full_cards.insert(m, current + current_value );
            }
        }
    }
    // sum all the values in hashmap
    let full_sum: usize = full_cards.values().sum();
    full_sum
}

fn main() {
    println!("Part 1> The Sum of all multipliers is: {}", part_one());
    println!("Part 2> The Sum of all scratchcards is: {}", part_two());
}
