use const_format::concatcp;
use regex::Regex;
use std::cmp;

use crate::utils;

const COLOR_RE: &str = r"(?P<amount>[0-9]+) (?P<color>red|green|blue)";
// const ROUND_RE: &str = concatcp!(" *(", COLOR_RE, r"[,;]?)* *");
const LINE_RE: &str = concatcp!("^Game (?<game_id>[0-9]+): (?<games>.*)$");

pub fn day02() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day02")? {
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
    println!("DAY02: {}", file);

    let file = std::fs::File::open(file)?;
    let reader = std::io::BufReader::new(file);
    let result = sum_lines(reader)?;
    Ok(result)
}

fn sum_lines<R: std::io::BufRead>(reader: R) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;

    let mut sum_p1 = 0;
    let mut sum_p2 = 0;

    let line_re = Regex::new(LINE_RE)?;
    let color_re = Regex::new(COLOR_RE)?;

    for line in reader.lines() {
        let l = line?.to_owned();

        let captures = line_re.captures(l.as_str());

        if let Some(captures) = captures {
            let (_, [game_id, games]) = captures.extract();
            // println!("{}: {}", game_id, games);

            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            let mut invalid = false;
            for (_, [amount, color]) in color_re.captures_iter(games).map(|c| c.extract()) {
                let amount = amount.parse::<u32>()?;
                let limit = match color {
                    "red" => RED,
                    "green" => GREEN,
                    "blue" => BLUE,
                    _ => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid color",
                        )
                        .into())
                    }
                };

                if amount > limit {
                    invalid = true;
                }

                match color {
                    "red" => min_red = cmp::max(min_red, amount),
                    "green" => min_green = cmp::max(min_green, amount),
                    "blue" => min_blue = cmp::max(min_blue, amount),
                    _ => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid color",
                        )
                        .into())
                    }
                }

                // println!("{}: {}", amount, color);
            }

            if !invalid {
                sum_p1 += game_id.parse::<u32>()?;
            }

            sum_p2 += min_red * min_green * min_blue;
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "No match").into());
        }
    }

    Ok((sum_p1, sum_p2))
}
