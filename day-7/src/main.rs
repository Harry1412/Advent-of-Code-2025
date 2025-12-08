// Solution to day 7 of the Advent of Code challenge

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;

/// Read the file, storing each row of beamsplitters within a vector, and also
/// returning the start positions as a set
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
            beamsplitters.push(line.unwrap().chars().map(|c| matches!(c, '^')).collect());
        }
    }
    (beamsplitters, initial_positions.unwrap())
}

/// From a set of beams and rows of beam splitters, calculate the new beam
/// splitter positions after each row and track the new of splittings which
/// occur
fn count_number_of_splittings(
    beamsplitters: &Vec<Vec<bool>>,
    initial_positions: &HashSet<usize>,
) -> u32 {
    let mut positions = initial_positions.clone();
    let mut total = 0;
    for row in beamsplitters {
        let mut new_timelines = HashSet::new();
        for pos in positions {
            if row[pos] {
                new_timelines.extend([pos - 1, pos + 1]);
                total += 1;
            } else {
                new_timelines.insert(pos);
            }
        }
        positions = new_timelines;
    }
    total
}

/// Takes a hashmap and modify the existing value if the key exists, otherwise
/// adds the key/value to the map.
fn add_to_or_update_hashmap<K: Eq + Hash, V: AddAssign + Copy>(
    map: &mut HashMap<K, V>,
    key: K,
    value: V,
) {
    map.entry(key).and_modify(|v| *v += value).or_insert(value);
}

/// Counts the number of timelines created by beams passing through a number of
/// splitters from a set of starting positions. A dictionary is used to count
/// the number of ways a position is reached by a path after each iteration.
fn count_number_of_timelines(
    beamsplitters: &Vec<Vec<bool>>,
    initial_positions: &HashSet<usize>,
) -> u64 {
    let mut timelines: HashMap<usize, u64> = initial_positions.iter().map(|k| (*k, 1)).collect();
    for row in beamsplitters {
        let mut new_timelines = HashMap::new();
        for (pos, count) in timelines {
            if row[pos] {
                add_to_or_update_hashmap(&mut new_timelines, pos - 1, count);
                add_to_or_update_hashmap(&mut new_timelines, pos + 1, count);
            } else {
                add_to_or_update_hashmap(&mut new_timelines, pos, count);
            }
        }
        timelines = new_timelines;
    }
    timelines.values().sum()
}

fn main() {
    let (beamsplitters, initial_positions) = parse_file("input.txt");

    // Part 1
    let total = count_number_of_splittings(&beamsplitters, &initial_positions);
    println!("Total splitting = {}", total);

    // Part 2
    let total = count_number_of_timelines(&beamsplitters, &initial_positions);
    println!("Total splitting = {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let (beamsplitters, initial_positions) = parse_file("input.txt");
        assert_eq!(
            count_number_of_splittings(&beamsplitters, &initial_positions),
            1633
        )
    }

    #[test]
    fn part_2() {
        let (beamsplitters, initial_positions) = parse_file("input.txt");
        assert_eq!(
            count_number_of_timelines(&beamsplitters, &initial_positions),
            34339203133559
        )
    }
}
