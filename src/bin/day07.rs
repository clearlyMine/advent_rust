use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day7.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> i64 {
    let mut lines = input.lines();
    lines.next();
    let mut sum: i64 = 0;
    recurse(&mut lines, &mut sum);
    sum
}

fn process_part_2(input: &str) -> u64 {
    let mut lines = input.lines();
    lines.next();
    let mut directory_sizes: Vec<u64> = vec![];
    let total_size = recurse_2(&mut lines, &mut directory_sizes);
    let remaining = 70_000_000 - total_size;
    let needed = 30_000_000 - remaining;
    // println!("needed space {}", needed);
    *directory_sizes
        .iter()
        .filter(|i| i > &&needed)
        .min()
        .unwrap()
}

fn recurse(lines: &mut std::str::Lines, sum: &mut i64) -> i64 {
    let mut size: i64 = 0;
    while let Some(line) = lines.next() {
        if line == "$ cd .." {
            break;
        }
        if line == "$ ls" {
            continue;
        }
        let words: Vec<&str> = line.split(' ').collect();
        if words[0..2] == ["$", "cd"] {
            size += recurse(lines, sum);
            continue;
        }
        if words[0] != "dir" {
            size += words[0].parse::<i64>().unwrap();
        }
    }
    if size <= 100_000 {
        *sum += size;
    }
    size
}

fn recurse_2(lines: &mut std::str::Lines, directory_sizes: &mut Vec<u64>) -> u64 {
    let mut size: u64 = 0;
    while let Some(line) = lines.next() {
        if line == "$ cd .." {
            break;
        }
        if line == "$ ls" {
            continue;
        }
        let words: Vec<&str> = line.split(' ').collect();
        if words[0..2] == ["$", "cd"] {
            size += recurse_2(lines, directory_sizes);
            continue;
        }
        if words[0] != "dir" {
            size += words[0].parse::<u64>().unwrap();
        }
    }
    // println!("Size of {} is {}", name, size);
    directory_sizes.push(size);
    size
}
