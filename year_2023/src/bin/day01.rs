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
            let chars = line.chars().collect::<Vec<char>>();
            let line_length = line.len();
            let mut all_numbers: Vec<u32> = vec![0; line_length];
            for (i, char) in chars.iter().enumerate() {
                if char.is_numeric() {
                    all_numbers[i] = char.to_digit(10).unwrap();
                }
            }
            for s in 3..=5 {
                let mut i = 0;
                let mut j = i + s;
                while j <= line_length {
                    let word: String = chars[i..j].into_iter().collect();
                    let n = match word.as_str() {
                        "one" => 1,
                        "two" => 2,
                        "three" => 3,
                        "four" => 4,
                        "five" => 5,
                        "six" => 6,
                        "seven" => 7,
                        "eight" => 8,
                        "nine" => 9,
                        _ => 0,
                    };
                    if n > 0 {
                        all_numbers[i] = n;
                    }
                    i += 1;
                    j += 1;
                }
            }
            all_numbers = all_numbers.into_iter().filter(|x| x > &0).collect();
            let x = all_numbers[0] * 10 + all_numbers[all_numbers.len() - 1];
            x
        })
        .sum()
}
