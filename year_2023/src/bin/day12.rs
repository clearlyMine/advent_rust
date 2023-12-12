use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::{collections::HashMap, time::Instant};

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day12.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day12_sample.txt");

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
        .lines()
        .par_bridge()
        .into_par_iter()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(l, n)| {
            let damaged_list = n
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect_vec();

            let mut springs = l.chars().collect_vec();
            springs.push('.');

            (springs, damaged_list)
        })
        .map(|(springs, damaged_list)| {
            let mut cache: HashMap<(Vec<char>, usize, usize), usize> = HashMap::new();
            get_number_of_possible_solutions(&springs, &damaged_list, 0, &mut cache)
        })
        .sum()
}

fn process_part_2(input: &str) -> usize {
    input
        .lines()
        .par_bridge()
        .into_par_iter()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(l, n)| {
            //expand
            let mut damaged_list: String = n.to_string();
            let mut springs = l.to_string();
            for _ in 0..4 {
                damaged_list += ",";
                damaged_list += n;
                springs += "?";
                springs += l;
            }

            (springs, damaged_list)
        })
        .map(|(springs, damaged_list)| {
            let damaged_list: Vec<usize> = damaged_list
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let mut springs = springs.chars().collect_vec();
            springs.push('.');

            (springs, damaged_list)
        })
        .map(|(springs, damaged_list)| {
            let mut cache: HashMap<(Vec<char>, usize, usize), usize> = HashMap::new();
            get_number_of_possible_solutions(&springs, &damaged_list, 0, &mut cache)
        })
        .sum()
}

fn get_number_of_possible_solutions(
    word: &[char],
    constraints: &[usize],
    damaged_in_group: usize,
    cache: &mut HashMap<(Vec<char>, usize, usize), usize>,
) -> usize {
    if let Some(x) = cache.get(&(word.to_vec(), constraints.len(), damaged_in_group)) {
        return *x;
    }
    let res = get_number_of_possible_solutions_memoized(word, constraints, damaged_in_group, cache);
    cache.insert((word.to_vec(), constraints.len(), damaged_in_group), res);
    res
}

fn get_number_of_possible_solutions_memoized(
    word: &[char],
    constraints: &[usize],
    damaged_in_group: usize,
    cache: &mut HashMap<(Vec<char>, usize, usize), usize>,
) -> usize {
    if word.len() == 0 {
        if constraints.len() == 0 && damaged_in_group == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    let damaged = word.into_iter().filter(|c| **c == '#').count();
    let total_damaged_remaining = constraints.iter().fold(0, |acc, c| acc + c);
    if damaged > total_damaged_remaining {
        return 0;
    }

    match word[0] {
        '?' => ['.', '#'].into_iter().fold(0, |acc, new_letter| {
            let mut new_word: Vec<char> = vec![new_letter];
            new_word.append(&mut word[1..].to_vec());
            acc + get_number_of_possible_solutions(&new_word, constraints, damaged_in_group, cache)
        }),
        '.' => {
            let mut t = 0;
            if constraints.len() > 0 && constraints[0] == damaged_in_group {
                t = get_number_of_possible_solutions(&word[1..], &constraints[1..], 0, cache);
            }
            if damaged_in_group == 0 {
                t += get_number_of_possible_solutions(&word[1..], &constraints, 0, cache)
            }
            t
        }
        '#' => {
            get_number_of_possible_solutions(&word[1..], &constraints, damaged_in_group + 1, cache)
        }
        _ => panic!("WTF"),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 21)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 7_599)
    }

    #[test]
    fn part_2_sample_1() {
        assert_eq!(process_part_2(SAMPLE), 525_152)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 15_454_556_629_917)
    }
}
