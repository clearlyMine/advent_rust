use itertools::Itertools;
use std::time::Instant;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day13.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day13_sample.txt");

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
    input
        .split("\n\n")
        .into_iter()
        .map(|grid| parse_grid(grid))
        .map(|grid| {
            let ret = process_grid(grid);
            ret
        })
        .sum()
}

fn process_part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .into_iter()
        .map(|grid| parse_grid(grid))
        .map(|grid| {
            let ret = process_grid_with_diffs(grid);
            return ret;
        })
        .sum()
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<u8>>) {
    let mut f: String = "".to_string();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = if grid[i][j] == 0 { '.' } else { '#' };
            f = format!("{}{}", f, c);
        }
        f = format!("{}\n", f);
    }
    println!("{}", f);
}

fn process_grid(grid: Vec<Vec<u8>>) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    'main: for col in 1..width {
        let mut left = col - 1;
        let mut right = col;
        while right < width {
            if get_col(&grid, left) != get_col(&grid, right) {
                continue 'main;
            }
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }
        return col;
    }

    'main: for row in 1..height {
        let mut top = row - 1;
        let mut bottom = row;
        while bottom < height {
            if grid[top] != grid[bottom] {
                continue 'main;
            }
            if top == 0 {
                break;
            }
            top -= 1;
            bottom += 1;
        }
        return row * 100;
    }
    0
}

fn process_grid_with_diffs(grid: Vec<Vec<u8>>) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    'main: for col in 1..width {
        let mut left = col - 1;
        let mut right = col;
        let mut diffs = 0;
        while right < width {
            if diffs > 1 {
                continue 'main;
            }
            diffs += compare_cols(&grid, left, right);
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }
        if diffs == 1 {
            return col;
        }
    }

    'main: for row in 1..height {
        let mut top = row - 1;
        let mut bottom = row;
        let mut diffs = 0;
        while bottom < height {
            if diffs > 1 {
                continue 'main;
            }
            diffs += compare_rows(&grid, top, bottom);
            if top == 0 {
                break;
            }
            top -= 1;
            bottom += 1;
        }
        if diffs == 1 {
            return row * 100;
        }
    }
    0
}

fn compare_cols(grid: &[Vec<u8>], t: usize, b: usize) -> usize {
    get_col(grid, t)
        .into_iter()
        .zip(get_col(grid, b))
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn compare_rows(grid: &Vec<Vec<u8>>, l: usize, r: usize) -> usize {
    grid[l]
        .clone()
        .into_iter()
        .zip(grid[r].clone())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}
fn get_col(grid: &[Vec<u8>], col: usize) -> Vec<u8> {
    grid.iter().map(|row| row[col]).collect::<Vec<u8>>()
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    let lines = input.lines().collect_vec();
    let mut grid: Vec<Vec<u8>> = vec![vec![0; lines[0].len()]; lines.len()];
    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, char)| {
            if char == '#' {
                grid[row][col] = 1;
            }
        });
    });
    grid
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 405)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 30_575)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 400)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 37_478)
    }
}
