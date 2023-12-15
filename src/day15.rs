use itertools::Itertools;
use std::io::Read;

use crate::utils;

pub fn day15() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day15")? {
        handle(run(&file));
    }
    Ok(())
}

fn handle(result: Result<(usize, usize), Box<dyn std::error::Error>>) {
    match result {
        Ok((result_p1, result_p2)) => {
            println!("Result: Part1 = {} | Part2 = {}", result_p1, result_p2)
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();
}

fn run(file: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    println!("DAY15: {}", file);

    let file = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(file);

    let mut input = Vec::new();
    reader.read_to_end(&mut input)?;

    let result_p1 = solve_p1(&input).ok_or("No solution")?;
    let result_p2 = solve_p2(&input).ok_or("No solution")?;

    Ok((result_p1, result_p2))
}

fn solve_p1(input: &[u8]) -> Option<usize> {
    input
        .split(|&c| c == b',')
        .map(|input| {
            let mut v = 0;
            for &c in input {
                if c == b'\n' {
                    continue;
                }
                v += c as usize;
                v *= 17;
                v %= 256;
            }
            v
        })
        .sum::<usize>()
        .into()
}

fn solve_p2(input: &[u8]) -> Option<usize> {
    input
        .split(|&c| c == b',')
        .map(|input| {
            let mut hash = 0;
            let mut label = String::new();
            let mut reading_action = false;
            let mut action = None;
            let mut value = None;
            for &c in input {
                match (reading_action, c) {
                    (_, b'\n') => (),
                    (true, _) => {
                        assert!(c.is_ascii_digit());
                        value = Some(c - b'0');
                    }
                    (false, b'=' | b'-') => {
                        reading_action = true;
                        action = Some(c);
                    }
                    (false, _) => {
                        label.push(c as char);
                        hash += c as usize;
                        hash *= 17;
                        hash %= 256;
                    }
                }
            }
            (hash, label, action, value)
        })
        .sorted_by(|e1, e2| e1.0.cmp(&e2.0))
        .group_by(|e| e.0)
        .into_iter()
        .map(|(hash, group)| {
            // First attempt - process in reverse order
            // Was hoping to avoid needing to remove elements
            // but at it turns out, it's not possible to
            // do this while keeping the correct order
            // of the elements.

            // let group = group.collect_vec();
            // let mut removed = HashSet::new();
            // let mut contents = Vec::new();
            // for (_, label, action, value) in group.into_iter().rev() {
            //     match (action, value) {
            //         (Some(b'='), Some(value)) => {
            //             if removed.insert(label) {
            //                 contents.push(value);
            //             }
            //         },
            //         (Some(b'-'), None) => {
            //             removed.insert(label);
            //         },
            //         _ => {
            //             return None;
            //         },
            //     }
            // }

            let mut contents = Vec::new();

            for (_, label, action, value) in group {
                let index = contents
                    .iter()
                    .enumerate()
                    .filter(|(_, (l, _))| l == &label)
                    .map(|e| e.0)
                    .next();

                match (action, value, index) {
                    (Some(b'='), Some(value), Some(index)) => {
                        contents[index].1 = value;
                    }
                    (Some(b'='), Some(value), None) => {
                        contents.push((label, value));
                    }
                    (Some(b'-'), None, Some(index)) => {
                        contents.remove(index);
                    }
                    (Some(b'-'), None, None) => (),
                    _ => {
                        return None;
                    }
                }
            }

            contents
                .into_iter()
                // .rev()
                .enumerate()
                .map(|(i, (_, value))| {
                    // println!("{}: {} * {} * {}", hash, hash + 1, i + 1, value);
                    (hash + 1) * (i + 1) * value as usize
                })
                .sum::<usize>()
                .into()
        })
        .sum()
}
