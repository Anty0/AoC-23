use const_format::concatcp;
use regex::Regex;

use crate::utils;

const NUM_PART1_RE: &str = "([0-9])";
const NUM_PART2_RE: &str = "([0-9]|one|two|three|four|five|six|seven|eight|nine)";

const LINE_PART1_RE_STR: &str =
    concatcp!("(?m)^(?U:.*)", NUM_PART1_RE, r".*", NUM_PART1_RE, r".*$");
const LINE_PART2_RE_STR: &str =
    concatcp!("(?m)^(?U:.*)", NUM_PART2_RE, r".*", NUM_PART2_RE, r".*$");

pub fn day01() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day01")? {
        handle(run(&file, false));
        handle(run(&file, true));
    }
    Ok(())
}

fn handle(result: Result<u32, Box<dyn std::error::Error>>) {
    match result {
        Ok(sum) => println!("Result: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
    println!();
}

fn run(file: &str, part2: bool) -> Result<u32, Box<dyn std::error::Error>> {
    println!("DAY01: Part{}: {}", if part2 { "2" } else { "1" }, file);

    let file = std::fs::File::open(file)?;
    let reader = std::io::BufReader::new(file);
    let result = sum_lines(reader, part2)?;
    Ok(result)
}

fn parse_num(c: &str) -> u32 {
    match c {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => c.parse().unwrap(),
    }
}

fn sum_lines<R: std::io::BufRead>(
    reader: R,
    part2: bool,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut sum = 0;

    let num_re;
    let line_re;
    if part2 {
        num_re = Regex::new(NUM_PART2_RE)?;
        line_re = Regex::new(LINE_PART2_RE_STR)?;
    } else {
        num_re = Regex::new(NUM_PART1_RE)?;
        line_re = Regex::new(LINE_PART1_RE_STR)?;
    }

    for line in reader.lines() {
        let l = line?.to_owned();
        let num1;
        let num2;

        let captures = line_re.captures(l.as_str());
        match captures {
            Some(captures) => {
                // Multiple numbers present
                num1 = parse_num(captures.get(1).unwrap().as_str());
                num2 = parse_num(captures.get(2).unwrap().as_str());
            }
            None => {
                // Only one number present
                let captures_num = num_re.captures(l.as_str());
                match captures_num {
                    Some(captures_num) => {
                        num1 = parse_num(captures_num.get(1).unwrap().as_str());
                        num2 = num1
                    }
                    None => {
                        // No numbers present - invalid line
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "No match",
                        ).into());
                    }
                }
            }
        }

        // println!("{} {} {}", n1, n2, l);
        sum += num1 * 10 + num2;
    }

    Ok(sum)
}
