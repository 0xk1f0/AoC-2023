use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // all codes
    let mut ascii_codes: Vec<(usize, Vec<usize>)> = Vec::new();
    for line in lines {
        ascii_codes = line
            .split(",")
            .map(|seq| (0, seq.chars().map(|c| c as usize).collect()))
            .collect();
    }
    // get the total sum of loads
    ascii_codes
        .iter_mut()
        .map(|seq| {
            let _ = seq
                .1
                .iter()
                .map(|&code| seq.0 = (((seq.0 + code) * 17) % 256) + 1)
                .count();
            seq.0
        })
        .sum()
}

fn part_two() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // vector for all codes
    let mut ascii: Vec<(usize, Vec<char>)> = Vec::new();
    // boxes hashmap for easy lookup
    let mut boxes: HashMap<usize, Vec<String>> = HashMap::new();
    // collect all items to vector
    for line in lines {
        ascii = line
            .split(",")
            .map(|seq| (0, seq.chars().collect()))
            .collect();
    }
    // perform insertion and deletion for all boxes
    ascii
        .iter_mut()
        .map(|seq| {
            // from a string from the chars
            let str = String::from_iter(seq.1.iter());
            // calculate the sequence hashlabel
            let _ = seq
                .1
                .iter()
                .take_while(|&x| x != &'=' && x != &'-')
                .map(|&code| {
                    seq.0 = ((seq.0 + code as usize) * 17) % 256;
                })
                .count();
            let mut current_box = boxes.get(&seq.0).unwrap_or(&vec![]).clone();
            if seq.1.contains(&'-') {
                // check if box has item
                if let Some(x) = current_box
                    .iter()
                    .position(|x| x.starts_with(&str.split("-").nth(0).unwrap()))
                {
                    // remove if true
                    current_box.remove(x);
                    
                }
                // write back to hashmap
                boxes.insert(seq.0, current_box);
            } else {
                // check if box has item at position
                if let Some(x) = current_box
                    .iter()
                    .position(|x| x.starts_with(&str.split("=").nth(0).unwrap()))
                {
                    // insert new item before old
                    current_box.insert(x, str);
                    // remove old
                    current_box.remove(x + 1);
                } else {
                    // if box doesnt have item, append new
                    current_box.push(str);
                }
                // write to hashmap
                boxes.insert(seq.0, current_box);
            }
        })
        .count();
    // get the focal power sum
    boxes
        .iter_mut()
        .map(|(k, v)| {
            v.iter_mut()
                .enumerate()
                .map(|(i, vec)| {
                    let x = (k + 1) * (i + 1) * vec.split("=").nth(1).unwrap().parse::<usize>().unwrap();
                    x
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    println!("Part 1> The Hash Sum is: {}", part_one());
    println!("Part 2> The Focusing Power is: {}", part_two());
}
