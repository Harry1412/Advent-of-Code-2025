// Solution to day 11 of the Advent of Code challenge

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Stores a graph with a number of nodes and edges. When introducing a new edge
/// to the graph, if the node is new then it will be assigned an integer id.
/// This id can be view in node map and is then used to represent the edge.
#[derive(Debug)]
struct Graph {
    node_map: HashMap<String, u16>,
    edges: HashMap<u16, Vec<u16>>,
}
impl Graph {
    fn new() -> Self {
        Graph {
            node_map: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    /// Add a set of edges from a start point to some end pints
    fn add_edges(&mut self, start: &str, ends: Vec<&str>) {
        let s_id = self._add_or_get_node_value(start);
        let e_ids = ends
            .iter()
            .map(|id| self._add_or_get_node_value(id))
            .collect();
        self.edges.insert(s_id, e_ids);
    }
    fn _add_or_get_node_value(&mut self, node_id: &str) -> u16 {
        match self.node_map.get(node_id) {
            Some(n) => *n,
            None => {
                let new_id = self.node_map.len() as u16;
                self.node_map.insert(node_id.to_string(), new_id);
                new_id
            }
        }
    }
}

/// Parses the input file to produce a graph of nodes & edges
fn parse_file(name: &str) -> Graph {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    let mut graph = Graph::new();
    for line in reader.lines() {
        let line_data = line.unwrap();
        let sep = line_data.find(":").unwrap();
        graph.add_edges(
            line_data.get(..sep).unwrap(),
            line_data.get(sep + 2..).unwrap().split(" ").collect(),
        );
    }
    graph
}

/// Counts the number of paths which lead from the node 'you' to the node 'out'.
/// This is achieved by start from 'you' and iterating through the edges,
/// storing the current node of each path and the number of paths that led to
/// that node. Once a path reaches the terminal node of a dead-end the path is
/// removed. Note: This will only work for non-cyclic graphs.
fn count_paths_from_you_to_out(graph: &Graph) -> u16 {
    let target_node = graph.node_map.get("out").unwrap();

    let mut n_paths_to_out = 0;
    let mut paths = HashMap::from([(graph.node_map.get("you").unwrap(), 1)]);

    // Run until all possible paths have been explored
    while !paths.is_empty() {
        let mut new_paths = HashMap::new();
        for (node, count) in paths {
            // End path if at terminal node
            if node == target_node {
                n_paths_to_out += count
            }
            // Otherwise extend if there are additional edges from the node
            else if let Some(new_nodes) = graph.edges.get(node) {
                for n in new_nodes {
                    new_paths
                        .entry(n)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
            }
        }
        paths = new_paths
    }
    n_paths_to_out
}

/// Counts the number of paths from 'svr' to 'out' which visited both 'fft' and
/// 'dac'. This works similar to the above function, but adds a flag to track
/// whether the path has visited each of the required nodes.
fn count_problem_paths(graph: &Graph) -> u64 {
    let target_node = graph.node_map.get("out").unwrap();
    let req1 = graph.node_map.get("fft").unwrap();
    let req2 = graph.node_map.get("dac").unwrap();

    let mut problem_paths = 0;
    let mut paths = HashMap::from([((graph.node_map.get("svr").unwrap(), false, false), 1)]);

    // Run until all possible paths have been explored
    while !paths.is_empty() {
        let mut new_paths = HashMap::new();
        for (node, count) in paths {
            // End path if at terminal node
            if node.0 == target_node {
                if node.1 && node.2 {
                    problem_paths += count;
                }
            }
            // Otherwise extend if there are additional edges from the node
            else if let Some(new_nodes) = graph.edges.get(node.0) {
                for n in new_nodes {
                    // If the new node is either of the required ones then
                    // update the tuple with this
                    let key = if n == req1 {
                        (n, true, node.2)
                    } else if n == req2 {
                        (n, node.1, true)
                    } else {
                        (n, node.1, node.2)
                    };
                    new_paths
                        .entry(key)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                }
            }
        }
        paths = new_paths
    }
    problem_paths
}

fn main() {
    let graph = parse_file("input.txt");

    // Part 1
    let total = count_paths_from_you_to_out(&graph);
    println!("Total paths = {}", total);

    // Part 1
    let total = count_problem_paths(&graph);
    println!("Total problematic paths = {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let graph = parse_file("input.txt");
        assert_eq!(count_paths_from_you_to_out(&graph), 552)
    }

    #[test]
    fn part_2() {
        let graph = parse_file("input.txt");
        assert_eq!(count_problem_paths(&graph), 307608674109300)
    }
}
