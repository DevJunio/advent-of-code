use nom::bytes::complete::tag;
use nom::character::complete::{self, alpha1, digit1, line_ending};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::collections::HashMap;
use std::ops::Not;

fn main() {
    let file = include_str!("../input/input.txt");
    let res = process_part1(file);
    println!("Part 1: {res}");

    let res = process_part2(file);
    println!("Part 2: {res}");
}

fn process_part1(input: &str) -> String {
    let (_, games) = parse_games(input).unwrap();
    games.iter()
         .filter_map(validate_cubes)
         .sum::<u32>()
         .to_string()
}

fn process_part2(input: &str) -> String {
    let (_, games) = parse_games(input).unwrap();
    games.iter()
         .map(validate_cubes_power)
         .sum::<u32>()
         .to_string()
}

struct Game<'a> {
    rounds: Vec<Vec<Cube<'a>>>,
    id: &'a str,
}

struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

fn rules(color: &str) -> u32 {
    match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => 0,
    }
}

fn validate_cubes(game: &Game) -> Option<u32> {
    game.rounds
        .iter()
        .any(|round| round.iter().any(|cube| cube.amount > rules(cube.color)))
        .not()
        .then_some(game.id.parse::<u32>().unwrap())
}

fn validate_cubes_power(game: &Game) -> u32 {
    game.rounds
        .iter()
        .fold(HashMap::<&str, u32>::new(), |mut acc, round| {
            round.into_iter().for_each(|cube| {
                acc.entry(cube.color)
                   .and_modify(|v| {
                       *v = (*v).max(cube.amount)
                   })
                   .or_insert(cube.amount);
            });
            acc
        })
        .values()
        .product::<u32>()
}

// red 4
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube { color, amount }))
}

// 4 red, 4 blue, 3 green
fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}

// Game 5: 6 blue, 4 green;
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    // between possible `:` and `;`, use `fn round', receiving a Vec of rounds
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, Game { rounds, id }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    #[test]
    fn test_part1() -> Result<(), Error> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process_part1(input));
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Error> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process_part2(input));
        Ok(())
    }
}
