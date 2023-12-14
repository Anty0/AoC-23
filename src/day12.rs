use crate::utils;
use core::fmt::Debug;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, io::BufRead, usize};

const LINE_RE: &str = r"^(?P<modes>[.#?]+) (?P<groups>[0-9,]+)$";

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum OperationalMode {
    Good,
    Bad,
    Unknown,
}

#[derive(Clone)]
struct Line {
    modes: Vec<OperationalMode>,
    groups: Vec<usize>,
}

impl Debug for OperationalMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationalMode::Good => write!(f, "."),
            OperationalMode::Bad => write!(f, "#"),
            OperationalMode::Unknown => write!(f, "?"),
        }
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let modes = self
            .modes
            .iter()
            .map(|m| format!("{:?}", m))
            .collect::<Vec<_>>()
            .join("");
        let groups = self
            .groups
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "({} {})", modes, groups)
    }
}

pub fn day12() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day12")? {
        handle(run(&file, false));
        handle(run(&file, true));
    }
    Ok(())
}

fn handle(result: Result<usize, Box<dyn std::error::Error>>) {
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();
}

fn run(file: &str, part2: bool) -> Result<usize, Box<dyn std::error::Error>> {
    println!("DAY12: Part{}: {}", if part2 { "2" } else { "1" }, file);

    let file = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(file);

    let input: Vec<Line> = parse_input(&mut reader, part2)?;
    // println!("{:?}", input);

    let result = solve(&input).ok_or("No solution")?;

    Ok(result)
}

fn parse_input<R: std::io::BufRead>(
    reader: &mut R,
    part2: bool,
) -> Result<Vec<Line>, Box<dyn std::error::Error>> {
    let line_re = Regex::new(LINE_RE)?;

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let captures = line_re.captures(line.as_str()).ok_or("Invalid line")?;

        let (_, [modes, groups]) = captures.extract();

        let mut modes = modes.to_string();
        let mut groups = groups.to_string();

        if part2 {
            let modes2 = [
                modes.as_str(),
                modes.as_str(),
                modes.as_str(),
                modes.as_str(),
                modes.as_str(),
            ];
            modes = modes2.join("?");

            let groups2 = [
                groups.as_str(),
                groups.as_str(),
                groups.as_str(),
                groups.as_str(),
                groups.as_str(),
            ];
            groups = groups2.join(",");
        }

        let modes = modes
            .chars()
            .map(|c| match c {
                '.' => Ok(OperationalMode::Good),
                '#' => Ok(OperationalMode::Bad),
                '?' => Ok(OperationalMode::Unknown),
                _ => Err("Invalid mode"),
            })
            .collect::<Result<Vec<OperationalMode>, _>>()?;

        let groups = groups
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;

        lines.push(Line { modes, groups });
    }

    Ok(lines)
}

fn solve(input: &[Line]) -> Option<usize> {
    // let len = input.len();
    input
        .iter()
        .map(|line| {
            // backtrack_groups(
            //     &mut HashMap::new(),
            //     &line.modes,
            //     &line.groups,
            //     line.groups.iter().sum::<usize>() + line.groups.len() - 1,
            //     line.modes
            //         .iter()
            //         .filter(|m| {
            //             **m == OperationalMode::Bad || **m == OperationalMode::Unknown
            //         })
            //         .count(),
            //     line.groups.iter().sum::<usize>(),
            // )

            // Multi-threaded version
            let line = line.clone();
            std::thread::spawn(move || {
                let solutions_count = backtrack_groups(
                    &mut HashMap::new(),
                    &line.modes,
                    &line.groups,
                    line.groups.iter().sum::<usize>() + line.groups.len() - 1,
                    line.modes
                        .iter()
                        .filter(|m| **m == OperationalMode::Bad || **m == OperationalMode::Unknown)
                        .count(),
                    line.groups.iter().sum::<usize>(),
                );
                solutions_count
            })
        })
        .collect_vec() // Force evaluation
        .into_iter()
        .map(|handle| handle.join().unwrap())
        // .enumerate()
        // .map(|(i, solutions_count)| {
        //     println!("{}/{}: {}", i, len, solutions_count);
        //     solutions_count
        // })
        .sum::<usize>()
        .into()

    // Previous solution; worked fine for part 1

    // let mut result = 0;

    // for line in input {
    //     // let mut line = line.clone();
    //     println!("{:?} - START", line);

    //     // let solutions_count_non_optimized = backtrack_possibilities(&mut line, false, 0);
    //     let solutions_count = backtrack_possibilities(&mut line, true, 0);
    //     result += solutions_count;
    //     // println!("{:?} {}", line, solutions_count);
    //     // println!("{:?} {}", line, solutions_count_non_optimized);
    //     // if solutions_count != solutions_count_non_optimized {
    //     //     println!("OPTIMIZATION FAILURE ABOVE");
    //     // }

    //     println!("{:?} {} - DONE", line, solutions_count)
    // }

    // Some(result)
}

// Previous solution; worked fine for part 1

// fn calculate_groups(line: &Line) -> (Vec<usize>, Vec<usize>, usize) {
//     let mut unknown_groups_count = 0;
//     let mut groups = Vec::new();
//     let mut group_indexes = Vec::new();

//     let mut unknown_group_size = 0;
//     let mut group_size = 0;

//     for (i, mode) in line.modes.iter().enumerate() {
//         match *mode {
//             OperationalMode::Good => {
//                 if group_size > 0 {
//                     groups.push(group_size);
//                     group_indexes.push(i - group_size);
//                     group_size = 0;
//                 }
//                 if unknown_group_size > 0 {
//                     unknown_groups_count += 1;
//                     unknown_group_size = 0;
//                 }
//             },
//             OperationalMode::Bad => {
//                 group_size += 1;
//                 if unknown_group_size > 0 {
//                     unknown_groups_count += 1;
//                     unknown_group_size = 0;
//                 }
//             },
//             OperationalMode::Unknown => {
//                 group_size += 1;
//                 unknown_group_size += 1;
//             },
//         }
//     }

//     if group_size > 0 {
//         groups.push(group_size);
//         group_indexes.push(line.modes.len() - group_size);
//     }
//     if unknown_group_size > 0 {
//         unknown_groups_count += 1;
//     }

//     (groups, group_indexes, unknown_groups_count)
// }

// fn backtrack_possibilities(line: &mut Line, optim: bool, index: usize) -> usize {
//     if index >= line.modes.len() {
//         let (current_groups, _, _) = calculate_groups(line);
//         if current_groups == line.groups {
//             // println!("{:?} {:?} {} - OK", line, current_groups, index);
//             return 1;
//         } else {
//             return 0;
//         }
//     }

//     if line.modes[index] != OperationalMode::Unknown {
//         return backtrack_possibilities(line, optim, index + 1);
//     }

//     if optim {
//         // Optimizations are enabled

//         let (current_groups, current_groups_indexes, unknown_groups_count) = calculate_groups(line);

//         // println!("{:?} {:?} {}", line, current_groups, index);

//         if current_groups.len() > line.groups.len() + unknown_groups_count {
//             // No need to backtrack more
//             // This will never be a valid solution
//             return 0;
//         }

//         if index > 1 && current_groups
//                 .iter()
//                 .zip(current_groups_indexes.iter())
//                 .zip(line.groups.iter())
//                 .filter(|((s, i), _)| **i + **s < index - 1)
//                 .any(|((s, _), g)| *s != *g) {
//             // No need to backtrack more
//             // This will never be a valid solution
//             return 0;
//         }
//     }

//     let mut result = 0;

//     for mode in &[OperationalMode::Good, OperationalMode::Bad] {
//         let mut line = line.clone();
//         line.modes[index] = *mode;
//         result += backtrack_possibilities(&mut line, optim, index + 1);
//     }

//     line.modes[index] = OperationalMode::Unknown;

//     result
// }

// New solution; works for both parts

fn check_group(line: &[OperationalMode], group: usize) -> bool {
    // Check if line begins with group of given size

    // group x '#'
    for i in 0..group {
        let c = line.get(i);
        match c {
            None => return false,
            Some(OperationalMode::Good) => return false,
            Some(OperationalMode::Bad) => (),
            Some(OperationalMode::Unknown) => (),
        }
    }

    // one x '.' or EOL
    let c = line.get(group);
    match c {
        None => (),
        Some(OperationalMode::Good) => (),
        Some(OperationalMode::Bad) => return false,
        Some(OperationalMode::Unknown) => (),
    }

    true
}

fn updated_min_len(min_len: usize, group: usize) -> usize {
    // Usize underflow would panic
    // This check prevents that
    if min_len > group + 1 {
        min_len - group - 1
    } else {
        0
    }
}

fn assume_bad(
    cache: &mut HashMap<(Vec<OperationalMode>, Vec<usize>), usize>,
    line: &[OperationalMode],
    groups: &[usize],
    min_len: usize,
    bad: usize,
    min_bad: usize,
    group: usize,
) -> usize {
    if !check_group(line, group) {
        // Expected group is not present - this is not a valid solution
        0
    } else if line.len() == group {
        // Edge case - last group at the end of line
        // This is only required because recursive call would
        // cut line using &line[group+1..] which will panic if
        // line is not longer than group (otherwise the recursive
        // call would handle this case with ease)
        // Since we already know that the line will be empty
        // we can just check if there are any groups left
        // instead of calling recursive function
        if groups.len() == 1 {
            // Valid solution
            1
        } else {
            // We didn't match all groups
            0
        }
    } else {
        // All checks passed - we can place the group here
        // and continue recursively with the rest of the line
        backtrack_groups(
            cache,
            &line[group + 1..],
            &groups[1..],
            updated_min_len(min_len, group),
            bad - group,
            min_bad - group,
        )
    }
}

fn backtrack_groups(
    cache: &mut HashMap<(Vec<OperationalMode>, Vec<usize>), usize>,
    line: &[OperationalMode],
    groups: &[usize],
    min_len: usize,
    bad: usize,
    min_bad: usize,
) -> usize {
    // println!("{:?} {:?} {}", line, groups, min_len);

    if line.is_empty() {
        if groups.is_empty() {
            // Valid solution
            return 1;
        } else {
            // We didn't match all groups
            return 0;
        }
    }

    if bad < min_bad {
        // Not enough bad springs to fulfill all groups
        return 0;
    }

    if line.len() < min_len {
        // Line is too short to fit all groups
        return 0;
    }

    let state = (line.to_vec(), groups.to_vec());

    if let Some(&result) = cache.get(&state) {
        // We have already calculated result for this state
        return result;
    }

    let current = line[0];
    let group = groups.first();

    let result = match (current, group) {
        (OperationalMode::Good, _) => {
            // Current is good ('.')
            backtrack_groups(cache, &line[1..], groups, min_len, bad, min_bad)
        }
        (OperationalMode::Bad, None) => {
            // Current is bad ('#')
            // This marks the start of a group, but we don't have any groups left
            // Thus this is not a valid solution
            0
        }
        (OperationalMode::Bad, Some(&group)) => {
            // Current is bad ('#')
            // We have to match current group otherwise it is not a valid solution
            assume_bad(cache, line, groups, min_len, bad, min_bad, group)
        }
        (OperationalMode::Unknown, None) => {
            // Assume good ('.') for current
            backtrack_groups(cache, &line[1..], groups, min_len, bad, min_bad)
        }
        (OperationalMode::Unknown, Some(&group)) => {
            // Assume bad ('#') for current
            let mut result = assume_bad(cache, line, groups, min_len, bad, min_bad, group);

            // Assume good ('.') for current
            result += backtrack_groups(cache, &line[1..], groups, min_len, bad - 1, min_bad);

            result
        }
    };

    // Save result for this state
    cache.insert(state, result);

    result
}
