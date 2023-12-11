use std::{io::BufRead, collections::HashSet};
use core::ops::Range;
use crate::utils;

struct MapInfo {
    map: Vec<(usize, usize)>,
    empty_x: HashSet<usize>,
    empty_y: HashSet<usize>,
}

pub fn day11() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day11")? {
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
    println!("DAY11: Part{}: {}", if part2 { "2" } else { "1" }, file);

    let file = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(file);

    let map = parse_map(&mut reader)?;
    // println!("{:?}", map);

    let result = solve(&map, part2).ok_or("No solution")?;

    Ok(result)
}

fn parse_map<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<MapInfo, Box<dyn std::error::Error>> {
    let mut map = Vec::new();
    let mut max_x = 0;
    let mut non_empty_x = HashSet::new();
    let mut empty_y = HashSet::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        let mut empty = true;

        if line.len() > max_x {
            max_x = line.len();
        }

        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    map.push((x, y));
                    empty = false;
                    non_empty_x.insert(x);
                },
                '.' => {},
                _ => return Err(format!("Invalid char: {}", c).into()),
            }
        }

        if empty {
            empty_y.insert(y);
        }
    }

    let empty_x = (0..max_x)
        .filter(|&x| !non_empty_x.contains(&x))
        .collect();

    Ok(MapInfo { map, empty_x, empty_y })
}

fn iter_path(from: usize, to: usize) -> Range<usize> {
    if from < to {
        from..to
    } else {
        to..from
    }
}

fn solve(map: &MapInfo, part2: bool) -> Option<usize> {
    let mut sum = 0;

    let expanded_value = if part2 { 1000000 } else { 2 };

    for i in 0..map.map.len() {
        for j in i+1..map.map.len() {
            let (x1, y1) = map.map[i];
            let (x2, y2) = map.map[j];

            let distance =
                iter_path(x1, x2)
                    .map(|x| map.empty_x.contains(&x))
                    .map(|x| if x { expanded_value } else { 1 })
                    .sum::<usize>() +
                iter_path(y1, y2)
                    .map(|y| map.empty_y.contains(&y))
                    .map(|y| if y { expanded_value } else { 1 })
                    .sum::<usize>();
            
            sum += distance;
        }
    }

    Some(sum)
}
