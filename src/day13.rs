use crate::utils;
use itertools::Itertools;
use std::io::BufRead;

pub fn day13() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day13")? {
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
    println!("DAY13: {}", file);

    let file = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(file);

    let input: Vec<Vec<Vec<char>>> = parse_input(&mut reader)?;
    // println!("{:?}", input);

    let result_p1 = solve_p1(&input).ok_or("No solution")?;
    let result_p2 = solve_p2(&input).ok_or("No solution")?;

    Ok((result_p1, result_p2))
}

fn parse_input<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<Vec<Vec<Vec<char>>>, Box<dyn std::error::Error>> {
    let mut input = Vec::new();

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            input.push(lines);
            lines = Vec::new();
        } else {
            lines.push(line.chars().collect_vec());
        }
    }

    if !lines.is_empty() {
        input.push(lines);
    }

    Ok(input)
}

fn solve_group_p1(group: &[Vec<char>]) -> usize {
    let mut possible_mirrors_x: Vec<usize> = (1..group[0].len()).collect();
    let mut possible_mirrors_y: Vec<usize> = (1..group.len()).collect();

    // println!("sx={:?}", possible_mirrors_x);
    // println!("sy={:?}", possible_mirrors_y);

    for line in group {
        possible_mirrors_x.retain(|&mx| {
            // println!("x{}_inner_l={:?}", mx, line[..mx].iter().rev().collect::<Vec<_>>());
            // println!("x{}_inner_r={:?}", mx, line[mx..].iter().collect::<Vec<_>>());
            // println!("x{}_inner_a={:?}", mx, line);
            line[mx..]
                .iter()
                .zip(line[..mx].iter().rev())
                .all(|(a, b)| a == b)
        });
        // println!("x={:?}", possible_mirrors_x);
    }

    for x in 0..group[0].len() {
        possible_mirrors_y.retain(|&my| {
            // println!("y{}_inner_l={:?}", my, group[..my].iter().rev().map(|line| line[y]).collect::<Vec<_>>());
            // println!("y{}_inner_r={:?}", my, group[my..].iter().map(|line| line[y]).collect::<Vec<_>>());
            // println!("y{}_inner_a={:?}", my, group.iter().map(|line| line[y]).collect::<Vec<_>>());
            group[my..]
                .iter()
                .zip(group[..my].iter().rev())
                .all(|(a, b)| a[x] == b[x])
        });
        // println!("y={:?}", possible_mirrors_y);
    }

    // println!("x={:?}", possible_mirrors_x);
    // println!("y={:?}", possible_mirrors_y);

    let result_x = possible_mirrors_x.into_iter().sum::<usize>();
    let result_y = possible_mirrors_y.into_iter().sum::<usize>() * 100;

    result_x + result_y
}

fn solve_p1(input: &[Vec<Vec<char>>]) -> Option<usize> {
    input
        .iter()
        .map(|group| solve_group_p1(group))
        .sum::<usize>()
        .into()
}

fn solve_group_p2(group: &[Vec<char>]) -> usize {
    let mut possible_mirrors_x: Vec<(usize, bool)> =
        (1..group[0].len()).map(|x| (x, false)).collect();
    let mut possible_mirrors_y: Vec<(usize, bool)> = (1..group.len()).map(|y| (y, false)).collect();

    for line in group {
        possible_mirrors_x = possible_mirrors_x
            .into_iter()
            .filter_map(|(mx, fixed)| {
                let invalid_count = line[mx..]
                    .iter()
                    .zip(line[..mx].iter().rev())
                    .filter(|(a, b)| a != b)
                    .count();
                // println!("x{}_inner_l={:?}", mx, line[..mx].iter().rev().collect::<Vec<_>>());
                // println!("x{}_inner_r={:?}", mx, line[mx..].iter().collect::<Vec<_>>());
                // println!("x{}_inner_a={:?}", mx, line);
                // println!("x{}_invalid_count={:?}", mx, invalid_count);
                match (fixed, invalid_count) {
                    (false, 0) => Some((mx, false)),
                    (false, 1) => Some((mx, true)),
                    (true, 0) => Some((mx, true)),
                    _ => None,
                }
            })
            .collect();
        // println!("x={:?}", possible_mirrors_x);
    }

    for x in 0..group[0].len() {
        possible_mirrors_y = possible_mirrors_y
            .into_iter()
            .filter_map(|(my, fixed)| {
                let invalid_count = group[my..]
                    .iter()
                    .zip(group[..my].iter().rev())
                    .filter(|(a, b)| a[x] != b[x])
                    .count();
                // println!("y{}_inner_l={:?}", my, group[..my].iter().rev().map(|line| line[x]).collect::<Vec<_>>());
                // println!("y{}_inner_r={:?}", my, group[my..].iter().map(|line| line[x]).collect::<Vec<_>>());
                // println!("y{}_inner_a={:?}", my, group.iter().map(|line| line[x]).collect::<Vec<_>>());
                // println!("y{}_invalid_count={:?}", my, invalid_count);
                match (fixed, invalid_count) {
                    (false, 0) => Some((my, false)),
                    (false, 1) => Some((my, true)),
                    (true, 0) => Some((my, true)),
                    _ => None,
                }
            })
            .collect();
        // println!("y={:?}", possible_mirrors_y);
    }

    let result_x = possible_mirrors_x
        .into_iter()
        .filter(|(_, fixed)| *fixed)
        .map(|(x, _)| x)
        .sum::<usize>();
    let result_y = possible_mirrors_y
        .into_iter()
        .filter(|(_, fixed)| *fixed)
        .map(|(y, _)| y)
        .sum::<usize>()
        * 100;

    result_x + result_y
}

fn solve_p2(input: &[Vec<Vec<char>>]) -> Option<usize> {
    input
        .iter()
        .map(|group| solve_group_p2(group))
        .sum::<usize>()
        .into()
}
