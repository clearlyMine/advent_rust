use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{alpha1, digit1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/day02.txt");

    let time_start = Instant::now();
    let res1 = process_part_1(input);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
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

fn process_part_1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (_, games) = parse_line(line).unwrap();
            (i, games)
        })
        .fold(0, |mut sum, (i, games)| {
            let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
            for game in games {
                min_red = min_red.max(game.red);
                min_blue = min_blue.max(game.blue);
                min_green = min_green.max(game.green);
            }
            if min_red <= 12 && min_blue <= 14 && min_green <= 13 {
                sum += i + 1;
            }
            sum
        })
}

fn process_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, games) = parse_line(line).unwrap();
            games
        })
        .map(|games| {
            let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
            for game in games {
                min_red = min_red.max(game.red);
                min_blue = min_blue.max(game.blue);
                min_green = min_green.max(game.green);
            }
            min_red * min_green * min_blue
        })
        .sum()
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
