use crate::utils;
use itertools::Itertools;
use std::{collections::HashMap, io::BufRead};

pub fn day14() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day14")? {
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
    println!("DAY14: {}", file);

    let file = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(file);

    let input: Vec<Vec<char>> = parse_input(&mut reader)?;
    // println!("{:?}", input);

    let result_p1 = solve_p1(&input).ok_or("No solution")?;
    let result_p2 = solve_p2(&input).ok_or("No solution")?;

    Ok((result_p1, result_p2))
}

fn parse_input<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    let mut input = Vec::new();

    for line in reader.lines() {
        let line = line?;

        input.push(line.chars().collect_vec());
    }

    Ok(input)
}

fn solve_p1(input: &[Vec<char>]) -> Option<usize> {
    let mut result = 0;

    let mut column_weights = (0..input[0].len()).map(|_| input.len()).collect_vec();

    for (y, line) in input.iter().enumerate() {
        let line_weight = input.len() - y;
        for (x, &c) in line.iter().enumerate() {
            match c {
                '.' => (),
                '#' => column_weights[x] = line_weight - 1,
                'O' => {
                    // print!("{} ", column_weights[x]);
                    result += column_weights[x];
                    column_weights[x] -= 1;
                }
                _ => return None,
            }
        }
        // println!();
        // println!("{:?}", column_weights);
    }

    Some(result)
}

fn rotate_map(input: &[Vec<char>]) -> Option<Vec<Vec<char>>> {
    let width = input[0].len();
    let height = input.len();

    let mut result = vec![vec!['.'; height]; width];

    let mut ceiling_cache = vec![0; width];

    for (y, line) in input.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    ceiling_cache[x] = y + 1;
                    result[x][height - 1 - y] = '#';
                }
                'O' => {
                    // print!("{} ", column_weights[x]);
                    result[x][height - 1 - ceiling_cache[x]] = 'O';
                    ceiling_cache[x] += 1;
                }
                _ => return None,
            }
        }
    }

    Some(result)
}

fn sum_weights(input: &[Vec<char>]) -> Option<usize> {
    let mut result = 0;

    for (y, line) in input.iter().enumerate() {
        let line_weight = input.len() - y;
        for &c in line {
            match c {
                '.' => (),
                '#' => (),
                'O' => {
                    result += line_weight;
                }
                _ => return None,
            }
        }
    }

    Some(result)
}

fn solve_p2(input: &[Vec<char>]) -> Option<usize> {
    let mut input = input.to_vec();

    let mut cache_index: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut cache_value: Vec<Vec<Vec<char>>> = Vec::new();

    let cycles: usize = 1_000_000_000 * 4;

    for i in 0..cycles {
        input = rotate_map(&input)?;

        if cache_index.contains_key(&input) {
            // Loop detected, no need to continue

            // println!("{}: {}", i, cache_index[&input]);
            let cycle_start = cache_index[&input];
            let cycle_length = i - cycle_start;
            let cycle_remaining = cycles - cycle_start - 1;
            input = cache_value[cycle_start + cycle_remaining % cycle_length].clone();
            break;
        } else {
            cache_index.insert(input.clone(), i);
            cache_value.push(input.clone());
        }
        // if i % 1000 == 0 {
        //     println!("{}/{}", i, cycles);
        // }
    }

    // println!("{}", input.iter().map(|l| l.iter().join("")).join("\n"));

    sum_weights(&input)
}
