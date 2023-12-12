use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> i64 {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // initial seeds
    let mut inital_seeds: Vec<i64> = Vec::new();
    // categories with vector of (offset, start, end)
    let mut categories: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    // bool for making new entries
    let mut is_collecting: bool = false;

    // iterate over the lines of the file
    for line in reader.lines().enumerate() {
        let current_line = line.1.unwrap();
        // set inital seeds
        if line.0 == 0 {
            inital_seeds = current_line
                .split(" ")
                .skip(1)
                .map(|m| m.parse().unwrap())
                .collect();
        }

        // set new category and collecting to true
        if current_line.contains("map") {
            is_collecting = true;
            categories.push(Vec::new());
            continue;
        }

        // set collection to false
        if current_line.len() == 0 {
            is_collecting = false;
            continue;
        }

        // check for collection
        if is_collecting {
            // parse all numbers to a vector
            let numbers: Vec<i64> = current_line
                .split(" ")
                .map(|m| m.parse().unwrap())
                .collect();
            let mut this_category = categories.last().unwrap().clone();
            // insert new values
            this_category.push((numbers[1] - numbers[0], numbers[1], numbers[1] + numbers[2]));
            // pop and write back to vector
            categories.pop();
            categories.push(this_category.to_owned());
        }
    }

    // initially set min_val
    let mut min_result = inital_seeds[0];

    // loop all seeds
    for seed in inital_seeds {
        let mut current_seed = seed;
        for category in &categories {
            for entry in category {
                // if seed in special range, set accodingly
                // break immediately afterwards, since cat is finished
                if current_seed >= entry.1 && current_seed <= entry.2 {
                    current_seed = current_seed - entry.0;
                    break;
                }
            }
        }
        // compare to current with min to find lowest
        min_result = min_result.min(current_seed);
    }
    // return min value
    return min_result;
}

fn part_two() -> i64 {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader
    let reader = BufReader::new(input);
    // initial seeds
    let mut inital_seeds: Vec<(i64, i64)> = Vec::new();
    // categories with vector of (offset, start, end)
    let mut categories: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    // bool for making new entries
    let mut is_collecting: bool = false;

    // iterate over the lines of the file
    for line in reader.lines().enumerate() {
        let current_line = line.1.unwrap();
        // set inital seeds
        if line.0 == 0 {
            let splits: Vec<i64> = current_line
                .split(" ")
                .skip(1)
                .map(|m| m.parse().unwrap())
                .collect();
            let mut all_seeds = splits.iter();
            for _i in 0..all_seeds.len() / 2 {
                inital_seeds.push((
                    all_seeds.next().unwrap().to_owned(),
                    all_seeds.next().unwrap().to_owned(),
                ));
            }
        }

        // set new category and collecting to true
        if current_line.contains("map") {
            is_collecting = true;
            categories.push(Vec::new());
            continue;
        }

        // set collection to false
        if current_line.len() == 0 {
            is_collecting = false;
            continue;
        }

        // check for collection
        if is_collecting {
            // parse all numbers to a vector
            let numbers: Vec<i64> = current_line
                .split(" ")
                .map(|m| m.parse().unwrap())
                .collect();
            let mut this_category = categories.last().unwrap().clone();
            // insert new values
            this_category.push((numbers[1] - numbers[0], numbers[1], numbers[1] + numbers[2]));
            // pop and write back to vector
            categories.pop();
            categories.push(this_category.to_owned());
        }
    }

    // initially set min_val
    let mut min_result = inital_seeds[0].0;

    // loop all seed ranges
    // @TODO: THIS TAKES VERY LONG
    // I think an optimal solution would be parallel processing
    // since we can never exit early because we can't determine nested seed values
    for seed_range in inital_seeds {
        // loop all sub seeds in seed range
        for sub_seed in seed_range.0..seed_range.0 + seed_range.1 {
            let mut current_seed = sub_seed;
            for category in &categories {
                for entry in category {
                    // if seed in special range, set accodingly
                    // break immediately afterwards, since cat is finished
                    if current_seed >= entry.1 && current_seed <= entry.2 {
                        current_seed = current_seed - entry.0;
                        break;
                    }
                }
            }
            // compare to current with min to find lowest
            min_result = min_result.min(current_seed);
        }
    }
    // return min value
    return min_result;
}

fn main() {
    println!("Part 1> The lowest location number is: {}", part_one());
    println!("Part 2> The lowest location number is: {}", part_two());
}
