use itertools::Itertools;
use std::time::Instant;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day11.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day11_sample.txt");

fn main() {
    let time_start = Instant::now();
    let res1 = process_part_1(INPUT);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(INPUT, 1_000_000);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let galaxies = parse_galaxies(input, 2);
    let mut total = 0;

    for (i, gal) in galaxies.clone().into_iter().enumerate() {
        for next_gal in &galaxies[i + 1..] {
            total += get_manhattan_distance(gal, *next_gal);
        }
    }
    total
}

fn process_part_2(input: &str, expansion: usize) -> usize {
    let galaxies = parse_galaxies(input, expansion);
    let mut total = 0;

    for (i, gal) in galaxies.clone().into_iter().enumerate() {
        for next_gal in &galaxies[i + 1..] {
            total += get_manhattan_distance(gal, *next_gal);
        }
    }
    total
}

fn get_manhattan_distance(first: Coord, second: Coord) -> usize {
    first.row.abs_diff(second.row) + first.col.abs_diff(second.col)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Coord {
        Coord { row, col }
    }
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines().collect_vec();
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
    let _ = lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, char)| {
            grid[row][col] = char;
        });
    });
    grid
}

fn parse_galaxies(input: &str, expansion: usize) -> Vec<Coord> {
    let grid = parse_grid(input);
    let mut galaxies: Vec<Coord> = vec![];

    grid.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, char)| {
            if char == &'#' {
                galaxies.push(Coord::new(row, col));
            }
        })
    });

    let empty_rows = grid
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(row, line)| {
            if line.iter().any(|char| char == &'#') {
                None
            } else {
                Some(row)
            }
        })
        .collect_vec();

    let mut empty_cols: Vec<usize> = vec![];
    for col in 0..grid[0].len() {
        let mut empty_col = true;
        for row in 0..grid.len() {
            if grid[row][col] == '#' {
                empty_col = false;
                break;
            }
        }
        if empty_col {
            empty_cols.push(col);
        }
    }

    galaxies.iter_mut().for_each(|gal| {
        let (row, col) = (gal.row, gal.col);
        let rows_added = empty_rows.iter().filter(|r| r < &&row).count() * (expansion - 1);
        let cols_added = empty_cols.iter().filter(|c| c < &&col).count() * (expansion - 1);
        *gal = Coord::new(row + rows_added, col + cols_added);
    });

    galaxies
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 374)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 9_623_138)
    }

    #[test]
    fn part_2_sample_1() {
        assert_eq!(process_part_2(SAMPLE, 10), 1_030)
    }

    #[test]
    fn part_2_sample_2() {
        assert_eq!(process_part_2(SAMPLE, 100), 8_410)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT, 1_000_000), 726820169514)
    }
}
