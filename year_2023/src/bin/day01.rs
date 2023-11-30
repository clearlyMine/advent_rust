use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/day01_sample.txt");

    let time_start = Instant::now();
    let lines = input.lines();
    for line in lines {}
    let res1 = process_part_1();
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2();
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1() {}

fn process_part_2() {}
