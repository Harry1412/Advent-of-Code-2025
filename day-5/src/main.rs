// Solution to day 5 of the Advent of Code challenge

use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

/// Reads the specified value and generates the required id ranges and ids.
fn parse_file(name: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    let mut split_loc = 0;
    for (i, line) in reader.lines().enumerate() {
        let line_data = line.unwrap();
        if line_data.is_empty() {
            split_loc = i;
        } else {
            data.push(line_data);
        }
    }
    let ids: Vec<u64> = data
        .split_off(split_loc)
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let ranges = data
        .iter()
        .map(|x| {
            let range = x.split_once("-").unwrap();
            let start = range.0.parse().unwrap();
            let end = range.1.parse().unwrap();
            start..=end
        })
        .collect();

    (ranges, ids)
}

/// Checks if a set of ranges contains a provided id.
fn ranges_contain_id(ranges: &Vec<RangeInclusive<u64>>, id: &u64) -> bool {
    for r in ranges {
        if r.contains(id) {
            return true;
        }
    }
    false
}

/// Sums the number of ids from the provided vector which exist with the set of
/// valud ranges.
fn count_id_in_range(ranges: &Vec<RangeInclusive<u64>>, ids: &Vec<u64>) -> u64 {
    ids.iter()
        .map(|x| match ranges_contain_id(ranges, x) {
            true => 1_u64,
            false => 0,
        })
        .sum()
}

/// Takes a vector of ranges and simplifies then by merging overlapping/adjacent
/// ranges
fn merge_ranges(ranges: &Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut merged_ranges = vec![sorted_ranges[0].clone()];
    for range in sorted_ranges[1..].iter() {
        let previous_range = merged_ranges.last().unwrap();
        if *range.start() <= previous_range.end() + 1 {
            let new_range = *previous_range.start()..=*max(previous_range.end(), range.end());
            let end_loc = merged_ranges.len() - 1;
            merged_ranges[end_loc] = new_range;
        } else {
            merged_ranges.push(range.clone());
        }
    }
    merged_ranges
}

/// Sum the number of valid ids within the provided vector of ranges
fn sum_ids_in_range(ranges: &Vec<RangeInclusive<u64>>) -> u64 {
    ranges.iter().map(|x| x.end() - x.start() + 1).sum()
}

fn main() {
    let (ranges, ids) = parse_file("input.txt");
    // Remove overlapping ranges now as this also benefits part 1
    let simplified_ranges = merge_ranges(&ranges);

    // Part 1
    let total = count_id_in_range(&simplified_ranges, &ids);
    println!("Fresh ingredients = {}", total);

    // Part 2
    let total = sum_ids_in_range(&simplified_ranges);
    println!("Number of valid ids = {total}");
}
