use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/day01.txt");

    let time_start = Instant::now();
    let res1 = process_part_1(input);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let mut chars = line.chars();
        let first = chars
            .find(|x| x.is_numeric())
            .expect("At least 1 number was expected");
        let last = chars.rfind(|x| x.is_numeric()).unwrap_or(first);
        let first = first.to_digit(10).unwrap();
        let last = last.to_digit(10).unwrap();

        acc + first * 10 + last
    })
}

fn process_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .fold(0, |acc, line| {
            let mut chars = line.chars();
            let first = chars
                .find(|x| x.is_numeric())
                .expect("At least 1 number was expected");
            let last = chars.rfind(|x| x.is_numeric()).unwrap_or(first);
            let first = first.to_digit(10).unwrap();
            let last = last.to_digit(10).unwrap();

            acc + first * 10 + last
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part_2() {
        let input = include_str!("../../inputs/day01_sample_02.txt");
        assert_eq!(process_part_2(input), 281);
    }
}
