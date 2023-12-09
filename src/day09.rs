use std::io::BufRead;
use itertools::Itertools;
use regex::Regex;

use crate::utils;

const NUM_RE: &str = r"(-?[0-9]+)";

pub fn day09() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day09")? {
        handle(run(&file));
    }
    Ok(())
}

fn handle(result: Result<(i64, i64), Box<dyn std::error::Error>>) {
    match result {
        Ok((result_p1, result_p2)) => println!("Result: Part1 {} | Part2 {}", result_p1, result_p2),
        Err(e) => println!("Error: {}", e),
    }
    println!();
}

fn run(file: &str) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    println!("DAY09: {}", file);

    let file = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(file);

    let input = parse_input(&mut reader)?;

    // println!("{:?}", input);

    let results = input
        .into_iter()
        .map(|line| solve_line(&line))
        .collect::<Option<Vec<(i64, i64)>>>()
        .ok_or("No solution")?;

    let (results1, results2): (Vec<i64>, Vec<i64>) = results
        .into_iter()
        .unzip();
        // .sum::<Option<i64>>()
        // .ok_or("No solution")?;

    Ok((results1.iter().sum(), results2.iter().sum()))
}

fn parse_input<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<Vec<Vec<i64>>, Box<dyn std::error::Error>> {
    let num_re = Regex::new(NUM_RE)?;

    let mut lines = Vec::new();
    
    for line in reader.lines() {
        let line = line?;

        let mut nums = Vec::new();
        
        for cap in num_re.captures_iter(line.as_str()) {
            let (_, [num]) = cap.extract();
            nums.push(num.parse::<i64>()?);
        }
        lines.push(nums);
    }

    Ok(lines)
}

fn solve_line(line: &[i64]) -> Option<(i64, i64)> {
    // println!("{:?}", line);

    if line.is_empty() {
        // No pattern found
        return None;
    }

    if line.iter().all(|&n| n == 0) {
        // Pattern found (this level of derivation is constant 0)
        return Some((0, 0));
    }

    // Next derivation of line
    let next_line = line.iter().zip(line.iter().skip(1)).map(|(&n1, &n2)| n2 - n1).collect_vec();

    // Try to solve next derivation
    let next_num = solve_line(&next_line);

    // Apply solution if found
    next_num.map(|(prev_num, next_num)| {(
        line.first().unwrap() - prev_num,
        line.last().unwrap() + next_num
    )})
}
