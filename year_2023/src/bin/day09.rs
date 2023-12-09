use std::time::Instant;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day09.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day09_sample.txt");

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

fn process_part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold(0, |sum, numbers| sum + calculate_next_number(&numbers))
}

fn process_part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold(0, |sum, numbers| sum + calculate_previous_number(&numbers))
}

fn calculate_next_number(numbers: &Vec<i32>) -> i32 {
    calculate_next(&numbers) + numbers.last().unwrap()
}

fn calculate_next(numbers: &Vec<i32>) -> i32 {
    if !numbers.into_iter().any(|num| *num != 0) {
        return 0;
    }
    let mut differences: Vec<i32> = vec![];
    for num in numbers.windows(2) {
        differences.push(num[1] - num[0]);
    }
    calculate_next(&differences) + differences.last().unwrap()
}

fn calculate_previous_number(numbers: &Vec<i32>) -> i32 {
    let x = calculate_previous(&numbers);
    numbers.first().unwrap() - x
}

fn calculate_previous(numbers: &Vec<i32>) -> i32 {
    if !numbers.into_iter().any(|num| *num != 0) {
        return 0;
    }
    let mut differences: Vec<i32> = vec![];
    for num in numbers.windows(2) {
        differences.push(num[1] - num[0]);
    }
    let x = calculate_previous(&differences);
    differences.first().unwrap() - x
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 114)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 1584748274)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 2)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 1026)
    }
}
