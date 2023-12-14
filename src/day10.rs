use crate::utils;
use std::{collections::HashMap, collections::HashSet, io::BufRead};

type PipeShape = char;

#[derive(Debug)]
struct PipeMap {
    map: Vec<Vec<PipeShape>>,
    start: (usize, usize),
}

const ALL_DIRECTIONS: [(i64, i64); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn dir_invert(dir: (i64, i64)) -> (i64, i64) {
    (-dir.0, -dir.1)
}

// fn dir_rotate_right(dir: (i64, i64)) -> (i64, i64) {
//     (dir.1, -dir.0)
// }

// fn dir_rotate_left(dir: (i64, i64)) -> (i64, i64) {
//     (-dir.1, dir.0)
// }

fn shape_connects_to(shape: PipeShape, direction: (i64, i64)) -> bool {
    matches!(
        (shape, direction),
        ('|', (0, -1))
            | ('|', (0, 1))
            | ('-', (-1, 0))
            | ('-', (1, 0))
            | ('L', (0, -1))
            | ('L', (1, 0))
            | ('J', (0, -1))
            | ('J', (-1, 0))
            | ('7', (0, 1))
            | ('7', (-1, 0))
            | ('F', (0, 1))
            | ('F', (1, 0))
            | ('S', _)
    )
}

fn shapes_are_connected(shape1: PipeShape, shape2: PipeShape, direction: (i64, i64)) -> bool {
    shape_connects_to(shape1, direction) && shape_connects_to(shape2, dir_invert(direction))
}

fn shape_neighbors_right(shape: PipeShape, dir: (i64, i64)) -> Vec<(i64, i64)> {
    match (shape, dir) {
        ('|', (0, 1)) => vec![(-1, 0)],
        ('|', (0, -1)) => vec![(1, 0)],
        ('-', (1, 0)) => vec![(0, 1)],
        ('-', (-1, 0)) => vec![(0, -1)],

        ('L', (0, -1)) => vec![],
        ('L', (1, 0)) => vec![(-1, 0), (0, 1)],
        ('J', (0, -1)) => vec![(0, 1), (1, 0)],
        ('J', (-1, 0)) => vec![],

        ('7', (0, 1)) => vec![],
        ('7', (-1, 0)) => vec![(0, -1), (1, 0)],
        ('F', (0, 1)) => vec![(0, -1), (-1, 0)],
        ('F', (1, 0)) => vec![],
        ('S', _) => vec![],
        _ => {
            println!("{} {:?}", shape, dir);
            vec![]
        }
    }
}

fn shape_neighbors_left(shape: PipeShape, dir: (i64, i64)) -> Vec<(i64, i64)> {
    match (shape, dir) {
        ('|', (0, 1)) => vec![(1, 0)],
        ('|', (0, -1)) => vec![(-1, 0)],
        ('-', (1, 0)) => vec![(0, -1)],
        ('-', (-1, 0)) => vec![(0, 1)],

        ('L', (0, -1)) => vec![(-1, 0), (0, 1)],
        ('L', (1, 0)) => vec![],
        ('J', (0, -1)) => vec![],
        ('J', (-1, 0)) => vec![(0, 1), (1, 0)],

        ('7', (0, 1)) => vec![(0, -1), (1, 0)],
        ('7', (-1, 0)) => vec![],
        ('F', (0, 1)) => vec![],
        ('F', (1, 0)) => vec![(0, -1), (-1, 0)],
        ('S', _) => vec![],
        _ => vec![],
    }
}

// fn print_map(map: &PipeMap, highlight1: &HashSet<(usize, usize)>, highlight2: &HashSet<(usize, usize)>, directions: &HashMap<(usize, usize), (i64, i64)>) {
//     for (y, row) in map.map.iter().enumerate() {
//         for (x, shape) in row.iter().enumerate() {
//             let symbol = match directions.get(&(x, y)) {
//                 Some((0, -1)) => '^',
//                 Some((0, 1)) => 'v',
//                 Some((-1, 0)) => '<',
//                 Some((1, 0)) => '>',
//                 _ => *shape,
//             };
//             if highlight1.contains(&(x, y)) && highlight2.contains(&(x, y)) {
//                 print!("\x1b[1;31m{}\x1b[0m", symbol);
//             } else if highlight1.contains(&(x, y)) {
//                 print!("\x1b[1;32m{}\x1b[0m", symbol);
//             } else if highlight2.contains(&(x, y)) {
//                 print!("\x1b[1;34m{}\x1b[0m", symbol);
//             } else {
//                 print!("{}", symbol);
//             }
//         }
//         println!();
//     }
//     println!();
// }

pub fn day10() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day10")? {
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
    println!("DAY10: Part{}: {}", if part2 { "2" } else { "1" }, file);

    let file = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(file);

    let map = parse_pipe_map(&mut reader)?;
    // println!("{:?}", map);

    let solver = if part2 { solve_p2 } else { solve_p1 };
    let result = solver(&map).ok_or("No solution")?;

    Ok(result)
}

fn parse_pipe_map<R: std::io::BufRead>(
    reader: &mut R,
) -> Result<PipeMap, Box<dyn std::error::Error>> {
    let mut map = Vec::new();
    let mut start = (0, 0);

    for (y, line) in reader.lines().enumerate() {
        let line = line?;

        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x, y);
            }
            row.push(c);
        }

        map.push(row);
    }

    Ok(PipeMap { map, start })
}

fn solve_p1(map: &PipeMap) -> Option<u64> {
    // Dijkstra algorithm using HashMap instead of 2D array to keep track of visited nodes
    // for no reason at all
    // Also queue is not a priority queue, but it doesn't change much as there are only 2 paths to follow

    // Assume map is rectangular
    let width = map.map[0].len();
    let height = map.map.len();

    let mut visited = HashMap::new();
    let mut queue = Vec::new();

    visited.insert(map.start, 0);
    queue.push((map.start, 0));

    while let Some((pos, steps)) = queue.pop() {
        let (x, y) = pos;
        let shape = map.map[y][x];

        for dir in &ALL_DIRECTIONS {
            let (dx, dy) = dir;
            let new_pos = (x as i64 + dx, y as i64 + dy);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= width as i64
                || new_pos.1 >= height as i64
            {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if visited.contains_key(&new_pos) && visited[&new_pos] <= steps + 1 {
                continue;
            }

            if let Some(new_shape) = map.map.get(new_pos.1).and_then(|row| row.get(new_pos.0)) {
                if shapes_are_connected(shape, *new_shape, *dir) {
                    visited.insert(new_pos, steps + 1);
                    queue.push((new_pos, steps + 1));
                }
            }
        }
    }

    // println!("{:?}", visited);

    visited.values().max().copied()
}

fn flood_fill(
    map: &PipeMap,
    edges: &HashSet<(usize, usize)>,
    queue: &mut Vec<(usize, usize)>,
) -> (HashSet<(usize, usize)>, bool) {
    // Flood fill limited by set of edge pipes

    // Assume map is rectangular
    let width = map.map[0].len();
    let height = map.map.len();

    let mut overflow = false;
    let mut visited = HashSet::new();

    while let Some(pos) = queue.pop() {
        if edges.contains(&pos) || visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        let (x, y) = pos;

        for dir in &ALL_DIRECTIONS {
            let (dx, dy) = dir;
            let new_pos = (x as i64 + dx, y as i64 + dy);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= width as i64
                || new_pos.1 >= height as i64
            {
                overflow = true;
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            queue.push(new_pos);
        }
    }

    (visited, overflow)
}

fn solve_p2(map: &PipeMap) -> Option<u64> {
    // Deformed Dijkstra algorithm to only follow one path
    // this way we can use the direction vector to know which side of the loop is inside and which is outside
    // Actually we only know which side is right/left side of the loop
    // when flood filling we can check which side is outside by checking if the flood fill reaches the edge of the map

    // Assume map is rectangular
    let width = map.map[0].len();
    let height = map.map.len();

    // let mut directions = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = Vec::new();

    // We use these to initialize the flood fill
    // We need to check both sides of the loop to see which one is outside
    // (it should be possible also to figure this out by summing up all right/left
    // turns and checking if the sum is higher/lower than 0)
    let mut fill_queue_right = Vec::new();
    let mut fill_queue_left = Vec::new();

    visited.insert(map.start);
    queue.push(map.start);

    while let Some(pos) = queue.pop() {
        let (x, y) = pos;
        let shape = map.map[y][x];

        for dir in &ALL_DIRECTIONS {
            let (dx, dy) = dir;
            let new_pos = (x as i64 + dx, y as i64 + dy);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= width as i64
                || new_pos.1 >= height as i64
            {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            let (nx, ny) = new_pos;

            if visited.contains(&new_pos) {
                continue;
            }

            if let Some(new_shape) = map.map.get(new_pos.1).and_then(|row| row.get(new_pos.0)) {
                if shapes_are_connected(shape, *new_shape, *dir) {
                    // directions.insert(pos, *dir);

                    visited.insert(new_pos);
                    queue.push(new_pos);

                    for fill_dir in shape_neighbors_left(*new_shape, dir_invert(*dir)) {
                        let fill_pos = (nx as i64 + fill_dir.0, ny as i64 + fill_dir.1);
                        if !(fill_pos.0 < 0
                            || fill_pos.1 < 0
                            || fill_pos.0 >= width as i64
                            || fill_pos.1 >= height as i64)
                        {
                            let fill_pos = (fill_pos.0 as usize, fill_pos.1 as usize);
                            fill_queue_right.push(fill_pos);
                        }
                    }

                    for fill_dir in shape_neighbors_right(*new_shape, dir_invert(*dir)) {
                        let fill_pos = (nx as i64 + fill_dir.0, ny as i64 + fill_dir.1);
                        if !(fill_pos.0 < 0
                            || fill_pos.1 < 0
                            || fill_pos.0 >= width as i64
                            || fill_pos.1 >= height as i64)
                        {
                            let fill_pos = (fill_pos.0 as usize, fill_pos.1 as usize);
                            fill_queue_left.push(fill_pos);
                        }
                    }

                    // Only follow one direction
                    // Required to know which side of the loop is inside and which is outside
                    break;
                }
            }
        }
    }

    // Debug
    // let mut fill_overflow_right = false;
    // let mut fill_visited_right = HashSet::new();
    // let mut fill_overflow_left = false;
    // let mut fill_visited_left = HashSet::new();
    // fill_visited_right.extend(fill_queue_right);
    // fill_visited_left.extend(fill_queue_left);

    let (fill_visited_right, fill_overflow_right) =
        flood_fill(map, &visited, &mut fill_queue_right);
    let (fill_visited_left, fill_overflow_left) = flood_fill(map, &visited, &mut fill_queue_left);

    // print_map(map, &visited, &fill_visited_right, &directions);
    // print_map(map, &visited, &fill_visited_left, &directions);

    match (fill_overflow_right, fill_overflow_left) {
        (true, true) => {
            // Both sides of the loop are outside
            // This should not happen
            None
        }
        (false, false) => {
            // Both sides of the loop are inside
            // This should not happen
            None
        }
        (false, true) => {
            // Right side of the loop is inside
            // Left side of the loop is outside
            Some(fill_visited_right.len() as u64)
        }
        (true, false) => {
            // Right side of the loop is outside
            // Left side of the loop is inside
            Some(fill_visited_left.len() as u64)
        }
    }
}
