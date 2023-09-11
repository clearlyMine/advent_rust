use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day6.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let s = input.lines().next().unwrap();
    let mut letters = s.chars();
    let mut out: String = "".to_string();
    out.push(letters.next().unwrap());
    out.push(letters.next().unwrap());
    out.push(letters.next().unwrap());

    while let Some(c) = letters.next() {
        out.push(c);
        let last_four_characters: Vec<char> = out.chars().rev().take(4).collect();
        if check_if_all_unique(last_four_characters) {
            break;
        }
    }
    out.len()
}

fn process_part_2(input: &str) -> usize {
    let s = input.lines().next().unwrap();
    let mut letters = s.chars();
    let mut out: String = "".to_string();
    for _ in 0..13 {
        out.push(letters.next().unwrap());
    }

    while let Some(c) = letters.next() {
        out.push(c);
        let last_fourteen_characters: Vec<char> = out.chars().rev().take(14).collect();
        if check_if_all_unique(last_fourteen_characters) {
            break;
        }
    }
    out.len()
}

fn check_if_all_unique(last_four_characters: Vec<char>) -> bool {
    for i in 0..last_four_characters.len() {
        for j in i + 1..last_four_characters.len() {
            if last_four_characters[i] == last_four_characters[j] {
                return false;
            }
        }
    }
    true
}
