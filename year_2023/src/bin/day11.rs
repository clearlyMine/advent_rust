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

fn process_part_1(input: &str) -> u32 {
    let galaxies = &parse_galaxies(input, 2);

    galaxies
        .into_iter()
        .enumerate()
        .fold(0, |mut acc, (i, gal)| {
            for next_gal in &galaxies[i + 1..] {
                acc += gal.manhattan_distance(next_gal);
            }
            acc
        })
}

fn process_part_2(input: &str, expansion: u32) -> u64 {
    let galaxies = &parse_galaxies(input, expansion);

    galaxies
        .into_iter()
        .enumerate()
        .fold(0, |mut acc, (i, gal)| {
            for next_gal in &galaxies[i + 1..] {
                acc += gal.manhattan_distance(next_gal) as u64;
            }
            acc
        })
}

#[derive(Clone)]
struct Coord {
    row: u32,
    col: u32,
}

impl Coord {
    fn new(row: u32, col: u32) -> Coord {
        Coord { row, col }
    }

    fn manhattan_distance(&self, other: &Coord) -> u32 {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

fn parse_grid(input: &str) -> (Vec<Vec<char>>, Vec<Coord>) {
    let lines = input.lines().collect_vec();
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
    let mut galaxies: Vec<Coord> = vec![];
    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, char)| {
            if char == '#' {
                grid[row][col] = char;
                galaxies.push(Coord::new(row as u32, col as u32));
            }
        });
    });
    (grid, galaxies)
}

fn parse_galaxies(input: &str, expansion: u32) -> Vec<Coord> {
    let (grid, mut galaxies) = parse_grid(input);

    let empty_rows = grid
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(row, line)| {
            if line.iter().any(|char| char == &'#') {
                None
            } else {
                Some(row as u32)
            }
        })
        .collect::<Vec<u32>>();

    let mut empty_cols: Vec<u32> = vec![];
    for col in 0..grid[0].len() {
        let mut empty_col = true;
        for row in 0..grid.len() {
            if grid[row][col] == '#' {
                empty_col = false;
                break;
            }
        }
        if empty_col {
            empty_cols.push(col as u32);
        }
    }

    galaxies.iter_mut().for_each(|gal| {
        let (row, col) = (gal.row, gal.col);
        let rows_added = empty_rows.iter().filter(|r| r < &&row).count() as u32 * (expansion - 1);
        let cols_added = empty_cols.iter().filter(|c| c < &&col).count() as u32 * (expansion - 1);
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
        assert_eq!(process_part_2(INPUT, 1_000_000), 726_820_169_514)
    }
}
