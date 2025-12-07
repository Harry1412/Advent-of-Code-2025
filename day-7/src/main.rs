// Solution to day 7 of the Advent of Code challenge

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read the file, storing each row of beamsplitters within a vector, and also
/// returning the start vector.
fn parse_file(name: &str) -> (Vec<Vec<bool>>, Vec<bool>) {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    let mut beamsplitters: Vec<Vec<bool>> = Vec::new();
    let mut initial_condition: Option<Vec<bool>> = None;

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            initial_condition = Some(
                line.unwrap()
                    .chars()
                    .map(|c| match c {
                        'S' => true,
                        _ => false,
                    })
                    .collect(),
            )
        } else if i % 2 == 0 {
            beamsplitters.push(
                line.unwrap()
                    .chars()
                    .map(|c| match c {
                        '^' => true,
                        _ => false,
                    })
                    .collect(),
            );
        }
    }
    (beamsplitters, initial_condition.unwrap())
}

fn count_number_of_splittings(
    beamsplitters: &Vec<Vec<bool>>,
    initial_condition: &Vec<bool>,
) -> u32 {
    let mut condition = initial_condition.clone();
    let mut total = 0;
    // This code would fail if there was adjacent beam splitters or if the beam
    // splitters are on the end rows, however this is not present in the problem
    for row in beamsplitters {
        for i in 0..condition.len() {
            if condition[i] && row[i] {
                condition[i - 1] = true;
                condition[i] = false;
                condition[i + 1] = true;
                total += 1;
            }
        }
    }
    total
}

fn main() {
    let (beamsplitters, initial_condition) = parse_file("input.txt");

    // Part 1
    let total = count_number_of_splittings(&beamsplitters, &initial_condition);
    println!("Total splitting = {}", total);
}
