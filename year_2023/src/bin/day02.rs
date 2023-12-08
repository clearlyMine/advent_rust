use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{alpha1, digit1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::time::Instant;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day02.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day02_sample.txt");

fn main() {
    let time_start = Instant::now();
    let res1 = process_part_1(INPUT);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(INPUT);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let (red_limit, blue_limit, green_limit) = (12, 14, 13);
    input
        .lines()
        .map(|line| {
            let (_, games) = parse_line(line).expect("Unable to parse line");
            games
        })
        .enumerate()
        .fold(0, |sum, (i, games)| {
            let (mut min_red_needed, mut min_green_needed, mut min_blue_needed) = (0, 0, 0);
            for game in games {
                min_red_needed = min_red_needed.max(game.red);
                min_blue_needed = min_blue_needed.max(game.blue);
                min_green_needed = min_green_needed.max(game.green);

                if min_red_needed > red_limit
                    || min_blue_needed > blue_limit
                    || min_green_needed > green_limit
                {
                    return sum;
                }
            }
            sum + i + 1
        })
}

fn process_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, games) = parse_line(line).unwrap();
            games
        })
        .fold(0, |sum, games| {
            let (mut min_red_needed, mut min_green_needed, mut min_blue_needed) = (0, 0, 0);
            for game in games {
                min_red_needed = min_red_needed.max(game.red);
                min_blue_needed = min_blue_needed.max(game.blue);
                min_green_needed = min_green_needed.max(game.green);
            }
            sum + min_red_needed * min_green_needed * min_blue_needed
        })
}

#[derive(Debug, Clone, Copy)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new(red: u32, green: u32, blue: u32) -> Game {
        Game { red, green, blue }
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, _) = take_till(|c| c == ':')(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, games) = separated_list1(tag("; "), parse_game)(input)?;
    Ok((input, games))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, balls) =
        separated_list1(tag(", "), separated_pair(parse_number, space1, alpha1))(input)?;
    let mut blue = 0;
    let mut green = 0;
    let mut red = 0;

    for (count, color) in balls {
        match color {
            "blue" => blue = count,
            "green" => green = count,
            "red" => red = count,
            _ => panic!("Unknown color: {}", color),
        }
    }
    Ok((input, Game::new(red, green, blue)))
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 8)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 1931)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 2286)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 83105)
    }
}
