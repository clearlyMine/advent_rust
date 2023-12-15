use itertools::Itertools;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::{collections::HashMap, time::Instant};

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day15.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day15_sample.txt");

fn main() {
    let time_start = Instant::now();
    let res1 = process_part_1(SAMPLE);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(INPUT);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    input
        .trim()
        .split(",")
        .map(|s| {
            s.chars().fold(0, |acc, c| {
                let mut acc = acc + c as usize;
                acc *= 17;
                acc % 256
            })
        })
        .sum()
}

fn process_part_2(input: &str) -> usize {
    let mut hm: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    input.trim().split(",").for_each(|s| {
        if s.contains("=") {
            let (label, focal_length) = s.split_once("=").unwrap();
            let box_num = label.chars().fold(0, |acc, c| {
                let mut acc = acc + c as usize;
                acc *= 17;
                acc % 256
            });
            let focal_length = focal_length.parse::<usize>().unwrap();
            //find if same label is present
            let mut found = false;
            for x in hm[box_num].iter_mut() {
                if x.0 == label {
                    *x = (label.to_string(), focal_length);
                    found = true;
                    // println!(
                    //     "replacing {}:{} with {}:{} in box:{}",
                    //     x.0, x.1, label, focal_length, box_num
                    // );
                    break;
                }
            }
            if !found {
                // println!("adding {}:{} in box:{}", label, focal_length, box_num);
                hm[box_num].push((label.to_string(), focal_length));
            }
        } else if s.contains("-") {
            let (label, _focal_length) = s.split_once("-").unwrap();
            let box_num = label.chars().fold(0, |acc, c| {
                let mut acc = acc + c as usize;
                acc *= 17;
                acc % 256
            });
            let mut found_at: isize = -1;
            for (i, x) in hm[box_num].iter().enumerate() {
                if x.0 == label {
                    found_at = i as isize;
                    break;
                }
            }
            if found_at > -1 {
                // println!("removing {}:{} from box:{}", label, _focal_length, box_num);
                hm[box_num].remove(found_at as usize);
            }
        }
    });

    // dbg!(hm.clone());
    hm.iter()
        .enumerate()
        .map(|(box_index, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_index, (_, focal_length))| {
                    (box_index + 1) * (lens_index + 1) * focal_length
                })
                .sum::<usize>()
        })
        .sum()
}

#[derive(PartialEq, Eq, Hash)]
struct Grid(Vec<Vec<Land>>);

struct Row(Vec<Land>);

impl Hash for Row {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Land>>) {
    let mut f: String = "".to_string();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = match grid[i][j] {
                Land::Ground => '.',
                Land::MovableRock => 'O',
                _ => '#',
            };
            f = format!("{}{}", f, c);
        }
        f = format!("{}\n", f);
    }
    println!("{}", f);
}

fn get_col(grid: &[Vec<Land>], col: usize) -> Vec<Land> {
    grid.iter().map(|row| row[col]).collect::<Vec<Land>>()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Land {
    Ground,
    MovableRock,
    ImmovableRock,
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
