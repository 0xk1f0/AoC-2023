use num::integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // for basic moves
    let mut base_moves: Vec<usize> = Vec::new();
    // define a vector for all cords
    let mut destinations: HashMap<String, (String, String)> = HashMap::new();
    // iterate all lines and extract
    for (i, line) in lines.into_iter().enumerate() {
        // get base moves in first line and skip second
        if i < 2 {
            if line.len() > 0 {
                let chars: Vec<usize> = line
                    .chars()
                    .map(|c| match c {
                        'L' => 0,
                        'R' => 1,
                        _ => 3,
                    })
                    .collect();
                base_moves.extend(chars);
            }
            continue;
        }
        // extract current destination
        let dest = line.split(" ").nth(0).unwrap().to_string();
        // extract right destination
        let left: String = line
            .split(",")
            .nth(0)
            .unwrap()
            .split("(")
            .last()
            .unwrap()
            .to_string();
        // extract right destination
        let right: String = line
            .split(" ")
            .nth(3)
            .unwrap()
            .split(")")
            .nth(0)
            .unwrap()
            .to_string();
        // insert a new destination
        destinations.insert(dest, (left, right));
    }
    // starting value
    let mut value: &String = &String::from("AAA");
    // iterator for walking the moves
    let mut iterator: usize = 0;
    // sum for allup sum of moves
    let mut sum: usize = 0;
    // do this while value is not "ZZZ"
    while value != "ZZZ" {
        // match direction
        match base_moves[iterator] {
            0 => value = &destinations.get(value).unwrap().0,
            1 => value = &destinations.get(value).unwrap().1,
            _ => panic!("'the fuck?"),
        }
        // restart the iterator if we run out of moves
        if iterator == base_moves.len() - 1 {
            iterator = 0;
        } else {
            iterator += 1
        }
        // count a step
        sum += 1;
    }
    // return sum of steps
    return sum;
}

fn part_two() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // for basic moves
    let mut base_moves: Vec<usize> = Vec::new();
    // define a vector for all cords
    let mut destinations: HashMap<String, (String, String)> = HashMap::new();
    // define starting dests
    let mut starting_destinations: Vec<String> = Vec::new();
    // iterate all lines and extract
    for (i, line) in lines.into_iter().enumerate() {
        // get base moves in first line and skip second
        if i < 2 {
            if line.len() > 0 {
                let chars: Vec<usize> = line
                    .chars()
                    .map(|c| match c {
                        'L' => 0,
                        'R' => 1,
                        _ => 3,
                    })
                    .collect();
                base_moves.extend(chars);
            }
            continue;
        }
        // extract current destination
        let dest = line.split(" ").nth(0).unwrap().to_string();
        // extract right destination
        let left: String = line
            .split(",")
            .nth(0)
            .unwrap()
            .split("(")
            .last()
            .unwrap()
            .to_string();
        // extract right destination
        let right: String = line
            .split(" ")
            .nth(3)
            .unwrap()
            .split(")")
            .nth(0)
            .unwrap()
            .to_string();
        // if value ends with "A" we have a starting node
        if dest.ends_with("A") {
            starting_destinations.push(dest.clone());
        }
        // insert a new destination
        destinations.insert(dest, (left, right));
    }
    // capture all individual step values
    let mut all_steps: Vec<usize> = Vec::new();
    for dest in starting_destinations {
        // iterator for walking the moves
        let mut iterator: usize = 0;
        // individual step value
        let mut step: usize = 0;
        // temporary destination for the loop
        let mut temp = dest;
        // match directions until we hit a node that ends with "Z"
        while !temp.ends_with("Z") {
            // match direction
            match base_moves[iterator] {
                0 => temp = destinations.get(&temp).unwrap().0.clone(),
                1 => temp = destinations.get(&temp).unwrap().1.clone(),
                _ => panic!("'the fuck?"),
            }
            // restart the iterator if we run out of moves
            if iterator == base_moves.len() - 1 {
                iterator = 0;
            } else {
                iterator += 1
            }
            // count a step
            step += 1;
        }
        // record steps
        all_steps.push(step);
    }
    // calculate the least common multiplier for all step values
    // @NOTE: it took me a while to find that this is possible
    // one can see from the example values that each starting node begins
    // to loop once it reaches a destination ending in "Z" once
    // That's why this works
    return all_steps.iter().fold(1, |a, b| lcm(a, *b));
}

fn main() {
    println!("Part 1> Needs: {} Moves", part_one());
    println!("Part 2> Needs: {} Moves", part_two());
}
