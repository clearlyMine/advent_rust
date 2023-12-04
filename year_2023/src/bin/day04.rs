use nom::bytes::complete::{tag, take_till};
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/day04.txt");

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
    input
        .lines()
        .map(|line| {
            let (_, numbers) = parse_line(line).unwrap();
            let winning_numbers = &numbers[0];
            let our_numbers = &numbers[1];
            let mut count: i32 = -1;
            for n in winning_numbers {
                if our_numbers.contains(&n) {
                    count += 1;
                }
            }
            if count > -1 {
                return 2_u32.pow(count as u32);
            }
            0
        })
        .sum()
}

fn process_part_2(input: &str) -> u32 {
    let mut number_of_cards: HashMap<usize, u32> = HashMap::new();
    let lines = input.lines();
    let num_of_lines = lines.clone().count();
    for i in 1..=num_of_lines {
        number_of_cards.insert(i, 1);
    }
    for (i, line) in lines.enumerate() {
        // dbg!(number_of_cards.clone());
        let (_, numbers) = parse_line(line).unwrap();
        let winning_numbers = &numbers[0];
        let our_numbers = &numbers[1];
        let mut count: i32 = -1;
        for n in winning_numbers {
            if our_numbers.contains(&n) {
                count += 1;
            }
        }
        // dbg!(count);
        let current_card_count = *number_of_cards.get(&(i + 1)).unwrap();
        if count > -1 {
            let count = count as usize + 2;
            for p in i + 2..=i + count {
                number_of_cards
                    .entry(p)
                    .and_modify(|num| *num += current_card_count);
            }
        }
    }
    let mut sum = 0;
    for (_, v) in number_of_cards {
        sum += v;
    }
    sum
}

fn parse_line(input: &str) -> IResult<&str, [HashSet<u32>; 2]> {
    let (input, _) = take_till(|c| c == ':')(input)?;
    let (input, _) = tag(": ")(input)?;
    let (winning, our) = input.split_once('|').unwrap();
    let winning_numbers = winning.split(' ').collect::<HashSet<&str>>();
    let winning_numbers = winning_numbers
        .into_iter()
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
    // dbg!(winning_numbers);
    let our_numbers = our.split(' ').collect::<HashSet<&str>>();
    let our_numbers = our_numbers
        .into_iter()
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
    // dbg!(our_numbers.clone());

    Ok(("", [winning_numbers, our_numbers]))
}
