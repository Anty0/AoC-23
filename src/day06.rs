use regex::Regex;

use crate::utils;

const NUM_RE: &str = r"([0-9]+)";
const INPUT_RE: &str = r"^Time: *(?P<times>.*)\nDistance: *(?P<distances>.*)\n*$";

type Race = (u64, u64);

pub fn day06() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day06")? {
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
    println!("DAY06: Part{}: {}", if part2 { "2" } else { "1" }, file);

    let file = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(file);

    let races = parse_input(&mut reader, part2)?;
    // println!("{:?}", races);
    let result = solve(&races);

    Ok(result)
}

fn parse_input<R: std::io::BufRead>(
    reader: &mut R, part2: bool,
) -> Result<Vec<Race>, Box<dyn std::error::Error>> {
    let input_re = Regex::new(INPUT_RE)?;
    let num_re = Regex::new(NUM_RE)?;

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let captures = input_re.captures(input.as_str());

    if let Some(captures) = captures {
        let (_, [times, distances]) = captures.extract();

        let mut times = times.to_string();
        let mut distances = distances.to_string();

        if part2 {
            // Pre-process times and distances by removing spaces
            times = times.replace(' ', "");
            distances = distances.replace(' ', "");
        }

        let mut times_list = Vec::new();
        let mut distances_list = Vec::new();

        for num in num_re.captures_iter(times.as_str()) {
            let (_, [num]) = num.extract();
            times_list.push(num.parse::<u64>()?);
        }

        for num in num_re.captures_iter(distances.as_str()) {
            let (_, [num]) = num.extract();
            distances_list.push(num.parse::<u64>()?);
        }

        assert_eq!(times_list.len(), distances_list.len());

        let races = times_list.into_iter().zip(distances_list.into_iter()).collect();

        return Ok(races);
    }

    Err("Invalid input".into())
}

fn solve(races: &[Race]) -> u64 {
    races
        .iter()
        .map(|(time, distance)| solution_range(*time, *distance))
        .map(|(from, to)| to - from + 1)
        .filter(|x| *x > 0)
        .product()
}

fn solution_range(time: u64, distance: u64) -> (u64, u64) {
    let distance = distance + 1; // We need to travel at least one more than the distance

    let d = time * time - 4 * distance;
    // assert!(d >= 0); // No need for this check
    // u64 is always positive and Rust will error out if underflow occurs
    let d = d as f64;
    let d = d.sqrt();
    let d = d / 2.0;

    let t = time as f64 / 2.0;

    let from = (t - d).ceil() as u64;
    let to = (t + d).floor() as u64;

    // // Brute force solution
    // let mut from = time;
    // let mut to = 0;
    // for i in 1 .. time {
    //     let traveled = i * (time - i);
    //     // println!("-- {} {} - {} {}", time, distance, i, traveled);

    //     if traveled <= distance {
    //         continue;
    //     }

    //     from = cmp::min(from, i);
    //     to = cmp::max(to, i);
    // }
    
    // println!("{} {} - {} {} - {}", time, distance, from, to, to - from + 1);
    (from, to)
}
