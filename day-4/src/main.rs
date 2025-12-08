// Solution to day 4 of the Advent of Code challenge

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Reads the provided file, converting . to 0 and @ to 1 within an array
fn parse_file(name: &str) -> Grid {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        // This will map characters automatically to their byte representation
        let line_data: Vec<u8> = Vec::from(line.unwrap())
            .iter()
            .map(|x| match x {
                46 => 0,
                64 => 1,
                _ => panic!("Unrecognised value in vector."),
            })
            .collect();
        grid.push(line_data);
    }
    Grid::new(grid)
}

/// Stores a square grid of coordinates, representing the location of paper
/// rolls, where a 1 indicates the presence of a roll and 0 the lack thereof.
#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<u8>>,
    x_size: usize,
    y_size: usize,
}
impl Grid {
    pub fn new(grid: Vec<Vec<u8>>) -> Self {
        let x_size = grid.len();
        let y_size = grid[0].len();
        Grid {
            grid,
            x_size,
            y_size,
        }
    }
    /// Retrieve a value from the grid
    pub fn get(&self, x_pos: usize, y_pos: usize) -> u8 {
        self.grid[x_pos][y_pos]
    }
    /// Set a value within the grid
    pub fn set(&mut self, x_pos: usize, y_pos: usize, value: u8) {
        self.grid[x_pos][y_pos] = value;
    }
    /// Checks if a provided position has a roll
    pub fn has_roll(&self, x_pos: usize, y_pos: usize) -> bool {
        self.get(x_pos, y_pos) == 1
    }
    /// Checks if a roll is accessible, as defined by having <= n adjacent rolls
    pub fn can_be_accessed(&self, x_pos: usize, y_pos: usize, n_max: u8) -> bool {
        let mut count = 0;
        for i in 0..3 {
            if (i == 0 && x_pos == 0) || (i == 2 && x_pos == self.x_size - 1) {
                continue;
            }
            for j in 0..3 {
                if (i == 1 && j == 1)
                    || (j == 0 && y_pos == 0)
                    || (j == 2 && y_pos == self.y_size - 1)
                {
                    continue;
                }

                count += self.get(x_pos + i - 1, y_pos + j - 1);
                if count > n_max {
                    return false;
                }
            }
        }
        true
    }
    /// Remove a number of rolls from the grid at the provided indices
    pub fn remove_rolls(&mut self, indices: &Vec<(usize, usize)>) {
        for (x, y) in indices {
            self.set(*x, *y, 0);
        }
    }
}

/// Finds the number of accessible rolls in the grid, where it is accessible if
/// the number of adjacent rolls is less then 4.
fn count_accessible_rolls(grid: &Grid) -> u32 {
    let mut total_accessible = 0;
    for i in 0..grid.x_size {
        for j in 0..grid.y_size {
            if grid.has_roll(i, j) && grid.can_be_accessed(i, j, 3) {
                total_accessible += 1
            }
        }
    }
    total_accessible
}

/// Identifies accessible rolls and removes them iteratively until None remain
fn count_removeable_rolls(mut grid: Grid) -> usize {
    let mut total_removed = 0;
    loop {
        let mut accessible_coords = Vec::new();
        for i in 0..grid.x_size {
            for j in 0..grid.y_size {
                if grid.has_roll(i, j) && grid.can_be_accessed(i, j, 3) {
                    accessible_coords.push((i, j));
                }
            }
        }
        if accessible_coords.is_empty() {
            break;
        }
        grid.remove_rolls(&accessible_coords);
        total_removed += accessible_coords.len();
    }
    total_removed
}

fn main() {
    let grid = parse_file("input.txt");

    // Part 1
    let total = count_accessible_rolls(&grid);
    println!("Total accessible rolls = {}", total);

    // Part 2
    let total = count_removeable_rolls(grid.clone());
    println!("Total removed rolls = {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let grid = parse_file("input.txt");
        assert_eq!(count_accessible_rolls(&grid), 1351)
    }

    #[test]
    fn part_2() {
        let grid = parse_file("input.txt");
        assert_eq!(count_removeable_rolls(grid.clone()), 8345)
    }
}
