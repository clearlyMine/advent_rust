use itertools::Itertools;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{alpha1, digit1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::hash::{Hash, Hasher};
use std::{collections::HashMap, collections::HashSet, time::Instant};

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day_sample.txt");

fn main() {
    let time_start = Instant::now();
    let res1 = process_part_1(SAMPLE);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(SAMPLE);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let grid = parse_grid(input);
    let width = grid[0].len();
    let height = grid.len();
    0
}

fn process_part_2(input: &str) -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn get_south(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn get_east(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col + 1,
        }
    }

    fn get_west(&self) -> Option<Coord> {
        if self.col == 0 {
            return None;
        }
        Some(Coord {
            row: self.row,
            col: self.col - 1,
        })
    }

    fn get_north(&self) -> Option<Coord> {
        if self.row == 0 {
            return None;
        }
        Some(Coord {
            row: self.row - 1,
            col: self.col,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Directions {
    North,
    South,
    East,
    West,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

#[derive(PartialEq, Eq, Hash)]
struct Grid(Vec<Vec<Land>>);

struct Row(Vec<Land>);

impl Hash for Row {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Land {
    Ground,
    MovableRock,
    ImmovableRock,
}

fn parse_line(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, _) = take_till(|c| c == ':')(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, games) = separated_list1(tag("; "), parse_game)(input)?;
    Ok((input, games))
}

fn parse_line(input: &str) -> IResult<&str, [HashSet<u32>; 2]> {
    let (input, _) = take_till(|c| c == ':')(input)?;
    let (input, _) = tag(": ")(input)?;
    let (winning, our) = input.split_once('|').unwrap();
    let winning_numbers = winning.split(' ').collect::<HashSet<&str>>();
    let winning_numbers = winning_numbers
        .into_iter()
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
    // dbg!(winning_numbers);
    let our_numbers = our.split(' ').collect::<HashSet<&str>>();
    let our_numbers = our_numbers
        .into_iter()
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
    // dbg!(our_numbers.clone());

    Ok(("", [winning_numbers, our_numbers]))
}

fn parse_grid(input: &str) -> Vec<Vec<Land>> {
    let lines = input.lines().collect_vec();
    let mut grid: Vec<Vec<Land>> = vec![vec![Land::Ground; lines[0].len()]; lines.len()];
    lines.iter().enumerate().for_each(|(row, line)| {
        for (col, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            grid[row][col] = match char {
                '#' => Land::ImmovableRock,
                'O' => Land::MovableRock,
                _ => panic!("WTF"),
            }
        }
    });
    grid
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

fn tilt_east(grid: &mut Vec<Vec<Land>>, height: usize, width: usize) {
    for row in 0..height {
        let row_contents = &*grid[row].clone();
        let mut current_right = width - 1;
        for (i, char) in row_contents.iter().rev().enumerate() {
            let col = width - i - 1;
            match char {
                Land::MovableRock => {
                    grid[row][col] = Land::Ground;
                    grid[row][current_right] = Land::MovableRock;
                    if current_right == 0 {
                        break;
                    }
                    current_right -= 1;
                }
                Land::ImmovableRock => {
                    if col == 0 {
                        break;
                    }
                    current_right = col - 1;
                }
                _ => {}
            }
        }
    }
    // print_grid(&grid);
}

fn calculate_load_on_north_beam(grid: &[Vec<Land>]) -> usize {
    let height = grid.len();
    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|c| c == &&Land::MovableRock).count() * (height - i))
        .sum()
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    let mut f: String = "".to_string();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            f = format!("{}{}", f, grid[row][col]);
        }
        f = format!("{}\n", f);
    }
    println!("{}", f);
}

fn get_col(grid: &[Vec<char>], col: usize) -> Vec<char> {
    grid.iter().map(|row| row[col]).collect::<Vec<char>>()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 0)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 0)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 0)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 0)
    }
}
