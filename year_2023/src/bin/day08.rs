use nom::bytes::complete::{tag, take};
use nom::IResult;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/day08.txt");

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
    // let input = include_str!("../../inputs/day08_sample_part1_1.txt");
    let (orig_directions, n) = input.split_once("\n\n").unwrap();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    n.lines().for_each(|line| {
        let (_, (main, (left, right))) = parse_node_line(line).unwrap();
        nodes.insert(main, (left, right));
    });

    let num_of_directions = orig_directions.len();
    let directions = orig_directions.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut cur = "AAA";
    while cur != "ZZZ" {
        let dir = directions[i % num_of_directions];
        let next = nodes.get(&cur).unwrap();
        cur = if dir == 'L' { &next.0 } else { &next.1 };
        i += 1;
    }
    i
}

fn process_part_2(input: &str) -> usize {
    // let input = include_str!("../../inputs/day08_sample_part2.txt");
    let (orig_directions, n) = input.split_once("\n\n").unwrap();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    n.lines().for_each(|line| {
        let (_, (main, (left, right))) = parse_node_line(line).unwrap();
        nodes.insert(main, (left, right));
    });

    let num_of_directions = orig_directions.len();
    let directions = orig_directions.chars().collect::<Vec<char>>();
    nodes
        .clone()
        .iter()
        .filter_map(|(n, _)| if n.ends_with("A") { Some(n) } else { None })
        .map(|mut cur| {
            let mut i = 0;
            while !cur.ends_with("Z") {
                let dir = directions[i % num_of_directions];
                let next = nodes.get(cur).unwrap();
                cur = if dir == 'L' { &next.0 } else { &next.1 };
                i += 1;
            }
            i
        })
        .reduce(|acc, step| lcm(acc, step))
        .unwrap()
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let rem = max % min;
        if rem == 0 {
            return min;
        }

        max = min;
        min = rem;
    }
}

fn parse_node_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, main) = take(3 as usize)(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = take(3 as usize)(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = take(3 as usize)(input)?;
    Ok((input, (main, (left, right))))
}
