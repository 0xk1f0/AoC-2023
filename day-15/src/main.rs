use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // all codes
    let mut ascii_codes: Vec<(usize, Vec<usize>)> = Vec::new();
    for line in lines {
        ascii_codes = line.split(",").map(|seq| {
            (0, seq.chars().map(|c| c as usize).collect())
        }).collect();
    }
    // get the total sum of loads
    ascii_codes.iter_mut().map(|seq| {
        let _ = seq.1.iter().map(|&code| {
            seq.0 = ((seq.0 + code) * 17) % 256
        }).count();
        seq.0
    }).sum()
}

fn part_two() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // all codes
    let mut ascii: Vec<(usize, Vec<char>)> = Vec::new();
    // boxes
    let mut boxes: HashMap<usize,Vec<String>> = HashMap::new();
    for line in lines {
        ascii = line.split(",").map(|seq| {
            (0, seq.chars().collect())
        }).collect();
    }
    // get the total sum of loads
    ascii.iter_mut().map(|seq| {
        let str = String::from_iter(seq.1.iter());
        if seq.1.contains(&'-') {
            boxes.iter_mut().map(|boxy| {
                if let Some(x) = boxy.1.iter().position(|x| x.starts_with(&str.split("-").nth(0).unwrap())) {
                    boxy.1.remove(x);
                }
            }).count();
        } else {
            let _ = seq.1.iter().take_while(|&x| x != &'=').map(|&code| {
                seq.0 = ((seq.0 + code as usize) * 17) % 256;
            }).count();
            boxes.iter_mut().map(|boxy| {
                if let Some(x) = boxy.1.iter().position(|x| x.starts_with(&str.split("=").nth(0).unwrap())) {
                    boxy.1.remove(x);
                }
            }).count();
            let mut current_box = boxes.get(&seq.0).unwrap_or(&vec![]).clone();
            current_box.insert(0, str);
            boxes.insert(seq.0, current_box);
        }
        println!("{boxes:?}");
    }).count();

    boxes.iter_mut().map(|(k, v)| {
        let iterator: usize = 1;
        let sum: usize = v.iter_mut().map(|vec| {
            let x = (k + 1) * iterator * vec.split("=").nth(1).unwrap().parse::<usize>().unwrap();
            println!("{} - {}", vec, vec.split("=").nth(1).unwrap().parse::<usize>().unwrap());
            x
        }).sum::<usize>();

        sum
    }).sum()
}

fn main() {
    println!("Part 1> The Hash Sum is: {}", part_one());
    println!("Part 2> The Focusing Power is: {}", part_two());
}