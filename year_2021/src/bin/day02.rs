use std::time::Instant;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day02.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day02_sample.txt");

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
    let c = input.lines().fold((0, 0), |acc, line| {
        let (direction, distance) = line.split_once(" ").unwrap();
        let distance = distance.parse::<i32>().unwrap();
        let (x, y) = match direction {
            "forward" => (distance, 0),
            "up" => (0, -distance),
            "down" => (0, distance),
            _ => panic!("Unknown direction"),
        };
        (acc.0 + x, acc.1 + y)
    });
    (c.0 * c.1).try_into().unwrap()
}

fn process_part_2(input: &str) -> u32 {
    let c = input.lines().fold((0, 0, 0), |acc, line| {
        let (direction, distance) = line.split_once(" ").unwrap();
        let distance = distance.parse::<i32>().unwrap();
        let x = match direction {
            "forward" => (distance, acc.2 * distance, 0),
            "up" => (0, 0, -distance),
            "down" => (0, 0, distance),
            _ => panic!("Unknown direction"),
        };
        (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2)
    });
    (c.0 * c.1).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 150)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 1840243)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 900)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 1727785422)
    }
}
