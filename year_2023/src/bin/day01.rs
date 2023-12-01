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
    let mut numbers: Vec<u32> = vec![];
    //this is faster than using functional programming here
    for line in input.lines() {
        let chars = line.chars();
        let mut all_numbers: Vec<u32> = vec![];
        for char in chars {
            if char.is_numeric() {
                all_numbers.push(char.to_digit(10).unwrap_or(0));
            }
        }
        let x = all_numbers[0] * 10 + all_numbers[all_numbers.len() - 1];
        numbers.push(x);
    }
    numbers.iter().sum()
}

fn process_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let n = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2",
                "3", "4", "5", "6", "7", "8", "9",
            ];
            let mut first = 0;
            let mut last = 0;
            let mut first_index = usize::MAX;
            let mut last_index = usize::MIN;

            for x in n {
                let a = match x {
                    "one" | "1" => 1,
                    "two" | "2" => 2,
                    "three" | "3" => 3,
                    "four" | "4" => 4,
                    "five" | "5" => 5,
                    "six" | "6" => 6,
                    "seven" | "7" => 7,
                    "eight" | "8" => 8,
                    "nine" | "9" => 9,
                    _ => 0,
                };
                if let Some(p) = line.find(x) {
                    if first_index > p {
                        first_index = p;
                        first = a;
                    }
                }
                if let Some(p) = line.rfind(x) {
                    if last_index <= p {
                        last_index = p;
                        last = a;
                    }
                }
            }
            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part_2() {
        let input = include_str!("../../inputs/day01_sample_02.txt");
        assert_eq!(process_part_2(input), 6);
    }
}
