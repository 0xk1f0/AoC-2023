use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    // open our input file
    let input = File::open("input.txt").unwrap();
    // define a new buffer reader and get lines
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    // define a matrix
    let mut matrix: Vec<Vec<((usize, usize), (usize, usize))>> = Vec::with_capacity(lines.len());
    // visited nodes
    let mut visited: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    // iterate lines and add to matrix
    for (row, line) in lines.iter().enumerate() {
        matrix.push(
            line.chars()
                .enumerate()
                .map(|(col, c)| ((col, row), (c.to_digit(10).unwrap() as usize, 99999)))
                .collect(),
        );
    }

    // define a start cursor
    let mut cursor = ((0,0), (0,0));
    // define the target
    let target =
        matrix[matrix.len() - 1][matrix[matrix.len() - 1].len() - 1].clone();
    // keep track of previous location
    let mut previous = cursor.clone();

    let mut loop_protect = 0;
    while cursor != target && loop_protect < 1000 {
        loop_protect += 1;
        let mut neighbours: Vec<((usize, usize), (usize, usize))> = Vec::new();

        if cursor.0.0 - 1 < 100000 {
            neighbours.push(matrix[cursor.0.1][cursor.0.0 - 1]);
        }

        if cursor.0.1 - 1 < 100000 {
            neighbours.push(matrix[cursor.0.1 - 1][cursor.0.0]);
        }

        if cursor.0.0 + 1 < matrix[0].len() {
            neighbours.push(matrix[cursor.0.1][cursor.0.0 + 1]);
        }

        if cursor.0.1 + 1 < matrix.len() {
            neighbours.push(matrix[cursor.0.1 + 1][cursor.0.0]);
        }

        neighbours.retain(|&x| x.0 != previous.0 && !visited.contains(&x));
        // weight
        let mut node_weight: ((usize, usize), usize) = ((0, 0) , 99999);
        for (i, &nb) in neighbours.iter().enumerate() {
            let mut new_weight = cursor.1.1 + nb.1.0;
            if new_weight < nb.1.1 {
                println!("Readjusting {:?} to {}", nb, new_weight);
                matrix[nb.0.1][nb.0.0] = ((nb.0.0,nb.0.1),(nb.1.0, new_weight));
            } else {
                new_weight = nb.1.1;
            }
            if node_weight.1 > new_weight {
                node_weight = ( (neighbours[i].0.0, neighbours[i].0.1), new_weight)
            }
        }
        previous = cursor;
        visited.insert(cursor);
        cursor = matrix[node_weight.0.1][node_weight.0.0];

        let mut temp = matrix.clone();
        temp[cursor.0.1][cursor.0.0] = ((0, 0), (0, 0));
        let temp2: Vec<Vec<((usize, usize), (char, usize))>> = temp.iter().map(|i| i.iter().map(|v| {
            if v.1.0 != 0 {
                ((v.0.0, v.0.1), ('.', v.1.1))
            } else {
                ((v.0.0, v.0.1), ('X', v.1.1))
            }
        }).collect()).collect();
        temp2.iter().map(|r| println!("{}", String::from_iter(r.iter().map(|e| e.1.0)))).count();
    }

    cursor.1.1
}

fn main() {
    println!("Part 1> The Shortest Path cost is: {}", part_one());
}
