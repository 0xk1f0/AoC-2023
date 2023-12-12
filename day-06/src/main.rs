use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn part_one() -> u32 {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // number regex
    let numbers_match = Regex::new(r"\d+").unwrap();
    // get all lines
    let mut lines = reader.lines();
    // get all times
    let times: Vec<u32> = numbers_match.find_iter(&lines.next().unwrap().unwrap()).map(|m| m.as_str().parse().unwrap()).collect();
    // get all distances
    let distances: Vec<u32> = numbers_match.find_iter(&lines.next().unwrap().unwrap()).map(|m| m.as_str().parse().unwrap()).collect();
    // define a result vector
    let mut results: Vec<u32> = Vec::with_capacity(times.len());
    // iterate all races
    let mut iteration = 0;
    for race_time in times {
        let mut possibilities: u32 = 0;
        for time in 0..race_time {
            if (time * (race_time - time)) > distances[iteration] {
                // if time is lower, match
                possibilities += 1;
            }
        }
        // add to result vector and iterate by one
        results.push(possibilities);
        iteration += 1;
    }
    // multiply all to get factor
    let mut factor: u32 = 1;
    for result in results {
        factor *= result;
    }
    // return the factor
    return factor;
}

fn part_two() -> u64 {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // number regex
    let numbers_match = Regex::new(r"\d+").unwrap();
    // get all lines
    let lines: Vec<_> = reader.lines().collect();
    let times: Vec<&str> = numbers_match.find_iter(&lines[0].as_ref().unwrap()).map(|m| m.as_str()).collect();
    // get time
    let distances: Vec<&str> = numbers_match.find_iter(&lines[1].as_ref().unwrap()).map(|m| m.as_str()).collect();
    // set time and distance with join
    let time = times.join("").parse::<u64>().unwrap();
    let distance = distances.join("").parse::<u64>().unwrap();
    // iterate through all possible times
    let mut possibilities: u64 = 0;
    for t in 1..time {
        if (t * (time - t)) > distance {
            // if time is lower, match
            possibilities += 1;
        }
    }
    // return number of possibilities
    return possibilities;
}

fn main() {
    println!("Part 1> The possiblity factor is: {}", part_one());
    println!("Part 2> The possiblities are: {}", part_two());
}
