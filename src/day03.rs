use itertools::Itertools;
use std::collections::HashMap;

use crate::utils;

pub fn day03() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day03")? {
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
    println!("DAY03: {}", file);

    let file = std::fs::File::open(file)?;
    let reader = std::io::BufReader::new(file);
    let schematic = parse_schematic(reader)?;
    // println!("{:?}", schematic);

    let sum_p1 = schematic.active_values().sum::<u32>();
    let sum_p2 = schematic
        .gear_values()
        .map(|values| values.iter().product::<u32>())
        .sum::<u32>();

    Ok((sum_p1, sum_p2))
}

#[derive(Debug)]
struct Schematic {
    occupied_to_id: HashMap<(usize, usize), u32>,
    id_to_value: HashMap<u32, u32>,
    gear_occupied: Vec<Vec<(usize, usize)>>,
    active: Vec<(usize, usize)>,
}

impl Schematic {
    fn new() -> Self {
        Self {
            occupied_to_id: HashMap::new(),
            id_to_value: HashMap::new(),
            gear_occupied: Vec::new(),
            active: Vec::new(),
        }
    }

    fn active_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.active
            .iter()
            .filter_map(move |p| self.occupied_to_id.get(p))
            .copied()
            .unique()
    }

    fn active_values(&self) -> impl Iterator<Item = u32> + '_ {
        self.active_ids().map(move |id| self.id_to_value[&id])
    }

    fn gear_ids(&self) -> impl Iterator<Item = Vec<u32>> + '_ {
        self.gear_occupied.iter().filter_map(move |occupied| {
            let ids: Vec<u32> = occupied
                .iter()
                .filter_map(move |p| self.occupied_to_id.get(p))
                .copied()
                .unique()
                .collect();
            match ids.len() {
                2 => Some(ids),
                _ => None,
            }
        })
    }

    fn gear_values(&self) -> impl Iterator<Item = Vec<u32>> + '_ {
        self.gear_ids().map(move |ids| {
            ids.iter()
                .map(move |id| self.id_to_value[id])
                .collect::<Vec<u32>>()
        })
    }
}

fn parse_schematic<R: std::io::BufRead>(
    reader: R,
) -> Result<Schematic, Box<dyn std::error::Error>> {
    let mut next_id = 0;
    let mut number_buffer = String::new();
    let mut schematic = Schematic::new();

    macro_rules! end_number {
        ($x:expr, $y:expr) => {
            if !number_buffer.is_empty() {
                let id = next_id;
                next_id += 1;

                let value = number_buffer.parse::<u32>()?;
                for x in $x - number_buffer.len()..$x {
                    schematic.occupied_to_id.insert((x, $y), id);
                    schematic.id_to_value.insert(id, value);
                }
                schematic.id_to_value.insert(id, value);
                number_buffer.clear();
            }
        };
    }

    macro_rules! add_active {
        ($x:expr, $y:expr) => {
            for y in $y - 1..=$y + 1 {
                for x in $x - 1..=$x + 1 {
                    schematic.active.push((x, y));
                }
            }
        };
    }

    macro_rules! add_gear {
        ($x:expr, $y:expr) => {
            let mut occupied = Vec::new();

            for y in $y - 1..=$y + 1 {
                for x in $x - 1..=$x + 1 {
                    occupied.push((x, y));
                }
            }

            schematic.gear_occupied.push(occupied);
        };
    }

    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        let mut max_x = 0;
        for (x, c) in line.chars().enumerate() {
            max_x = x;
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => number_buffer.push(c),
                '.' => end_number!(x, y),
                '*' => {
                    end_number!(x, y);
                    add_active!(x, y);
                    add_gear!(x, y);
                }
                _ => {
                    end_number!(x, y);
                    add_active!(x, y);
                }
            }
        }
        end_number!(max_x + 1, y);
    }

    Ok(schematic)
}
