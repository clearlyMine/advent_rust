use std::time::Instant;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day01.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day01_sample.txt");

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
    let res2 = process_part_2(INPUT);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    lines.windows(2).fold(0, |acc, pair| {
        let (first, second) = (
            pair[0].parse::<u32>().unwrap(),
            pair[1].parse::<u32>().unwrap(),
        );
        if first < second {
            acc + 1
        } else {
            acc
        }
    })
}

fn process_part_2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    lines
        .windows(3)
        .map(|window| {
            let (one, two, three) = (
                window[0].parse::<u32>().unwrap(),
                window[1].parse::<u32>().unwrap(),
                window[2].parse::<u32>().unwrap(),
            );
            one + two + three
        })
        .collect::<Vec<_>>()
        .windows(2)
        .fold(0, |acc, pair| {
            let (first, second) = (pair[0], pair[1]);
            if first < second {
                acc + 1
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 7)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 1722)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 5)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 1748)
    }
}
