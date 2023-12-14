use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day14.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day14_sample.txt");

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
    let grid = parse_grid(input);
    let width = grid[0].len();

    let mut total = 0;
    for col in 0..width {
        let column = get_col(&grid, col);
        let length = column.len();
        let mut current_top = 0;
        for (i, char) in column.iter().enumerate() {
            match char {
                RockType::MovableRock => {
                    total += length - current_top;
                    current_top += 1;
                }
                RockType::ImmovableRock => current_top = i + 1,
                _ => {}
            }
        }
    }

    total
}

fn process_part_2(input: &str) -> usize {
    let mut grid = parse_grid(input);

    let mut history: HashMap<String, usize> = HashMap::new();
    for i in 1..=1_000_000_000 {
        // if i % 1_000_000 == 0 {
        //     dbg!(i);
        // }
        grid = spin_cycle(grid);
        if let Some(loop_starts_at) = history.get(&get_key(&grid)) {
            let loop_size = i - loop_starts_at;
            // println!(
            //     "loop found!! Starts at:{} of size:{}",
            //     loop_starts_at, loop_size
            // );
            let target = loop_starts_at + (1_000_000_000 - loop_starts_at) % loop_size;
            // dbg!(target);
            // print_grid(&grid);
            for (k, v) in history {
                if v == target {
                    grid = parse_grid(k.as_str());
                    // print_grid(&grid);
                    break;
                }
            }
            break;
        }
        history.insert(get_key(&grid), i);
    }
    load_on_north_beam(&grid)
}

fn get_key(grid: &Vec<Vec<RockType>>) -> String {
    let mut s = "".to_string();
    for row in grid {
        for char in row {
            let c = match char {
                RockType::Ground => '.',
                RockType::MovableRock => 'O',
                _ => '#',
            };
            s = format!("{}{}", s, c);
        }
        s = format!("{}\n", s);
    }
    s
}

fn spin_cycle(grid: Vec<Vec<RockType>>) -> Vec<Vec<RockType>> {
    let mut grid = grid;
    let width = grid[0].len();
    let height = grid.len();
    // println!("\n\n---------------");
    //tilt north
    for col in 0..width {
        let column = get_col(&grid, col);
        let mut current_top = 0;
        for (row, char) in column.iter().enumerate() {
            match char {
                RockType::MovableRock => {
                    grid[row][col] = RockType::Ground;
                    grid[current_top][col] = RockType::MovableRock;
                    current_top += 1;
                }
                RockType::ImmovableRock => current_top = row + 1,
                _ => {}
            }
        }
    }
    // print_grid(&grid);

    //tilt west
    for row in 0..height {
        let row_contents = &grid[row].clone();
        let mut current_left = 0;
        for (col, char) in row_contents.iter().enumerate() {
            match char {
                RockType::MovableRock => {
                    grid[row][col] = RockType::Ground;
                    grid[row][current_left] = RockType::MovableRock;
                    current_left += 1;
                }
                RockType::ImmovableRock => current_left = col + 1,
                _ => {}
            }
        }
    }
    // print_grid(&grid);

    //tilt south
    for col in 0..width {
        let column = get_col(&grid, col);
        let mut current_bottom = height - 1;
        for (i, char) in column.iter().rev().enumerate() {
            let row = height - i - 1;
            match char {
                RockType::MovableRock => {
                    grid[row][col] = RockType::Ground;
                    grid[current_bottom][col] = RockType::MovableRock;
                    current_bottom -= 1;
                }
                RockType::ImmovableRock => current_bottom = row - 1,
                _ => {}
            }
        }
    }
    // print_grid(&grid);

    //tilt east
    for row in 0..height {
        let row_contents = &grid[row].clone();
        let mut current_right = width - 1;
        for (i, char) in row_contents.iter().rev().enumerate() {
            let col = width - i - 1;
            match char {
                RockType::MovableRock => {
                    grid[row][col] = RockType::Ground;
                    grid[row][current_right] = RockType::MovableRock;
                    current_right -= 1;
                }
                RockType::ImmovableRock => current_right = col - 1,
                _ => {}
            }
        }
    }
    // print_grid(&grid);
    grid
}

fn load_on_north_beam(grid: &[Vec<RockType>]) -> usize {
    let mut total = 0;
    let height = grid.len();
    for (i, row) in grid.iter().enumerate() {
        let mut movable_rocks = 0;
        for char in row {
            if char == &RockType::MovableRock {
                movable_rocks += 1;
            }
        }
        total += movable_rocks * (height - i);
    }
    total
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<RockType>>) {
    let mut f: String = "".to_string();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = match grid[i][j] {
                RockType::Ground => '.',
                RockType::MovableRock => 'O',
                _ => '#',
            };
            f = format!("{}{}", f, c);
        }
        f = format!("{}\n", f);
    }
    println!("{}", f);
}

fn get_col(grid: &[Vec<RockType>], col: usize) -> Vec<RockType> {
    grid.iter().map(|row| row[col]).collect::<Vec<RockType>>()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RockType {
    Ground,
    MovableRock,
    ImmovableRock,
}

fn parse_grid(input: &str) -> Vec<Vec<RockType>> {
    let lines = input.lines().collect_vec();
    let mut grid: Vec<Vec<RockType>> = vec![vec![RockType::Ground; lines[0].len()]; lines.len()];
    lines.iter().enumerate().for_each(|(row, line)| {
        for (col, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            grid[row][col] = match char {
                '#' => RockType::ImmovableRock,
                'O' => RockType::MovableRock,
                _ => panic!("WTF"),
            }
        }
    });
    grid
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 136)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 109_833)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 64)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 99_875)
    }
}
