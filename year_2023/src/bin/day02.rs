use nom::bytes::complete::{tag, take_till};
use nom::combinator::map_res;
use nom::{character, *};
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

#[derive(Debug, Clone, Copy)]
enum BallColor {
    Red,
    Blue,
    Green,
}

fn process_part_1(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (_, games) = parse_line(line).unwrap();
            let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
            for game in games {
                min_red = min_red.max(game.red);
                min_blue = min_blue.max(game.blue);
                min_green = min_green.max(game.green);
            }
            if min_red <= 12 && min_blue <= 14 && min_green <= 13 {
                return i + 1;
            }
            0
        })
        .sum()
}

fn process_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, games) = parse_line(line).unwrap();
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
    let (input, games) = parse_games(input)?;
    Ok((input, games))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let mut games: Vec<Game> = vec![];
    let mut input = input;
    loop {
        let x;
        (input, x) = bytes::complete::take_till(|c| c == ';')(input)?;
        let (rest, g) = parse_balls(x)?;
        if rest.len() != 0 {
            panic!()
        }
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for game in g {
            match game.1 {
                BallColor::Red => red = game.0,
                BallColor::Blue => blue = game.0,
                BallColor::Green => green = game.0,
            }
        }
        let game: Game = Game::new(red, green, blue);
        games.push(game);
        if input.len() == 0 {
            break;
        }
        (input, _) = tag("; ")(input)?;
    }
    Ok((input, games))
}

fn parse_balls(input: &str) -> IResult<&str, Vec<(u32, BallColor)>> {
    let mut balls: Vec<(u32, BallColor)> = vec![];
    let mut input = input;
    loop {
        let number;
        (input, number) = map_res(bytes::complete::take_till(|c| c == ' '), |s: &str| {
            s.parse::<u32>()
        })(input)?;
        (input, _) = tag(" ")(input)?;
        let ball_color;
        (input, ball_color) = take_till(|c| c == ',')(input)?;
        let ball = match ball_color {
            "red" => BallColor::Red,
            "blue" => BallColor::Blue,
            "green" => BallColor::Green,
            _ => panic!(),
        };
        balls.push((number, ball));
        if input.len() == 0 {
            break;
        }
        (input, _) = tag(", ")(input)?;
    }
    Ok((input, balls))
}
