// Solution to day 1 of the Advent of Code challenge

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Read a file and generate a vector of rotation values (where a left rotation
/// is denoted by a negative value)
fn parse_file(name: &str) -> Vec<i32> {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    let mut rotations = Vec::new();

    for line in reader.lines() {
        let mut line = line.unwrap();
        let direction = line.remove(0);
        let mut value: i32 = line.parse().unwrap();
        if direction == 'L' {
            value = -value;
        }
        rotations.push(value);
    }
    rotations
}

/// Apply a number of rotations and count the number of times the dial ends on
/// zero
fn count_zero_stops(start_value: i32, rotations: &Vec<i32>) -> u32 {
    let mut current_value = start_value;
    let mut n_zeros = 0;

    for value in rotations {
        current_value += value;
        current_value = current_value.rem_euclid(100);
        if current_value == 0 {
            n_zeros += 1
        }
    }

    n_zeros
}

/// Apply a number of rotations and count the number of times the dial passes or
/// ends on zero
fn count_zero_clicks(start_value: i32, rotations: &Vec<i32>) -> u32 {
    let mut current_value = start_value;
    let mut n_clicks = 0;

    for value in rotations {
        // Find new value
        let mut new_value = current_value + value;
        // Count a click if the updated value is zero
        if new_value == 0 {
            n_clicks += 1;
        }

        // If the dial has looped around then count additional clicks from this
        if new_value < 0 || new_value > 99 {
            // Remove a click when starting from zero and becoming negative as
            // this should not be counted
            if current_value == 0 && *value < 0 {
                n_clicks -= 1;
            }

            // Find number of times the dial passes zero and the new value
            n_clicks += new_value.div_euclid(100).abs() as u32;
            new_value = new_value.rem_euclid(100);

            // When finishing on zero in the negative case an additional click
            // should also be added
            if new_value == 0 && *value < 0 {
                n_clicks += 1;
            }
        }
        current_value = new_value
    }

    n_clicks
}

fn main() {
    let rotations = parse_file("input.txt");

    // Part 1
    let n_zero_stops = count_zero_stops(50, &rotations);
    println!("Number of times stopped at zero = {}", n_zero_stops);

    // Part 2
    let n_zero_clicks = count_zero_clicks(50, &rotations);
    println!("Number of zero clicks = {}", n_zero_clicks);
}
