use itertools::Itertools;
use regex::Regex;

use crate::utils;

const GAME_RE: &str = r"^(?P<cards>[2-9TJQKA]{5}) (?P<bet>[0-9]+)$";

type Card = u64;
type Cards = [Card; 5];
type Bet = u64;
type Amount = u64;
type Amounts = [Amount; 15];
type Type = u64;
type Game = (Cards, Bet, Type);

pub fn day07() -> Result<(), Box<dyn std::error::Error>> {
    for file in utils::input_files("day07")? {
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
    println!("DAY07: Part{}: {}", if part2 { "2" } else { "1" }, file);

    let file = std::fs::File::open(file)?;
    let mut reader = std::io::BufReader::new(file);

    let games = parse_input(&mut reader, part2)?;
    // println!("{:?}", games);
    let result = solve(&games);

    Ok(result)
}

fn parse_card(c: char, part2: bool) -> Result<Card, Box<dyn std::error::Error>> {
    match (c, part2) {
        ('2'..='9', _) => Ok(c.to_digit(10).unwrap() as Card),
        ('T', _) => Ok(10),
        ('J', false) => Ok(11),
        ('J', true) => Ok(1),
        ('Q', _) => Ok(12),
        ('K', _) => Ok(13),
        ('A', _) => Ok(14),
        _ => Err(format!("Invalid card: {}", c).into()),
    }
}

fn calculate_type(amounts: Amounts) -> Result<Type, Box<dyn std::error::Error>> {
    let jokers = amounts[1];
    let best: [Amount; 5] = amounts
        .into_iter()
        .dropping(2)
        .sorted()
        .rev()
        .take(5)
        .collect::<Vec<Amount>>()
        .try_into()
        .unwrap();

    // 7 - 5 of a kind
    // 6 - 4 of a kind
    // 5 - Full house
    // 4 - 3 of a kind
    // 3 - 2 pairs
    // 2 - 1 pair
    // 1 - High card

    match (jokers, best) {
        (5, [_, _, _, _, _]) => Ok(7),
        (4, [1, _, _, _, _]) => Ok(7),
        (3, [2, _, _, _, _]) => Ok(7),
        (3, [1, 1, _, _, _]) => Ok(6),
        (2, [3, _, _, _, _]) => Ok(7),
        (2, [2, 1, _, _, _]) => Ok(6),
        (2, [1, 1, 1, _, _]) => Ok(4),
        (1, [4, _, _, _, _]) => Ok(7),
        (1, [3, 1, _, _, _]) => Ok(6),
        (1, [2, 2, _, _, _]) => Ok(5),
        (1, [2, 1, 1, _, _]) => Ok(4),
        (1, [1, 1, 1, 1, _]) => Ok(2),
        (0, [5, _, _, _, _]) => Ok(7),
        (0, [4, 1, _, _, _]) => Ok(6),
        (0, [3, 2, _, _, _]) => Ok(5),
        (0, [3, 1, 1, _, _]) => Ok(4),
        (0, [2, 2, 1, _, _]) => Ok(3),
        (0, [2, 1, 1, 1, _]) => Ok(2),
        (0, [1, 1, 1, 1, 1]) => Ok(1),
        _ => Err(format!("Invalid amounts: {:?}", best).into()),
    }
}

fn parse_input<R: std::io::BufRead>(
    reader: R, part2: bool,
) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let game_re = Regex::new(GAME_RE)?;

    let mut games = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let captures = game_re.captures(&line);

        if let Some(captures) = captures {
            let (_, [cards, bet]) = captures.extract();

            let cards = cards
                .chars()
                .map(|c| parse_card(c, part2))
                .collect::<Result<Vec<Card>, _>>()?;
            assert_eq!(cards.len(), 5);
            let bet = bet.parse::<Bet>()?;
            let amounts = (0..15)
                .map(|v| cards.iter().filter(|c| **c == v).map(|_| 1).sum())
                .collect::<Vec<Amount>>();
            assert_eq!(amounts.len(), 15);

            let cards = cards.try_into().unwrap();
            let amounts = amounts.try_into().unwrap();
            let type_ = calculate_type(amounts)?;

            games.push((cards, bet, type_));
        }
    }

    Ok(games)
}

fn solve(games: &[Game]) -> u64 {
    games
        .iter()
        .sorted_by(|(cards1, _, type1), (cards2, _, type2)| {
            if type1 != type2 {
                type1.cmp(type2)
            } else {
                cards1.cmp(cards2)
            }
        })
        .map(|(_, bet, _)| bet)
        .enumerate()
        .map(|(i, bet)| *bet * (i as u64 + 1))
        .sum()
}
