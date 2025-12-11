// Solution to day 10 of the Advent of Code challenge

use good_lp::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Stores the required data for each machine
struct Machine {
    target_lights: u16,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}
impl Machine {
    fn new(target_lights: u16, buttons: Vec<Vec<usize>>, joltages: Vec<usize>) -> Self {
        Machine {
            target_lights,
            buttons,
            joltages,
        }
    }
}

fn find_start_and_end(data: &str, start_char: char, end_char: char) -> (usize, usize) {
    (data.find(start_char).unwrap(), data.find(end_char).unwrap())
}

/// Reads each line and converts data into required format for a machine.
fn parse_file(name: &str) -> Vec<Machine> {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|s| {
            let line_data = s.unwrap();
            let (i_start, i_end) = find_start_and_end(&line_data, '[', ']');
            let (j_start, j_end) = find_start_and_end(&line_data, '{', '}');
            let indicators = line_data
                .get(i_start + 1..i_end)
                .unwrap()
                .chars()
                .rev()
                .map(|c| match c {
                    '#' => '1',
                    '.' => '0',
                    _ => panic!("Unrecognised character."),
                })
                .collect::<String>();
            let target_lights = u16::from_str_radix(&indicators, 2).unwrap();
            let buttons = line_data
                .get(i_end + 2..j_start - 1)
                .unwrap()
                .split(" ")
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(",")
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect();
            let joltages = line_data
                .get(j_start + 1..j_end)
                .unwrap()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            Machine::new(target_lights, buttons, joltages)
        })
        .collect()
}

/// Uses a branch strategy to find the minimum number of presses required for
/// a target light configuration
fn min_presses_for_target_lights(machine: &Machine) -> u32 {
    let mut visited = HashSet::new();
    let mut states = vec![0];
    visited.insert(0);
    for n_presses in 1..100 {
        for state in states.split_off(0) {
            for button in &machine.buttons {
                let mut new_state = state;
                for b in button {
                    new_state ^= 1 << b;
                }
                if new_state == machine.target_lights {
                    return n_presses;
                }
                if visited.insert(new_state) {
                    states.push(new_state)
                }
            }
        }
    }
    panic!("Unable to find a valid solution after iteration limit.")
}

/// Sums the smallest number of button presses which are required to achieve
/// the correct light configuration across all machines
fn find_fewest_total_presses_lights(machines: &[Machine]) -> u32 {
    machines.iter().map(min_presses_for_target_lights).sum()
}

/// Finds the minimum button presses required to achieve a target joltage using
/// a linear programing solver
fn min_presses_for_target_joltages(machine: &Machine) -> u32 {
    let mut problem = ProblemVariables::new();

    // Define a variable for each button to store the number of presses -
    // constrain this as a positive integer
    let button_vars: Vec<Variable> = (0..machine.buttons.len())
        .map(|_| problem.add(variable().integer().min(0)))
        .collect();

    // Set objective as the number of button presses
    let objective: Expression = button_vars
        .iter()
        .fold(Expression::from(0), |acc, &var| acc + var);

    let mut model = problem.minimise(&objective).using(default_solver);

    for (indicator_idx, &target) in machine.joltages.iter().enumerate() {
        // Calculates the total joltage on each counter
        let constraint: Expression = button_vars.iter().zip(&machine.buttons).fold(
            Expression::from(0),
            |acc, (&var, button)| {
                if button.contains(&indicator_idx) {
                    acc + var
                } else {
                    acc
                }
            },
        );
        // Constrain that this much be equal to the target value
        model = model.with(constraint.eq(target as i32));
    }

    // Solve and get number of presses
    let solution = model.solve().unwrap();
    solution.eval(objective) as u32
}

/// Sums the smallest number of button presses which are required to achieve
/// the correct joltage configuration across all machines
fn find_fewest_total_presses_joltages(machines: &[Machine]) -> u32 {
    machines.iter().map(min_presses_for_target_joltages).sum()
}

fn main() {
    let machines = parse_file("input.txt");

    // Part 1
    let total = find_fewest_total_presses_lights(&machines);
    println!("Fewest total presses for lights = {}", total);

    // Part 2
    let total = find_fewest_total_presses_joltages(&machines);
    println!("Fewest total presses for joltages = {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let machines = parse_file("input.txt");
        assert_eq!(find_fewest_total_presses_lights(&machines), 558)
    }

    #[test]
    fn part_2() {
        let machines = parse_file("input.txt");
        assert_eq!(find_fewest_total_presses_joltages(&machines), 20317)
    }
}
