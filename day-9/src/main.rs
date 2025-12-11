// Solution to day 9 of the Advent of Code challenge

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Stores 2D coordinate
#[derive(Debug)]
struct Coordinate {
    x: u64,
    y: u64,
}
impl Coordinate {
    fn new(x: u64, y: u64) -> Self {
        Coordinate { x, y }
    }
    /// Find the size of the area of a rectangle created by this coordinate and
    /// another
    fn area_with(&self, other: &Coordinate) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

/// Reads each line of the file to a new coordinate within a vector
fn parse_file(name: &str) -> Vec<Coordinate> {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|c| {
            let data = c.unwrap();
            let values = data.split_once(",").unwrap();
            Coordinate::new(values.0.parse().unwrap(), values.1.parse().unwrap())
        })
        .collect()
}

/// Finds all posssible areas from a vector of coordinates and returns the
/// largest of these
fn largest_area(coords: &[Coordinate]) -> u64 {
    (0..coords.len())
        .flat_map(|i| (i + 1..coords.len()).map(move |j| coords[i].area_with(&coords[j])))
        .max()
        .unwrap()
}

/// Checks an area, as defined by two coordinates is not intersected by an edge.
/// As the edge is only along x or y coordinate of the edge is outside of the
/// range of the area.
fn area_not_intersected(
    corner_1: &Coordinate,
    corner_2: &Coordinate,
    edge_start: &Coordinate,
    edge_end: &Coordinate,
) -> bool {
    corner_1.x.min(corner_2.x) >= edge_start.x.max(edge_end.x)
        || corner_1.x.max(corner_2.x) <= edge_start.x.min(edge_end.x)
        || corner_1.y.min(corner_2.y) >= edge_start.y.max(edge_end.y)
        || corner_1.y.max(corner_2.y) <= edge_start.y.min(edge_end.y)
}

/// Find all possible areas and then iterates through them to find the largest
/// area which is not intersected by any of the edges formed by the coordinates.
fn largest_area_2(coords: &[Coordinate]) -> u64 {
    let mut sorted_areas: Vec<(u64, (&Coordinate, &Coordinate))> = (0..coords.len())
        .flat_map(|i| {
            (i + 1..coords.len())
                .map(move |j| (coords[i].area_with(&coords[j]), (&coords[i], &coords[j])))
        })
        .collect();
    sorted_areas.sort_by(|(a, _), (b, _)| b.cmp(a));

    let mut edges: Vec<(&Coordinate, &Coordinate)> =
        coords.windows(2).map(|c| (&c[0], &c[1])).collect();
    edges.push((&coords[coords.len() - 1], &coords[0]));

    // Find largest area by looking for the first area which is not intersected
    // by any edges
    sorted_areas
        .iter()
        .find(|(_, (c1, c2))| {
            edges
                .iter()
                .all(|(e1, e2)| area_not_intersected(c1, c2, e1, e2))
        })
        .unwrap()
        .0
}

fn main() {
    let coords = parse_file("input.txt");

    // Part 1
    let total = largest_area(&coords);
    println!("Largest area = {}", total);

    // Part 2
    let total = largest_area_2(&coords);
    println!("Largest area = {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let coords = parse_file("input.txt");
        assert_eq!(largest_area(&coords), 4790063600)
    }

    #[test]
    fn part_2() {
        let coords = parse_file("input.txt");
        assert_eq!(largest_area_2(&coords), 1516172795)
    }
}
