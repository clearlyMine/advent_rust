use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/day06.txt");

    let time_start = Instant::now();
    let res1 = process_part_1(input);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .trim()
        .split_whitespace()
        .map(|t| t.parse::<usize>().unwrap());

    let distances: Vec<usize> = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .trim()
        .split_whitespace()
        .map(|t| t.parse::<usize>().unwrap())
        .collect();

    times
        .into_iter()
        .enumerate()
        .map(|(i, time)| {
            let distance = distances[i];
            let mut min = usize::MAX;
            let mut max = usize::MIN;
            for wait in 0..time {
                if calculate_distance(wait, time - wait) > distance {
                    min = wait;
                    break;
                }
            }
            for wait in (0..time).rev() {
                if calculate_distance(wait, time - wait) > distance {
                    max = wait;
                    break;
                }
            }
            max - min + 1
        })
        .product()
}

#[inline]
fn calculate_distance(wait: usize, run_for: usize) -> usize {
    run_for * wait
}

fn process_part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let time: usize = lines
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .replace(' ', "")
        .parse()
        .unwrap();

    let distance: usize = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .replace(' ', "")
        .parse()
        .unwrap();

    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for wait in 0..time {
        if calculate_distance(wait, time - wait) > distance {
            min = wait;
            break;
        }
    }
    for wait in (0..time).rev() {
        if calculate_distance(wait, time - wait) > distance {
            max = wait;
            break;
        }
    }
    max - min + 1
}
