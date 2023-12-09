use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> isize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // a sum for the histories
    let mut sum: isize = 0;
    // iterate each line
    for line in lines {
        // extract all numbers
        let mut numbers: Vec<isize> = line.split(" ").map(|n| n.parse().unwrap()).collect();
        // define anew vector for all diffs
        let mut diffs: Vec<Vec<isize>> = Vec::new();
        // clone the current numbers into the diff vector
        diffs.push(numbers.clone());
        // while the last diff is not all zeros
        while !diffs.iter().last().unwrap().iter().all(|&n| n == 0) {
            numbers = numbers.windows(2).map(|n| n[1] - n[0]).collect();
            diffs.push(numbers.clone())
        }
        // reverse the diff vector to go bottom up
        diffs.reverse();

        for i in 0..diffs.len()-1 {
            // add the last value of the current vector in diffs
            // to the last value of the next vector in diffs and append the new value
            let new = diffs[i+1].iter().last().unwrap() + diffs[i].iter().last().unwrap();
            diffs[i+1].push(new);
        }
        // calculate the summ of all last vector elements of diffs
        sum += diffs.last().unwrap().last().unwrap();
    }
    // return the all up sum
    return sum;
}

fn part_two() -> isize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // a sum for the previous histories
    let mut sum: isize = 0;
    // iterate each line
    for line in lines {
        // extract all numbers
        let mut numbers: Vec<isize> = line.split(" ").map(|n| n.parse().unwrap()).collect();
        // define anew vector for all diffs
        let mut diffs: Vec<Vec<isize>> = Vec::new();
        // clone the current numbers into the diff vector
        diffs.push(numbers.clone());
        // while the last diff is not all zeros
        while !diffs.iter().last().unwrap().iter().all(|&n| n == 0) {
            numbers = numbers.windows(2).map(|n| n[1] - n[0]).collect();
            diffs.push(numbers.clone())
        }
        // reverse the diff vector to go bottom up
        diffs.reverse();

        for i in 0..diffs.len()-1 {
            // subtract the first value of the current vector in diffs
            // from the first value of the next vector in diffs and prepend the new value
            let new = diffs[i+1].iter().nth(0).unwrap() - diffs[i].iter().nth(0).unwrap();
            diffs[i+1].insert(0, new);
        }
        // calculate the sum of all first vector elements of diffs
        sum += diffs.last().unwrap().iter().nth(0).unwrap();
    }
    // return the all up sum
    return sum;
}

fn main() {
    println!("Part 1> The Extrapolated Values Sum is: {}", part_one());
    println!("Part 2> The Extrapolated Values Sum is: {}", part_two());
}
