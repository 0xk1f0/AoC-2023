use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn recurse_pattern(mut s: Vec<char>, sum: &mut usize, numbers: &Vec<Vec<usize>>, run: usize, index: usize, regex: &Regex) {
    if index == s.len() {
        // clone the chars as string
        let string = String::from_iter(s.clone());
        // check for matches
        let matches: Vec<regex::Match<'_>> = regex.find_iter(&string).collect();
        // if matched group count matches number run length
        // and each match has equal length to number run instance
        if matches.len() == numbers[run].len() && matches.iter().enumerate().all(|(i, x)| x.len() == numbers[run][i] ) {
            // add to sum
            *sum += 1;
        }
    } else if s[index] == '?' {
        s[index] = 'X';
        recurse_pattern(s.clone(), sum, numbers, run, index + 1, regex);
        s[index] = '.';
        recurse_pattern(s, sum, numbers, run, index + 1, regex);
    } else {
        recurse_pattern(s, sum, numbers, run, index + 1, regex);
    }
}

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // match regex
    let regex = Regex::new(r"(#|X)+").unwrap();
    // patterns
    let patterns: Vec<Vec<char>> = lines.iter().map(|l| l.split(" ").nth(0).unwrap().chars().collect()).collect();
    let numbers: Vec<Vec<usize>> = lines.iter().map(|l| l.split(" ").nth(1).unwrap().split(",").map(|s| s.parse().unwrap()).collect()).collect();
    // define a total sum
    let mut sum: usize = 0;
    // recurse the hell out of the patterns
    for i in 0..patterns.len() {
        recurse_pattern(patterns[i].to_vec(), &mut sum, &numbers, i, 0, &regex);
    }
    // return the all up sum
    return sum;
}

fn main() {
    println!("Part 1> Number of Possibilites: {}", part_one());
}
