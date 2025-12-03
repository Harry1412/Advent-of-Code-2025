// Solution to day 3 of the Advent of Code challenge

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read the input file and add each line to a vector.
fn parse_file(name: &str) -> Vec<String> {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    let mut banks = Vec::new();

    for line in reader.lines() {
        banks.push(line.unwrap());
    }
    banks
}

/// Get the maximum joltage from a bank of batteries, where the total is
/// comprised of two successive (but not necessarily adjacent) joltages from the
/// bank. For example, with joltage ratings 3978, the maximum would be 98.
fn get_joltage_1(ratings: &String) -> u32 {
    let r_vec: Vec<u32> = ratings.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let (loc, j1) = r_vec
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_i, x)| x)
        .unwrap();

    if loc < r_vec.len() - 1 {
        let new_vec = r_vec.get((loc + 1)..).unwrap();
        let j2 = new_vec.iter().max().unwrap();
        j1 * 10 + j2
    } else {
        let new_vec = r_vec.get(..r_vec.len() - 1).unwrap();
        let j2 = new_vec.iter().max().unwrap();
        j2 * 10 + j1
    }
}

/// Finds the largest element in a vector which does not exist within n elements
/// of the end
fn highest_suitable_element(vec: &Vec<u64>, exclusion_len: usize) -> usize {
    vec.get(..vec.len() - exclusion_len)
        .unwrap()
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, x)| x)
        .unwrap()
        .0
}

/// Get the maximum joltage from a bank of batteries, where the total is
/// comprised of n successive (but not necessarily adjacent) joltages from the
/// bank.
fn get_joltage_2(ratings: &String, n: usize) -> u64 {
    // Parse string data and convert to vector
    let mut r_vec: Vec<u64> = ratings
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    // Find the optimal ratings by iterating through to find the highest
    // possible rating for each position in the vector.
    let mut opt_ratings = Vec::new();
    for i in (0..n).rev() {
        let loc = highest_suitable_element(&r_vec, i);
        opt_ratings.push(*r_vec.get(loc).unwrap());
        r_vec = r_vec.split_off(loc + 1);
    }

    opt_ratings
        .iter()
        .enumerate()
        .map(|(i, r)| r * 10_u64.pow((n - 1 - i) as u32))
        .sum()
}

/// Sum the joltages from each bank to solve part 1
fn sum_joltages_2(banks: &Vec<String>) -> u32 {
    banks.iter().map(get_joltage_1).sum()
}

/// Sum the joltages from each bank to solve part 2
fn sum_joltages_n(banks: &Vec<String>, n: usize) -> u64 {
    banks.iter().map(|b| get_joltage_2(b, n)).sum()
}

fn main() {
    let banks = parse_file("input.txt");

    // Part 1
    let total = sum_joltages_2(&banks);
    println!("Part 1 total joltage = {}", total);

    // Part 2
    let total = sum_joltages_n(&banks, 12);
    println!("Part 2 total joltage = {}", total);

    // Part 1 (general) - This could be used instead, but the original solution
    // for part 1 is quicker so has been preserved
    let total = sum_joltages_n(&banks, 2);
    println!("Part 1 total joltage (from general) = {}", total);
}
