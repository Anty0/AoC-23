use regex::Regex;
use std::cmp;
use std::collections::HashMap;

use crate::utils;

const INPUT_RE: &str = r"(?m)^(?P<name>[a-z]+)s: +(?P<values>[0-9 ]+)$";
const NUM_RE: &str = r"([0-9]+)";

const MAPPING_LIST_RE: &str =
    r"(?P<name>[a-z]+)-to-(?P<name2>[a-z]+) map:\n(?P<mappings>[0-9 \n]*)(?:\n\n|$)";
const MAPPING_RE: &str = r"(?m)^(?P<destination>[0-9]+) +(?P<source>[0-9]+) +(?P<length>[0-9]+)$";

type ConversionMap = HashMap<String, HashMap<String, Vec<(u64, u64, u64)>>>;

pub fn day05() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day05")? {
        handle(run(&file));
    }
    Ok(())
}

fn handle(result: Result<(u64, u64), Box<dyn std::error::Error>>) {
    match result {
        Ok((sum_p1, sum_p2)) => println!("Result: Part1={} Part2={}", sum_p1, sum_p2),
        Err(e) => println!("Error: {}", e),
    }
    println!();
}

fn run(file: &str) -> Result<(u64, u64), Box<dyn std::error::Error>> {
    println!("DAY05: {}", file);

    let file = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(file);

    let (name, values) = parse_initial_input(&mut reader)?;
    let main_map = parse_map(&mut reader)?;
    // println!("{:?}: {:?}", name, values);
    // println!("{:?}", main_map);

    let conversion_names: [&str; 8] = [
        &name,
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ];

    let ranges_p1 = as_ranges(&values);
    // println!("{:?}", ranges_p1);
    let values_p1 = convert_ranges_names(&main_map, &conversion_names, &ranges_p1);
    // let values_p1_sorted = values_p1.iter().sorted().collect::<Vec<_>>();
    // println!("{:?}", values_p1_sorted);
    let min_p1 = min_value_ranges(&values_p1).ok_or("No values")?;

    let ranges_p2 = to_ranges(&values);
    // println!("{:?}", ranges_p2);
    let values_p2 = convert_ranges_names(&main_map, &conversion_names, &ranges_p2);
    // let values_p2_sorted = values_p2.iter().sorted().collect::<Vec<_>>();
    // println!("{:?}", values_p2_sorted);
    let min_p2 = min_value_ranges(&values_p2).ok_or("No values")?;

    Ok((min_p1, min_p2))
}

fn parse_initial_input<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<(String, Vec<u64>), Box<dyn std::error::Error>> {
    let input_re = Regex::new(INPUT_RE)?;
    let num_re = Regex::new(NUM_RE)?;

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let captures = input_re.captures(line.as_str());

    if let Some(captures) = captures {
        let (_, [name, values]) = captures.extract();
        let name = name.to_owned();
        let mut values_list = Vec::new();

        for num in num_re.captures_iter(values) {
            let (_, [num]) = num.extract();
            values_list.push(num.parse::<u64>()?);
        }

        return Ok((name, values_list));
    }

    Err("Invalid input".into())
}

fn parse_map<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<ConversionMap, Box<dyn std::error::Error>> {
    let mapping_list_re = Regex::new(MAPPING_LIST_RE)?;
    let mapping_re = Regex::new(MAPPING_RE)?;

    let mut map = HashMap::new();

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    for captures in mapping_list_re.captures_iter(input.as_str()) {
        let (_, [name, name2, mappings]) = captures.extract();
        let name = name.to_owned();
        let name2 = name2.to_owned();

        let source_map: &mut HashMap<String, Vec<(u64, u64, u64)>> = map.entry(name).or_default();
        let destination_map = source_map.entry(name2).or_default();

        for mapping in mapping_re.captures_iter(mappings) {
            let (_, [destination, source, length]) = mapping.extract();
            let destination = destination.parse::<u64>()?;
            let source = source.parse::<u64>()?;
            let length = length.parse::<u64>()?;

            destination_map.push((source, destination, length));
        }
    }

    Ok(map)
}

fn as_range(value: u64) -> (u64, u64) {
    (value, 1)
}

fn as_ranges(values: &[u64]) -> Vec<(u64, u64)> {
    values.iter().map(|value| as_range(*value)).collect()
}

fn to_ranges(values: &[u64]) -> Vec<(u64, u64)> {
    values
        .iter()
        .step_by(2)
        .zip(values.iter().skip(1).step_by(2))
        .map(move |(start, length)| (*start, *length))
        .collect()
}

fn convert_range<'a>(
    map: &'a ConversionMap,
    name_from: &'a str,
    name_to: &'a str,
    range: (u64, u64),
) -> impl Iterator<Item = (u64, u64)> + 'a {
    let (start, length) = range;
    let end = start + length;

    let mut missing = vec![range];

    let converted: Vec<(u64, u64)> = map[name_from][name_to]
        .iter()
        .filter(move |(source, _, length)| !(*source + *length <= start || *source >= end))
        .map(|(source, destination, length)| {
            let n_start = cmp::max(start, *source);
            let n_end = cmp::min(end, *source + *length);
            let n_length = n_end - n_start;
            let n_offset = n_start - *source;

            missing = missing
                .iter()
                .flat_map(|(s, l)| {
                    let e = *s + *l;
                    let mut new_missing = Vec::new();
                    if e <= n_start || *s >= n_end {
                        // No overlap
                        new_missing.push((*s, *l));
                    } else {
                        if *s < n_start {
                            new_missing.push((*s, n_start - *s));
                        }
                        if e > n_end {
                            new_missing.push((n_end, e - n_end));
                        }
                    }
                    new_missing.into_iter()
                })
                .collect();

            // println!("{}: {:?} --> {:?} -> {:?} <- {:?}", name_from, range, (*destination + n_offset, n_length), missing, (n_start, n_length, n_offset, (*source, *destination, *length)));
            // println!("{:?} => {:?}", (*source, *destination, *length), (*destination + n_offset, n_length));
            (*destination + n_offset, n_length)
        })
        .collect();

    // println!("{}: {:?} -> {:?} -> {:?}", name_from, range, converted, missing);
    converted.into_iter().chain(missing.into_iter())
}

fn convert_ranges<'a>(
    map: &'a ConversionMap,
    name_from: &'a str,
    name_to: &'a str,
    ranges: &[(u64, u64)],
) -> Vec<(u64, u64)> {
    ranges
        .iter()
        .flat_map(|range| convert_range(map, name_from, name_to, *range))
        .collect()
}

fn convert_ranges_names(
    map: &ConversionMap,
    names: &[&str],
    ranges: &[(u64, u64)],
) -> Vec<(u64, u64)> {
    names
        .iter()
        .zip(names.iter().skip(1))
        .fold(ranges.to_vec(), |ranges, (name_from, name_to)| {
            convert_ranges(map, name_from, name_to, &ranges)
        })
}

fn min_value_ranges(ranges: &[(u64, u64)]) -> Option<u64> {
    ranges.iter().map(|(value, _)| *value).min()
}

// Previous Part1 only implementation

// fn convert_value(
//     map: &ConversionMap,
//     name_from: &str,
//     name_to: &str,
//     value: u64,
// ) -> u64 {
//     map[name_from][name_to]
//         .iter()
//         .filter(|(source, _, length)| *source <= value && value < *source + length)
//         .map(|(source, destination, _)| value - *source + *destination)
//         .next().unwrap_or(value)
// }

// fn convert_values(
//     map: &ConversionMap,
//     name_from: &str,
//     name_to: &str,
//     values: &[u64],
// ) -> Vec<u64> {
//     values
//         .iter()
//         .map(|value| convert_value(map, name_from, name_to, *value))
//         .collect()
// }

// fn convert_names(
//     map: &ConversionMap,
//     names: &[&str],
//     values: &[u64],
// ) -> Vec<u64> {
//     names.iter().zip(names.iter().skip(1)).fold(values.to_vec(), |values, (name_from, name_to)| {
//         convert_values(map, name_from, name_to, &values)
//     })
// }
