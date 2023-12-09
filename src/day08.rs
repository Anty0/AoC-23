use std::{io::BufRead, collections::HashMap};
use itertools::Itertools;
use regex::Regex;

use crate::utils;

const PATH_RE: &str = r"(?m)^(?P<path>[LR]*)$";
const NODE_RE: &str = r"^(?P<name>[^ ]*) *= *\((?P<left>[^ ]*), *(?P<right>[^ ]*)\)$";

// type NodeMap = std::collections::HashMap<usize, String>;
type Node = (usize, usize);
type Nodes = Vec<Node>;

#[derive(Debug)]
struct ActorCycle {
    offset: u64,
    length: u64,
    prefix_end_indexes: Vec<u64>,
    loop_end_indexes: Vec<u64>,
}

pub fn day08() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day08")? {
        handle(run(&file, false));
        handle(run(&file, true));
    }
    Ok(())
}

fn handle(result: Result<u64, Box<dyn std::error::Error>>) {
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();
}

fn run(file: &str, part2: bool) -> Result<u64, Box<dyn std::error::Error>> {
    println!("DAY08: Part{}: {}", if part2 { "2" } else { "1" }, file);

    let file = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(file);

    let path = parse_path(&mut reader)?;

    let mut empty_line = String::new();
    reader.read_line(&mut empty_line)?;
    assert_eq!(empty_line, "\n");

    let (nodes, start_ids, end_ids) = parse_nodes(&mut reader, part2)?;
    
    // println!("{:?}", path);
    // println!("{:?}", nodes);
    // println!("{:?}", node_map);

    let cycles = start_ids
        .iter()
        .map(|&start_id| find_cycle(&nodes, &path, start_id, &end_ids))
        .collect_vec();

    // println!("{:?}", cycles);

    let result = solve(&cycles).ok_or("No solution")?;
    // let result = 0;

    Ok(result)
}

fn parse_path<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let path_re = Regex::new(PATH_RE)?;

    let mut input = String::new();
    reader.read_line(&mut input)?;

    let captures = path_re.captures(input.as_str());

    if let Some(captures) = captures {
        let (_, [path]) = captures.extract();

        let path = path
            .chars()
            .map(|c| if c == 'L' { 0 } else { 1 })
            .collect_vec();

        Ok(path)
    } else {
        Err(format!("Invalid input: {}", input).into())
    }
}

fn parse_nodes<R: std::io::BufRead>(
    reader: &mut R, part2: bool
) -> Result<(Nodes, Vec<usize>, Vec<usize>), Box<dyn std::error::Error>> {
    let node_re = Regex::new(NODE_RE)?;

    let mut nodes = Vec::new();
    // let mut node_map = NodeMap::new();
    let mut name_map = HashMap::new();
    let mut start_id = Vec::new();
    let mut end_id = Vec::new();

    for (id, line) in reader.lines(). enumerate() {
        let line = line?;
        let captures = node_re.captures(&line);

        if let Some(captures) = captures {
            let (_, [name, left, right]) = captures.extract();

            if part2 && name.ends_with('A') {
                start_id.push(id);
            } else if part2 && name.ends_with('Z') {
                end_id.push(id);
            } else if name == "AAA" {
                start_id.push(id);
            } else if name == "ZZZ" {
                end_id.push(id);
            }

            nodes.push((left.to_owned(), right.to_owned()));
            // node_map.insert(id, name.to_owned());
            name_map.insert(name.to_owned(), id);
        } else {
            return Err(format!("Invalid input: {}", line).into());
        }
    }

    // println!("{:?}", nodes);
    // println!("{:?}", node_map);
    // println!("{:?}", name_map);

    let nodes = nodes
        .into_iter()
        .map(|(left, right)| (name_map[left.as_str()], name_map[right.as_str()]))
        .collect();

    Ok((nodes, start_id, end_id))
}

fn find_cycle(nodes: &Nodes, path: &[usize], start_id: usize, end_ids: &[usize]) -> ActorCycle {
    let mut current = start_id;
    let mut visited = HashMap::new();
    let mut step_end_indexes = Vec::new();
    let mut steps = 0;

    loop {
        for (i, &direction) in path.iter().enumerate() {
            if end_ids.contains(&current) {
                step_end_indexes.push(steps);
            }

            if visited.contains_key(&(i, current)) {
                let offset = visited[&(i, current)];
                let length = steps - offset;
                return ActorCycle {
                    offset,
                    length,
                    prefix_end_indexes: step_end_indexes.iter().filter(|&&x| x < offset).copied().collect(),
                    loop_end_indexes: step_end_indexes.iter().filter(|&&x| x >= offset).map(|&x| x - offset).collect(),
                };
            }

            visited.insert((i, current.to_owned()), steps);
            steps += 1;

            let (left, right) = nodes[current];

            if direction == 0 {
                current = left;
            } else {
                current = right;
            }
        }
    }
}

fn solve(cycles: &[ActorCycle]) -> Option<u64> {
    let mut stack = Vec::new();
    let mut solutions = Vec::new();

    // println!("{:?}", cycles);

    solve_rec(cycles, &mut stack, &mut solutions);

    solutions.into_iter().min()
}

fn solve_rec(cycles: &[ActorCycle], stack: &mut Vec<(u64, Option<u64>)>, solutions: &mut Vec<u64>) {
    if cycles.is_empty() {
        // Check if current combination is a valid solution

        // println!("{:?}", stack);

        if stack.is_empty() {
            return;
        }

        if stack.len() == 1 {
            // Edge case - only one actor (all part1 solutions)
            let (offset, _) = stack[0];
            solutions.push(offset);
            return;
        }

        let mut period = 1;

        for actor in stack.iter() {
            match actor {
                (_, None) => {
                    // FIXME: Let's ignore support for non-periodic visits
                    // This isn't needed to complete the puzzle
                    return;
                }
                (offset, Some(length)) => {
                    if *offset == *length {
                        // Edge case used in this puzzle (part2 solution of user specific input)
                        // Greatly simplifies the solution
                        // All actors start their cycle at 0 making it easy to find where the period starts and ends
                        period = num_integer::lcm(period, *length);
                    } else {
                        // FIXME: No reason to support this - not used in this puzzle
                        // This represents inputs where we don't know where the period starts
                        // I'll happily ignore support for this for now
                        return;
                    }
                }
            }
        }

        solutions.push(period);

        // println!("{} - {:?}", period, stack);
        // println!("{:?}", stack);
        return;
    }

    let current = &cycles[0];
    let others = &cycles[1..];
    
    for &prefix_end_index in &current.prefix_end_indexes {
        stack.push((prefix_end_index, None));
        solve_rec(others, stack, solutions);
        stack.pop();
    }

    for &loop_end_index in &current.loop_end_indexes {
        let offset = current.offset + loop_end_index;
        stack.push((offset, Some(current.length)));
        solve_rec(others, stack, solutions);
        stack.pop();
    }
}
