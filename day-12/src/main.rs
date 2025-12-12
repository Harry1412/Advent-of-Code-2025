// Solution to day 12 of the Advent of Code challenge

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

/// Defines an area method to be defined across all objects - there isn't too
/// much reason to have this as Present and Region are always treated
/// separately, but it doesn't hurt to implement this way.
trait Area {
    fn area(&self) -> usize;
}

/// Stores the list of points which comprise a present
struct Present {
    points: Vec<(usize, usize)>,
}
impl Present {
    fn new() -> Self {
        Present { points: Vec::new() }
    }
}
impl Area for Present {
    /// Returns the area covered by the present
    fn area(&self) -> usize {
        self.points.len()
    }
}

/// Stores the size of a provided region and the presents which are required to
/// fit within it
struct Region {
    size: (usize, usize),
    required_presents: Vec<usize>,
}
impl Region {
    fn new(size: (usize, usize), required_presents: Vec<usize>) -> Self {
        Region {
            size,
            required_presents,
        }
    }
}
impl Area for Region {
    /// Returns the total available area within the area
    fn area(&self) -> usize {
        self.size.0 * self.size.1
    }
}

/// Reads lines after a present is detected and generates a Present object from
/// this
fn parse_present(iter: &mut Lines<BufReader<File>>) -> Present {
    let mut i = 0;
    let mut present = Present::new();
    while let Some(Ok(next_data)) = iter.next() {
        if next_data.is_empty() {
            break;
        }
        present.points.extend(
            &mut next_data
                .chars()
                .enumerate()
                .filter_map(|(j, c)| (c == '#').then_some((i, j))),
        );
        i += 1;
    }
    present
}

/// Reads a region line and converts to an object
fn parse_region(data: String) -> Region {
    let (s, a) = data.split_once(":").unwrap();
    let (s1, s2) = s.split_once("x").unwrap();
    let req_presents = a.split_whitespace().map(|s| s.parse().unwrap()).collect();
    Region::new((s1.parse().unwrap(), s2.parse().unwrap()), req_presents)
}

/// Parses the input file to produce a graph of nodes & edges
fn parse_file(name: &str) -> (Vec<Region>, Vec<Present>) {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    let mut presents = Vec::new();
    let mut regions = Vec::new();

    let mut iter = reader.lines();
    while let Some(data) = iter.next() {
        let line_data = data.unwrap();
        // Lines ending with : are the start of presents
        if line_data.ends_with(":") {
            presents.push(parse_present(&mut iter));
        }
        // Otherwise it is a region line
        else {
            regions.push(parse_region(line_data));
        }
    }
    (regions, presents)
}

/// Filter regions which can never fit the required presents by calculating the
/// total area that would need to be occupied by these and comparing to the area
/// of the region
fn filter_by_total_area(regions: Vec<Region>, presents: &Vec<Present>) -> Vec<Region> {
    regions
        .into_iter()
        .filter(|area| {
            let total_area = area
                .required_presents
                .iter()
                .zip(presents)
                .map(|(n, p)| n * p.area())
                .sum::<usize>();
            total_area <= area.area()
        })
        .collect()
}

fn main() {
    let (regions, presents) = parse_file("input.txt");

    let filtered_areas = filter_by_total_area(regions, &presents);
    // It transpires that this filtering is enough to get the correct number of
    // valid regions, so we shall avoid any further processing (this is very
    // specific to this problem).
    println!("Valid regions after filtering = {}", filtered_areas.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let (regions, presents) = parse_file("input.txt");
        assert_eq!(filter_by_total_area(regions, &presents).len(), 410)
    }
}
