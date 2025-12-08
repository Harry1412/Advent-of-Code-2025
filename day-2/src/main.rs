// Solution to day 2 of the Advent of Code challenge

use std::{fs::File, io::Read, ops::RangeInclusive};

/// Process the provided file and create a vector of ranges
fn parse_file(name: &str) -> Vec<RangeInclusive<u64>> {
    let mut file = File::open(name).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .split(",")
        .map(|range| {
            let values = range.split_once("-").unwrap();
            let start: u64 = values.0.parse().unwrap();
            let end: u64 = values.1.parse().unwrap();
            start..=end
        })
        .collect()
}

/// Checks if a provided id is invalid, where an invalid id is defined as the
/// first and second halves being identical i.e. of the format XYZXYZ
fn check_if_invalid(id: &u64) -> bool {
    let str_id = id.to_string();
    // Only check ids which are a multiple of 2
    if str_id.len().is_multiple_of(2) {
        let mid = str_id.len() / 2;
        let parts = str_id.split_at(mid);
        if parts.0 == parts.1 {
            return true;
        }
    }
    false
}

/// Sum all invalid ids within the provided ranges, where an invalid id is
/// defined as the first and second halves being identical i.e. of the format
/// XYZXYZ
fn sum_invalid_ids(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges
        .iter()
        .map(|r| r.to_owned().filter(check_if_invalid).sum::<u64>())
        .sum()
}

/// Checks if the n parts of a provided id are all equal. This assumes that the
/// string can be broken into n equal parts
fn check_parts_equal(mut id: String, n: usize) -> bool {
    let size = id.len() / n;
    // Get first n-length part of string
    let parts = id.split_off(size);
    // Then loop through remaining string and check equivalance
    for i in 0..n - 1 {
        if id != parts.get(size * i..size * (i + 1)).unwrap() {
            return false;
        }
    }
    true
}

/// Checks if a provided id is invalid, where an invalid id is defined as having
/// a component repeated any number of times e.g. XYXY, XYXYXY
fn check_if_invalid_2(id: &u64) -> bool {
    let str_id = id.to_string();
    let n_digits = str_id.len();
    // Loop through all possible factors of the number of digits
    for n in 2..=n_digits {
        // Only check ids which are a multiple of n
        if str_id.len().is_multiple_of(n) && check_parts_equal(str_id.clone(), n) {
            return true;
        }
    }
    false
}

/// Sum all invalid ids within the provided ranges, where an invalid id is
/// defined as having a component repeated any number of times e.g. XYXY, XYXYXY
fn sum_invalid_ids_2(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges
        .iter()
        .map(|r| r.to_owned().filter(check_if_invalid_2).sum::<u64>())
        .sum()
}

fn main() {
    let ranges = parse_file("input.txt");

    // Part 1
    let total = sum_invalid_ids(&ranges);
    println!("Invalid id total = {}", total);

    // Part 2
    let total2 = sum_invalid_ids_2(&ranges);
    println!("Invalid id total 2 = {}", total2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let ranges = parse_file("input.txt");
        assert_eq!(sum_invalid_ids(&ranges), 43952536386)
    }

    #[test]
    fn part_2() {
        let ranges = parse_file("input.txt");
        assert_eq!(sum_invalid_ids_2(&ranges), 54486209192)
    }
}
