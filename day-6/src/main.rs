// Solution to day 6 of the Advent of Code challenge

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Stores a 2D array within a 1D vector for more efficient value retrival
struct Array<T: Copy> {
    data: Vec<T>,
    dim_1: usize,
    dim_2: usize,
}
impl<T: Copy> Array<T> {
    pub fn new(data: Vec<T>, dim_1: usize, dim_2: usize) -> Self {
        Array { data, dim_1, dim_2 }
    }
    fn get(&self, i: usize, j: usize) -> T {
        if i >= self.dim_1 || j >= self.dim_2 {
            panic!("One or more indices are out of the data range.")
        }
        self.data[self.dim_1 * j + i]
    }
}

/// Read the file and map the values to a custom array type
fn parse_file(name: &str) -> (Array<u64>, Vec<char>) {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    let mut ops = Vec::new();
    let mut n_lines = 0;
    let operators = ['+', '*'];

    for line in reader.lines() {
        n_lines += 1;
        let line_data = line.unwrap();
        let first_character = &line_data.chars().next().unwrap();
        if operators.contains(first_character) {
            ops.append(&mut line_data.chars().filter(|x| x != &' ').collect());
        } else {
            data.append(
                &mut line_data
                    .split(" ")
                    .filter_map(|x| match x.parse() {
                        Ok(y) => Some(y),
                        Err(_) => None,
                    })
                    .collect(),
            );
        }
    }

    (Array::new(data, ops.len(), n_lines - 1), ops)
}

/// Perforrm Cephalod math on the array with required operations
fn cephalopod_math(data: &Array<u64>, ops: &Vec<char>) -> u64 {
    (0..data.dim_1)
        .map(|i| {
            let vec = (0..data.dim_2).map(|j| data.get(i, j));
            match ops[i] {
                '+' => vec.sum::<u64>(),
                '*' => vec.product::<u64>(),
                _ => panic!("Unrecognised operator."),
            }
        })
        .sum()
}

/// Read the file and map the values to a vector of vectors, where each
/// sub-vector contains the values that should be used within a particular
/// computation
fn parse_file_2(name: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    let mut ops = Vec::new();
    let operators = ['+', '*'];

    // The following is probably not ideal, but works for now.
    for line in reader.lines() {
        let line_data = line.unwrap();
        let first_character = &line_data.chars().next().unwrap();
        if operators.contains(first_character) {
            ops.append(&mut line_data.chars().filter(|x| x != &' ').collect());
        } else {
            if data.is_empty() {
                data.append(&mut line_data.chars().map(|f| String::from(f)).collect());
            } else {
                data.iter_mut()
                    .zip(line_data.chars())
                    .for_each(|(s, c)| s.push(c));
            }
        }
    }
    let mut c1 = 0;
    let mut conv_data = vec![Vec::new(); ops.len()];
    for value in data {
        let new_value = value.replace(" ", "");
        if new_value.is_empty() {
            c1 += 1;
        } else {
            conv_data[c1].push(new_value.parse().unwrap());
        }
    }

    (conv_data, ops)
}

/// Perforrm Cephalod math on the vector with required operations
fn cephalopod_math_2(data: &Vec<Vec<u64>>, ops: &Vec<char>) -> u64 {
    data.iter()
        .enumerate()
        .map(|(i, v)| match ops[i] {
            '+' => v.iter().sum::<u64>(),
            '*' => v.iter().product::<u64>(),
            _ => panic!("Unrecognised operator."),
        })
        .sum()
}

fn main() {
    // Part 1
    let (data, ops) = parse_file("input.txt");
    let total = cephalopod_math(&data, &ops);
    println!("Grand total = {}", total);

    // Part 2
    let (data, ops) = parse_file_2("input.txt");
    let total = cephalopod_math_2(&data, &ops);
    println!("Grand total (2) = {}", total);
}
