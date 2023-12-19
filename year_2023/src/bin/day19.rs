use nom::bytes::complete::{is_not, tag, take_till};
use nom::character::complete::{digit1, one_of};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::{collections::HashMap, time::Instant};

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day19.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day19_sample.txt");

fn main() {
    let time_start = Instant::now();
    let res1 = process_part_1(INPUT);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(INPUT);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let (operations, parts) = input.split_once("\n\n").unwrap();
    let (_, operations) = parse_operations(operations).unwrap();
    // dbg!(operations.clone());
    let (_, parts) = parse_parts(parts).unwrap();
    // dbg!(parts.clone());
    let mut accepted_parts_sum = 0;
    for part in parts {
        let mut curr_operation = "in".to_string();
        while curr_operation != "A" && curr_operation != "R" {
            if let Some(o) = operations.get(&curr_operation) {
                for op in o {
                    match op {
                        OperationType::Destination(val) => {
                            curr_operation = val.to_string();
                            break;
                        }
                        OperationType::Operation(op) => {
                            let cur_count = part.get(&op.part).unwrap();
                            let op_success = match op.operator {
                                Operator::LessThan => cur_count < &op.count,
                                Operator::GreaterThan => cur_count > &op.count,
                            };
                            if op_success {
                                curr_operation = op.destination.clone();
                                break;
                            }
                        }
                    }
                }
            }
        }
        if curr_operation == "A" {
            accepted_parts_sum += part.into_iter().map(|(_, c)| c).sum::<usize>();
        }
    }
    accepted_parts_sum
}

fn process_part_2(input: &str) -> usize {
    let (operations, _) = input.split_once("\n\n").unwrap();
    let (_, operations) = parse_operations(operations).unwrap();
    // dbg!(operations.clone());
    // dbg!(parts.clone());
    let mut accepted_part_combination = 0;
    let mut accepted_paths = vec![];
    let mut queue = vec![(OperationType::Destination("in".to_string()), vec![])];
    while let Some((next, constraints)) = queue.pop() {
        // dbg!(next.clone(), constraints.clone());
        if next == OperationType::Destination("A".to_string()) {
            // println!("path accepted");
            accepted_paths.push(constraints);
            continue;
        }
        if next == OperationType::Destination("R".to_string()) {
            // println!("path rejected");
            continue;
        }
        match next {
            OperationType::Destination(n) => {
                let mut inverted_constraints = constraints.clone();
                for o in operations.get(&n).unwrap() {
                    match o {
                        OperationType::Destination(_) => {
                            queue.push((o.clone(), inverted_constraints.clone()));
                        }
                        OperationType::Operation(oper) => {
                            queue.push((o.clone(), inverted_constraints.clone()));
                            let mut oper = oper.clone();
                            oper.operator = if oper.operator == Operator::LessThan {
                                oper.count -= 1;
                                Operator::GreaterThan
                            } else {
                                oper.count += 1;
                                Operator::LessThan
                            };
                            inverted_constraints.push(oper.clone());
                        }
                    }
                }
            }
            OperationType::Operation(o) => {
                let mut constraints = constraints.clone();
                //add the new operation to constraint
                constraints.push(o.clone());
                let dest = OperationType::Destination(o.destination);
                queue.push((dest, constraints.clone()));
            }
        }
        // dbg!(queue.clone());
    }
    // dbg!(accepted_paths.clone());

    for path in accepted_paths {
        // dbg!(path.clone());
        let mut min_x = 1;
        let mut max_x = 4000;
        let mut min_m = 1;
        let mut max_m = 4000;
        let mut min_a = 1;
        let mut max_a = 4000;
        let mut min_s = 1;
        let mut max_s = 4000;

        for ele in path {
            let count = ele.count;
            match ele.part {
                'x' => {
                    if ele.operator == Operator::LessThan {
                        max_x = max_x.min(count - 1);
                    } else {
                        min_x = min_x.max(count + 1);
                    }
                }
                'm' => {
                    if ele.operator == Operator::LessThan {
                        max_m = max_m.min(count - 1);
                    } else {
                        min_m = min_m.max(count + 1);
                    }
                }
                'a' => {
                    if ele.operator == Operator::LessThan {
                        max_a = max_a.min(count - 1);
                    } else {
                        min_a = min_a.max(count + 1);
                    }
                }
                's' => {
                    if ele.operator == Operator::LessThan {
                        max_s = max_s.min(count - 1);
                    } else {
                        min_s = min_s.max(count + 1);
                    }
                }
                _ => panic!(),
            }
        }
        if min_x > max_x || min_m > max_m || min_a > max_a || min_s > max_s {
            continue;
        }
        let t =
            (max_x - min_x + 1) * (max_m - min_m + 1) * (max_a - min_a + 1) * (max_s - min_s + 1);
        // dbg!(t);
        accepted_part_combination += t;
    }
    accepted_part_combination
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum OperationType {
    Operation(Operation),
    Destination(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Operation {
    part: char,
    operator: Operator,
    count: usize,
    destination: String,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
enum Operator {
    LessThan,
    GreaterThan,
}

fn parse_operations(input: &str) -> IResult<&str, HashMap<String, Vec<OperationType>>> {
    let mut out = HashMap::new();
    let (input, operations) = separated_list1(tag("\n"), parse_operation)(input)?;
    for (name, o) in operations {
        out.insert(name.to_string(), o);
    }
    Ok((input, out))
}

fn parse_operation(input: &str) -> IResult<&str, (String, Vec<OperationType>)> {
    let (input, name) = take_till(|c| c == '{')(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, operations) = take_till(|c| c == '}')(input)?;
    let (_, operations) = separated_list1(tag(","), parse_operation_type)(operations)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, (name.to_string(), operations)))
}

fn parse_operation_type(input: &str) -> IResult<&str, OperationType> {
    if !input.contains(":") {
        return Ok(("", OperationType::Destination(input.to_string())));
    }
    let (input, part) = one_of("xmas")(input)?;
    let (input, operator) = one_of("<>")(input)?;
    let operator = if operator == '>' {
        Operator::GreaterThan
    } else {
        Operator::LessThan
    };
    let (input, count) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, destination) = is_not(",}")(input)?;
    let out = Operation {
        part,
        count: count.parse().unwrap(),
        operator,
        destination: destination.to_string(),
    };
    Ok((input, OperationType::Operation(out)))
}

fn parse_parts(input: &str) -> IResult<&str, Vec<HashMap<char, usize>>> {
    separated_list1(tag("\n"), parse_part)(input)
}

fn parse_part(input: &str) -> IResult<&str, HashMap<char, usize>> {
    let (input, _) = tag("{")(input)?;
    let (input, parts) =
        separated_list1(tag(","), separated_pair(one_of("xmas"), tag("="), digit1))(input)?;
    let (input, _) = tag("}")(input)?;

    let mut out = HashMap::new();
    for (name, count) in parts {
        out.insert(name, count.parse().unwrap());
    }
    Ok((input, out))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 19_114)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 395_382)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 167_409_079_868_000)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 103_557_657_654_583)
    }
}
