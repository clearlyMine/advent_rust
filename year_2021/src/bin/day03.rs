use itertools::Itertools;
use std::time::Instant;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day03.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day03_sample.txt");

fn main() {
    let time_start = Instant::now();
    let res1 = process_part_1(SAMPLE);
    println!("Part 1_sample: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res1 = process_part_1(INPUT);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(SAMPLE);
    println!("Part 2_sample: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(INPUT);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|x| x.chars().map(|ch| ch == '1').collect_vec())
        .collect::<Vec<Vec<bool>>>();
    let mut res = vec![0; lines[0].len()];
    lines.iter().for_each(|line| {
        for (i, c) in line.iter().enumerate() {
            if *c {
                res[i] += 1;
            } else {
                res[i] -= 1;
            }
        }
    });
    let gamma: Vec<bool> = res.iter().map(|x| *x >= 0).collect_vec();
    let epsilon: Vec<bool> = gamma.iter().map(|x| !x).collect_vec();

    bool_vec_to_decimal(&gamma) * bool_vec_to_decimal(&epsilon)
}

fn process_part_2(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|x| x.chars().map(|x| x == '1').collect_vec())
        .collect::<Vec<_>>();
    let mut index = 0;
    let mut list = lines.clone();
    while list.len() > 1 {
        let most_frequent = get_most_frequent_in_position(&list, index);
        list = list
            .into_iter()
            .filter(|x| x[index] == most_frequent)
            .collect_vec();
        index += 1;
    }
    let oxygen_generating_rating = bool_vec_to_decimal(&list[0]);

    index = 0;
    list = lines.clone();
    while list.len() > 1 {
        let least_frequent = get_least_frequent_in_position(&list, index);
        list = list
            .into_iter()
            .filter(|x| x[index] == least_frequent)
            .collect_vec();
        index += 1;
    }
    let co2_scrubber_rating = bool_vec_to_decimal(&list[0]);
    oxygen_generating_rating * co2_scrubber_rating
}

fn bool_vec_to_decimal(bools: &Vec<bool>) -> u32 {
    bools.iter().rev().enumerate().fold(0, |mut acc, (i, num)| {
        if *num {
            acc += 1 << i;
        }
        acc
    })
}

fn get_most_frequent_in_position(inp: &Vec<Vec<bool>>, pos: usize) -> bool {
    let mut res = 0;
    inp.iter().for_each(|chars| {
        if chars[pos] {
            res += 1;
        } else {
            res -= 1;
        }
    });
    res >= 0
}

fn get_least_frequent_in_position(inp: &Vec<Vec<bool>>, pos: usize) -> bool {
    let mut res = 0;
    inp.iter().for_each(|chars| {
        if chars[pos] {
            res += 1;
        } else {
            res -= 1;
        }
    });
    res < 0
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 198)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 693486)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 230)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 3379326)
    }
}
