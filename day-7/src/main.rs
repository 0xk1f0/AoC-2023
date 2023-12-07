use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// generic Card
struct Card {
    bid: usize,
    special: usize,
    first: usize,
    second: usize,
    third: usize,
    fourth: usize,
    fifth: usize,
}

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a vector for all bids
    let mut cards: Vec<Card> = Vec::with_capacity(lines.len());
    // iterate all lines and extract
    for line in lines {
        // define a comparer hashmap
        let mut compare: HashMap<String, usize> = HashMap::new();
        // get the bid
        let bid: usize = line.split(" ").last().unwrap().parse().unwrap();
        // get the chars
        let chars: Vec<usize> = line
            .split(" ")
            .nth(0)
            .unwrap()
            .chars()
            .map(|m| {
                // check for duplicates
                if let Some(value) = compare.get(&m.to_string()) {
                    compare.insert(m.to_string(), value + 1);
                } else {
                    compare.insert(m.to_string(), 1);
                }
                // match all values to their strength
                match m.to_string().as_str() {
                    "2" => 1,
                    "3" => 2,
                    "4" => 3,
                    "5" => 4,
                    "6" => 5,
                    "7" => 6,
                    "8" => 7,
                    "9" => 8,
                    "T" => 9,
                    "J" => 10,
                    "Q" => 11,
                    "K" => 12,
                    "A" => 13,
                    _ => 0,
                }
            })
            .collect();
        // clone to vector and sort by size
        let mut values: Vec<usize> = compare.values().cloned().collect();
        values.sort_by(|b, a| a.partial_cmp(b).unwrap());
        // define result and instance counter
        let mut result: usize = 0;
        let mut instance = 1;
        // loop value and match result based on actual value
        for value in &values {
            result = match value {
                5 => 7, // match as highest
                4 => 6, // match as second highest and so on ....
                3 => {
                    let mut temp = 4;
                    for v in values.split_at(instance).1 {
                        match v {
                            2 => temp = 5,
                            _ => continue,
                        };
                    }
                    temp
                }
                2 => {
                    let mut temp = 2;
                    for v in values.split_at(instance).1 {
                        match v {
                            2 => temp = 3,
                            _ => continue,
                        };
                    }
                    temp
                }
                _ => 1,
            };
            if result == 1 {
                instance += 1;
            } else {
                break;
            }
        }
        // push to all cards
        cards.push(Card {
            bid: bid,
            special: result,
            first: chars[0],
            second: chars[1],
            third: chars[2],
            fourth: chars[3],
            fifth: chars[4],
        })
    }
    // sort by special and values descending
    cards.sort_unstable_by_key(|item| {
        (
            item.special,
            item.first,
            item.second,
            item.third,
            item.fourth,
            item.fifth,
        )
    });
    // iterate and multiply
    let mut sum: usize = 0;
    for (i, card) in cards.into_iter().enumerate() {
        sum += card.bid * (i + 1);
    }
    // return the sum
    return sum;
}

fn part_two() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a vector for all bids
    let mut cards: Vec<Card> = Vec::with_capacity(lines.len());
    // iterate all lines and extract
    for line in lines {
        // define a comparer hashmap
        let mut compare: HashMap<String, usize> = HashMap::new();
        // get the bid
        let bid: usize = line.split(" ").last().unwrap().parse().unwrap();
        // get the chars
        let chars: Vec<usize> = line
            .split(" ")
            .nth(0)
            .unwrap()
            .chars()
            .map(|m| {
                // check for duplicates
                if let Some(value) = compare.get(&m.to_string()) {
                    compare.insert(m.to_string(), value + 1);
                } else {
                    compare.insert(m.to_string(), 1);
                }
                // match all values to their strength
                match m.to_string().as_str() {
                    "J" => 1,
                    "2" => 2,
                    "3" => 3,
                    "4" => 4,
                    "5" => 5,
                    "6" => 6,
                    "7" => 7,
                    "8" => 8,
                    "9" => 9,
                    "T" => 10,
                    "Q" => 11,
                    "K" => 12,
                    "A" => 13,
                    _ => 0,
                }
            })
            .collect();
        // extract jokers
        let jokers: usize = compare.get("J").unwrap_or(&0).to_owned();
        // clone to vector and sort by size
        let mut values: Vec<usize> = compare.values().cloned().collect();
        values.sort_by(|b, a| a.partial_cmp(b).unwrap());
        // define result and instance counter
        let mut result: usize = 0;
        let mut instance = 1;
        // loop value and match result based on actual value and jokers
        for value in &values {
            result = match value {
                5 => 7,
                4 => {
                    if jokers == 4 || jokers == 1 {
                        7
                    } else {
                        6
                    }
                }
                3 => {
                    let mut temp = 4;
                    for v in values.split_at(instance).1 {
                        match v {
                            2 => {
                                if jokers == 3 {
                                    temp = 7
                                } else if jokers == 2 {
                                    temp = 7
                                } else {
                                    temp = 5
                                }
                            }
                            _ => continue,
                        };
                    }
                    if jokers == 3 && temp < 6 {
                        6
                    } else if jokers == 1 && temp < 6 {
                        6
                    } else {
                        temp
                    }
                }
                2 => {
                    let mut temp = 2;
                    for v in values.split_at(instance).1 {
                        match v {
                            2 => {
                                if jokers == 2 {
                                    temp = 6
                                } else if jokers == 1 {
                                    temp = 5
                                } else {
                                    temp = 3
                                }
                            }
                            _ => continue,
                        };
                    }
                    if jokers == 2 && temp < 4 {
                        4
                    } else if jokers == 1 && temp < 4 {
                        4
                    } else {
                        temp
                    }
                }
                _ => 1,
            };
            if result == 1 {
                instance += 1;
            } else {
                break;
            }
        }
        if jokers == 1 && result == 1 {
            result = 2
        }

        let end_card = Card {
            bid: bid,
            special: result,
            first: chars[0],
            second: chars[1],
            third: chars[2],
            fourth: chars[3],
            fifth: chars[4],
        };

        cards.push(end_card)
    }
    // sort by special and values descending
    cards.sort_unstable_by_key(|item| {
        (
            item.special,
            item.first,
            item.second,
            item.third,
            item.fourth,
            item.fifth,
        )
    });
    // iterate and multiply
    let mut sum: usize = 0;
    for (i, card) in cards.into_iter().enumerate() {
        sum += card.bid * (i + 1);
    }
    // return the sum
    return sum;
}

fn main() {
    println!("Part 1> The sum of all bids is: {}", part_one());
    println!("Part 2> The sum of all bids is: {}", part_two());
}
