use regex::Regex;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::utils;

const NUM_RE: &str = r"([0-9]+)";
const LINE_RE: &str = r"^Card +(?P<card_id>[0-9]+): (?P<winning>.*) \| (?P<available>.*)$";

pub fn day04() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day04")? {
        handle(run(&file));
    }
    Ok(())
}

fn handle(result: Result<(u32, u32), Box<dyn std::error::Error>>) {
    match result {
        Ok((sum_p1, sum_p2)) => println!("Result: Part1={} Part2={}", sum_p1, sum_p2),
        Err(e) => println!("Error: {}", e),
    }
    println!();
}

fn run(file: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    println!("DAY04: {}", file);

    let file = std::fs::File::open(file)?;
    let reader = std::io::BufReader::new(file);

    let result = count_nums(reader)?;

    Ok(result)
}

fn count_nums<R: std::io::BufRead>(reader: R) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let mut sum_p1 = 0_u32;
    let mut sum_p2 = 0_u32;

    let num_re = Regex::new(NUM_RE)?;
    let line_re = Regex::new(LINE_RE)?;

    let mut multiplier_queue = VecDeque::new();

    for line in reader.lines() {
        let l = line?.to_owned();

        let captures = line_re.captures(l.as_str());

        if let Some(captures) = captures {
            let (_, [_, winning, available]) = captures.extract();
            let mut winning_nums = HashSet::new();
            for winning in num_re.captures_iter(winning) {
                let (_, [num]) = winning.extract();
                winning_nums.insert(num.parse::<u32>()?);
            }

            let curr_multiplier = multiplier_queue.pop_front().unwrap_or(0) + 1;

            let mut count = 0_u32;
            for available in num_re.captures_iter(available) {
                let (_, [num]) = available.extract();
                if winning_nums.contains(&num.parse::<u32>()?) {
                    count += 1;
                }
            }

            for i in 1..=count {
                let curr = multiplier_queue.get((i - 1).try_into().unwrap());
                if let Some(curr) = curr {
                    multiplier_queue[(i - 1).try_into().unwrap()] = curr + curr_multiplier;
                } else {
                    multiplier_queue.push_back(curr_multiplier);
                }
            }

            // println!("{}: {} {} {:?}", l, count, curr_multiplier, multiplier_queue);
            sum_p2 += curr_multiplier;

            if count != 0 {
                if count == 1 {
                    sum_p1 += 1;
                } else {
                    sum_p1 += 2_u32.pow(count - 1);
                }
            }
        } else {
            return Err(format!("Invalid line: {}", l).into());
        }
    }

    Ok((sum_p1, sum_p2))
}
