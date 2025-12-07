// Solution to day 7 of the Advent of Code challenge

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read the file, storing each row of beamsplitters within a vector, and also
/// returning the start vector.
fn parse_file(name: &str) -> (Vec<Vec<bool>>, HashSet<usize>) {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    let mut beamsplitters: Vec<Vec<bool>> = Vec::new();
    let mut initial_positions: Option<HashSet<usize>> = None;

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            initial_positions = Some(HashSet::from_iter(
                line.unwrap()
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| match c {
                        'S' => Some(i),
                        _ => None,
                    }),
            ))
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
    (beamsplitters, initial_positions.unwrap())
}

fn count_number_of_splittings(
    beamsplitters: &Vec<Vec<bool>>,
    initial_positions: &HashSet<usize>,
) -> u32 {
    let mut positions = initial_positions.clone();
    let mut total = 0;
    // This code would fail if there was adjacent beam splitters or if the beam
    // splitters are on the end rows, however this is not present in the problem
    for row in beamsplitters {
        let mut new_positions = HashSet::new();
        for pos in positions {
            if row[pos] {
                new_positions.insert(pos - 1);
                new_positions.insert(pos + 1);
                total += 1;
            } else {
                new_positions.insert(pos);
            }
        }
        positions = new_positions;
    }
    total
}

fn main() {
    let (beamsplitters, initial_positions) = parse_file("input.txt");

    // Part 1
    let total = count_number_of_splittings(&beamsplitters, &initial_positions);
    println!("Total splitting = {}", total);
}
