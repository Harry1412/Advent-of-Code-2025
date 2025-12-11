// Solution to day 8 of the Advent of Code challenge

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Stores the x, y and z position of a node
struct Node {
    x: u64,
    y: u64,
    z: u64,
}
impl Node {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Node { x, y, z }
    }
    /// Returns the square of the distance between
    fn distance(&self, node: &Node) -> u64 {
        self.x.abs_diff(node.x).pow(2)
            + self.y.abs_diff(node.y).pow(2)
            + self.z.abs_diff(node.z).pow(2)
    }
}
impl From<Vec<u64>> for Node {
    /// Generate a node from a provided coordinate of the form [x, y, z]
    fn from(value: Vec<u64>) -> Self {
        if value.len() != 3 {
            panic!("Vector should have 3 elements.")
        }
        Node::new(value[0], value[1], value[2])
    }
}

/// Reads the file and generates a list of Nodes (junction boxes) from the file
/// coordinate
fn parse_file(name: &str) -> Vec<Node> {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|c| {
            Node::from(
                c.unwrap()
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .collect()
}

/// Finds all possible nodes and the distances between them, and then sorts from
/// shortest to largest
fn find_all_distances_sorted(nodes: &[Node]) -> Vec<((usize, usize), u64)> {
    let mut distances = Vec::new();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            distances.push(((i, j), nodes[i].distance(&nodes[j])))
        }
    }
    distances.sort_by_key(|(_, d)| *d);
    distances
}

/// Generates a vector of the n shortest connections between nodes, sorted from
/// shortest to largest distance
fn find_n_shortest_connections(nodes: &[Node], n: usize) -> Vec<(usize, usize)> {
    find_all_distances_sorted(nodes)
        .get(0..n)
        .unwrap()
        .iter()
        .map(|(c, _)| *c)
        .collect()
}

/// Generates a vector of connections between nodes, sorted from shortest to
/// largest distance
fn find_all_connections(nodes: &[Node]) -> Vec<(usize, usize)> {
    find_all_distances_sorted(nodes)
        .iter()
        .map(|(c, _)| *c)
        .collect()
}

/// Adds a new connection to a set of circuits, either merging existing
/// circuits, adding a node to an existing circuit or creating a new circuit.
fn add_connection_to_circuits(circuits: &mut Vec<HashSet<usize>>, connection: &(usize, usize)) {
    let loc1 = circuits.iter().position(|c| c.contains(&connection.0));
    let loc2 = circuits.iter().position(|c| c.contains(&connection.1));
    match (loc1, loc2) {
        (Some(i), Some(j)) => {
            let (i, j) = match i.cmp(&j) {
                Ordering::Less => (i, j),
                Ordering::Greater => (j, i),
                Ordering::Equal => return,
            };
            let to_add = circuits.remove(j);
            circuits[i].extend(to_add);
        }
        (Some(i), None) => {
            circuits[i].insert(connection.1);
        }
        (None, Some(i)) => {
            circuits[i].insert(connection.0);
        }
        (None, None) => circuits.push(HashSet::from([connection.0, connection.1])),
    }
}

/// Finds the n-largest circuits created by a set of connections between
/// junction boxes and multiply the sizes of these circuits together
fn find_and_multiply_n_largest_circuits(connections: &[(usize, usize)], n: usize) -> usize {
    let mut circuits = Vec::new();
    for con in connections {
        add_connection_to_circuits(&mut circuits, con);
    }
    let mut sizes: Vec<usize> = circuits.iter().map(|f| f.len()).collect();
    sizes.sort();
    sizes[sizes.len() - n..].iter().product()
}

/// Find the last connection from the vector which is required to connect all
/// junction boxes into a single circuit
fn find_last_connection_for_complete_circuit(
    connections: &[(usize, usize)],
    n_nodes: usize,
) -> (usize, usize) {
    let mut circuits = Vec::new();
    for con in connections.iter() {
        add_connection_to_circuits(&mut circuits, con);
        if circuits.len() == 1 && circuits[0].len() == n_nodes {
            return *con;
        }
    }
    panic!("Unable to complete circuit")
}

/// Find the last connection required from the vector of connections which is
/// required to connect all junction boxes. The product of the x coordinates of
/// the nodes involved in this connection is then found.
fn get_product_of_last_connection(connections: &[(usize, usize)], nodes: &[Node]) -> u64 {
    let last_connection = find_last_connection_for_complete_circuit(connections, nodes.len());
    nodes[last_connection.0].x * nodes[last_connection.1].x
}

fn main() {
    let nodes = parse_file("input.txt");

    // Part 1
    let shortest_connections = find_n_shortest_connections(&nodes, 1000);
    let total = find_and_multiply_n_largest_circuits(&shortest_connections, 3);
    println!("Total = {}", total);

    // Part 2
    let all_connections = find_all_connections(&nodes);
    let total = get_product_of_last_connection(&all_connections, &nodes);
    println!("Product = {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let nodes = parse_file("input.txt");
        let shortest_connections = find_n_shortest_connections(&nodes, 1000);
        assert_eq!(
            find_and_multiply_n_largest_circuits(&shortest_connections, 3),
            90036
        )
    }

    #[test]
    fn part_2() {
        let nodes = parse_file("input.txt");
        let all_connections = find_all_connections(&nodes);
        assert_eq!(
            get_product_of_last_connection(&all_connections, &nodes),
            6083499488
        )
    }
}
