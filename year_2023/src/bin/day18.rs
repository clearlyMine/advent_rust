use std::hash::{Hash, Hasher};
use std::{collections::HashMap, collections::HashSet, time::Instant};

use itertools::Itertools;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day18.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day18_sample.txt");

fn main() {
    let time_start = Instant::now();
    let res1 = process_part_1(INPUT);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(SAMPLE);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let dig_plan: Vec<Dig> = input.lines().map(|line| parse_line(line)).collect();
    let total_to_be_dug: usize = dig_plan.iter().map(|d| d.depth).sum();
    dbg!(total_to_be_dug);

    let start: (isize, isize) = (0, 0);
    let mut cur = start;
    let mut dug_tiles = vec![];
    for Dig {
        direction,
        depth,
        color,
    } in dig_plan
    {
        for _ in 0..depth {
            cur = match direction {
                Direction::North => (cur.0 - 1, cur.1),
                Direction::East => (cur.0, cur.1 + 1),
                Direction::West => (cur.0, cur.1 - 1),
                Direction::South => (cur.0 + 1, cur.1),
            };
            dug_tiles.push((cur, color.clone()));
        }
    }
    // dbg!(dug_tiles.clone());
    let dug_cloned = dug_tiles.clone();
    let rows = dug_cloned.iter().map(|((row, _), _)| row);
    let min_row = rows.clone().min().unwrap();
    let max_row = rows.max().unwrap();

    let cols = dug_cloned.iter().map(|((_, col), _)| col);
    let min_col = cols.clone().min().unwrap();
    let max_col = cols.max().unwrap();

    let height = (max_row - min_row + 1) as usize;
    let width = (max_col - min_col + 1) as usize;
    let mut grid = vec![vec![(false, "".to_string()); width]; height];
    for ((row, col), color) in dug_tiles.into_iter() {
        let next_row = (row as isize + min_row.abs()) as usize;
        let next_col = (col as isize + min_col.abs()) as usize;

        grid[next_row][next_col] = (true, color);
    }
    let tiles_outside = find_tiles_outside_trench(&grid);

    height * width - tiles_outside
}

fn process_part_2(input: &str) -> usize {
    let dig_plans = input
        .lines()
        .map(|line| {
            let hex = line.split_whitespace().collect::<Vec<_>>()[2];
            parse_hex(hex)
        })
        .collect::<Vec<NewDig>>();
    // dbg!(dig_plans.clone());
    let tiles_to_dig = dig_plans.clone().iter().fold(
        0,
        |acc,
         NewDig {
             direction: _,
             length,
         }| acc + length,
    );
    dbg!(tiles_to_dig);

    // let mut dug_tiles = HashSet::new();
    let mut dug_tiles = vec![];
    let mut cur = CoordWithNegative { row: 0, col: 0 };
    for NewDig { direction, length } in dig_plans {
        for _ in 0..length {
            cur = match direction {
                Direction::North => cur.get_north(),
                Direction::South => cur.get_south(),
                Direction::East => cur.get_east(),
                Direction::West => cur.get_west(),
            };
            // dug_tiles.insert(cur);
            dug_tiles.push(cur);
        }
    }

    let dug_cloned = dug_tiles.clone();
    let rows = dug_cloned
        .iter()
        .map(|CoordWithNegative { row, col: _ }| row);
    let min_row = rows.clone().min().unwrap();
    let max_row = rows.max().unwrap();
    dbg!(min_row, max_row);

    let cols = dug_cloned
        .iter()
        .map(|CoordWithNegative { row: _, col }| col);
    let min_col = cols.clone().min().unwrap();
    let max_col = cols.max().unwrap();
    dbg!(min_col, max_col);

    //shoelace
    let mut area: isize = 0;
    let n = dug_tiles.len() as isize;
    dbg!(n);
    for w in dug_tiles.windows(2) {
        area += ((w[0].row) * (w[1].col)) as isize;
        area -= ((w[0].col) * (w[1].row)) as isize;
        area += w[0].row.abs_diff(w[1].row) as isize;
        area += w[0].col.abs_diff(w[1].col) as isize;
    }
    let area = isize::abs(area) / 2;
    // let area = area - (n / 2) + 1;
    dbg!(area);
    //Pick's theorem
    // let area = area + (n / 2) + 1;
    let area = area + n + 1;
    area as usize

    //
    // let height = (max_row - min_row + 1) as usize;
    // let width = (max_col - min_col + 1) as usize;
    //
    // let mut tiles_outside = HashSet::new();
    // let mut visited = HashSet::new();
    //
    // for row in *min_row..=*max_row {
    //     for col in [*min_col, *max_col] {
    //         get_all_tiles_outside(
    //             row,
    //             col,
    //             &dug_tiles,
    //             &mut tiles_outside,
    //             min_row,
    //             max_row,
    //             min_col,
    //             max_col,
    //             &mut visited,
    //         );
    //         dbg!(tiles_outside.len());
    //     }
    // }
    //
    // for col in *min_col..=*max_col {
    //     for row in [*min_row, *max_row] {
    //         get_all_tiles_outside(
    //             row,
    //             col,
    //             &dug_tiles,
    //             &mut tiles_outside,
    //             min_row,
    //             max_row,
    //             min_col,
    //             max_col,
    //             &mut visited,
    //         );
    //         dbg!(tiles_outside.len());
    //     }
    // }
    // // dbg!(tiles_outside.clone());
    // // let tiles_outside = tiles_outside.len() - (height + 2) * 2 - (width + 2) * 2 + 4;
    //
    // height * width - tiles_outside.len()
}

// Function to find the HCF of two numbers using Euclidean algorithm
fn hcf(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        hcf(b, a % b)
    }
}

// Function to find the HCF of multiple numbers
fn find_hcf(numbers: &Vec<usize>) -> usize {
    // Initialize the result with the HCF of the first two numbers
    let mut result = hcf(numbers[0], numbers[1]);

    // Iterate through the rest of the numbers
    for &num in numbers.iter().skip(2) {
        result = hcf(result, num);
    }

    result
}

fn get_all_tiles_outside(
    row: isize,
    col: isize,
    dug_tiles: &HashSet<CoordWithNegative>,
    tiles_outside: &mut HashSet<CoordWithNegative>,
    min_row: &isize,
    max_row: &isize,
    min_col: &isize,
    max_col: &isize,
    visited: &mut HashSet<CoordWithNegative>,
) {
    let mut queue = vec![CoordWithNegative::new(row, col)];

    while let Some(cur) = queue.pop() {
        // dbg!(cur.clone());
        if dug_tiles.contains(&cur) {
            continue;
        }
        tiles_outside.insert(cur);
        for next in [
            cur.get_north(),
            cur.get_south(),
            cur.get_west(),
            cur.get_east(),
        ] {
            // dbg!(next.clone());
            if next.row >= min_row - 1
                && next.row <= max_row + 1
                && next.col >= min_col - 1
                && next.col <= max_col + 1
                && !visited.contains(&next)
            {
                queue.push(next);
                visited.insert(next);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct NewDig {
    direction: Direction,
    length: usize,
}

fn parse_hex(hex: &str) -> NewDig {
    let hex = hex.to_string();
    // dbg!(hex.clone());
    let length = &hex[2..7];
    // dbg!(length.clone());
    let length = usize::from_str_radix(length, 16).unwrap();
    // dbg!(length.clone());
    let direction = hex[7..8].to_string();
    // dbg!(direction.clone());
    let direction = match direction.as_str() {
        "0" => Direction::East,
        "1" => Direction::South,
        "2" => Direction::West,
        "3" => Direction::North,
        _ => panic!("Unknown direction"),
    };
    // dbg!(direction.clone());
    NewDig { direction, length }
}

fn find_tiles_outside_trench(grid: &Vec<Vec<(bool, String)>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    //pad grid
    let mut new_grid = vec![vec![(false, "".to_string()); width + 2]; height + 2];
    let height = height + 2;
    let width = width + 2;

    for row in 0..height - 2 {
        for col in 0..width - 2 {
            new_grid[row + 1][col + 1] = grid[row][col].clone();
        }
    }

    //DFS search all tiles outside
    let mut tiles_outside = HashSet::new();
    let mut queue: Vec<Coord> = vec![Coord { row: 0, col: 0 }];
    let mut visited: HashSet<Coord> = HashSet::new();

    while let Some(cur) = queue.pop() {
        if new_grid[cur.row][cur.col].0 {
            continue;
        }
        tiles_outside.insert(cur);
        let directions = vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        for direction in directions {
            if let Some(next) = match direction {
                Direction::South => cur.get_south(),
                Direction::North => cur.get_north(),
                Direction::West => cur.get_west(),
                Direction::East => cur.get_east(),
            } {
                if next.row < height && next.col < width && !visited.contains(&next) {
                    queue.push(next);
                    visited.insert(next);
                }
            }
        }
    }

    tiles_outside.len() - height * 2 - width * 2 + 4
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CoordWithNegative {
    row: isize,
    col: isize,
}

impl CoordWithNegative {
    fn new(row: isize, col: isize) -> CoordWithNegative {
        CoordWithNegative { row, col }
    }
    fn get_south(&self) -> CoordWithNegative {
        CoordWithNegative {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn get_east(&self) -> CoordWithNegative {
        CoordWithNegative {
            row: self.row,
            col: self.col + 1,
        }
    }

    fn get_west(&self) -> CoordWithNegative {
        CoordWithNegative {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn get_north(&self) -> CoordWithNegative {
        CoordWithNegative {
            row: self.row - 1,
            col: self.col,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn get_south(&self) -> Option<Coord> {
        Some(Coord {
            row: self.row + 1,
            col: self.col,
        })
    }

    fn get_east(&self) -> Option<Coord> {
        Some(Coord {
            row: self.row,
            col: self.col + 1,
        })
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
enum Direction {
    North,
    South,
    East,
    West,
}

struct Dig {
    direction: Direction,
    depth: usize,
    color: String,
}

fn parse_line(input: &str) -> Dig {
    let dig: Vec<&str> = input.split_whitespace().take(3).collect();
    let (direction, depth, color) = (dig[0], dig[1], dig[2]);
    let direction = match direction {
        "R" => Direction::East,
        "L" => Direction::West,
        "U" => Direction::North,
        "D" => Direction::South,
        _ => panic!("Unknown movement"),
    };
    let depth = depth.parse::<usize>().unwrap();
    let color = color.to_string();
    let color = color[1..color.len() - 1].to_string();
    Dig {
        direction,
        depth,
        color,
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<(bool, String)>>) {
    let mut f: String = "".to_string();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let c = if grid[row][col].0 { '#' } else { '.' };
            f = format!("{}{}", f, c);
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
        assert_eq!(process_part_1(SAMPLE), 62)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 62_500)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 952_408_144_115)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 122109860712709);
    }

    #[test]
    fn test_find_hcf() {
        let v = vec![20, 25, 385];
        assert_eq!(find_hcf(&v), 5);

        let v = vec![2, 25, 385];
        assert_eq!(find_hcf(&v), 1);

        let v = vec![500, 25, 375];
        assert_eq!(find_hcf(&v), 25);
    }
}
