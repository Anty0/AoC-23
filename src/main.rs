use std::result;

mod utils;
mod day01;
mod day02;
mod day03;

fn handle(result: result::Result<(), Box<dyn std::error::Error>>) {
    if let Err(e) = result {
        println!("Error: {}", e);
    }
}

fn main() {
    handle(day01::day01());
    handle(day02::day02());
    handle(day03::day03());
}
