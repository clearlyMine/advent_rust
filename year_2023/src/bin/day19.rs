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
    let (rules, parts) = input.split_once("\n\n").unwrap();

    let (_, rules) = parse_rules(rules).unwrap();
    let (_, parts) = parse_parts(parts).unwrap();

    let mut accepted_parts_sum = 0;
    for part in parts {
        let mut curr_rule = "in".to_string();
        while curr_rule != "A" && curr_rule != "R" {
            if let Some(o) = rules.get(&curr_rule) {
                for op in o {
                    match op {
                        RuleType::Destination(val) => {
                            curr_rule = val.to_string();
                            break;
                        }
                        RuleType::Rule(op) => {
                            let cur_count = part.get(&op.part).unwrap();
                            let op_success = match op.operator {
                                Operator::LessThan => cur_count < &op.count,
                                Operator::GreaterThan => cur_count > &op.count,
                            };
                            if op_success {
                                curr_rule = op.destination.clone();
                                break;
                            }
                        }
                    }
                }
            }
        }
        if curr_rule == "A" {
            accepted_parts_sum += part.into_iter().map(|(_, c)| c).sum::<usize>();
        }
    }
    accepted_parts_sum
}

fn process_part_2(input: &str) -> usize {
    let (rules, _) = input.split_once("\n\n").unwrap();
    let (_, rules) = parse_rules(rules).unwrap();

    let mut accepted_paths = vec![];

    let mut queue = vec![(RuleType::Destination("in".to_string()), vec![])];
    while let Some((next, constraints)) = queue.pop() {
        if next == RuleType::Destination("A".to_string()) {
            accepted_paths.push(constraints);
            continue;
        }
        if next == RuleType::Destination("R".to_string()) {
            continue;
        }
        match next {
            RuleType::Destination(next) => {
                let mut inverted_constraints = constraints.clone();
                for rule in rules.get(&next).unwrap() {
                    match rule {
                        RuleType::Destination(_) => {
                            queue.push((rule.clone(), inverted_constraints.clone()));
                        }
                        RuleType::Rule(r) => {
                            queue.push((rule.clone(), inverted_constraints.clone()));
                            let mut rule = r.clone();
                            rule.operator = if rule.operator == Operator::LessThan {
                                rule.count -= 1;
                                Operator::GreaterThan
                            } else {
                                rule.count += 1;
                                Operator::LessThan
                            };
                            inverted_constraints.push(rule.clone());
                        }
                    }
                }
            }
            RuleType::Rule(rule) => {
                let mut constraints = constraints.clone();
                constraints.push(rule.clone());
                let dest = RuleType::Destination(rule.destination);
                queue.push((dest, constraints.clone()));
            }
        }
    }

    let mut accepted_part_combination = 0;
    for path in accepted_paths {
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
        accepted_part_combination +=
            (max_x - min_x + 1) * (max_m - min_m + 1) * (max_a - min_a + 1) * (max_s - min_s + 1);
    }
    accepted_part_combination
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum RuleType {
    Rule(Rule),
    Destination(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Rule {
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

fn parse_rules(input: &str) -> IResult<&str, HashMap<String, Vec<RuleType>>> {
    let mut out = HashMap::new();
    let (input, rules) = separated_list1(tag("\n"), parse_rule)(input)?;
    for (name, o) in rules {
        out.insert(name.to_string(), o);
    }
    Ok((input, out))
}

fn parse_rule(input: &str) -> IResult<&str, (String, Vec<RuleType>)> {
    let (input, name) = take_till(|c| c == '{')(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, rules) = take_till(|c| c == '}')(input)?;
    let (_, rules) = separated_list1(tag(","), parse_rule_type)(rules)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, (name.to_string(), rules)))
}

fn parse_rule_type(input: &str) -> IResult<&str, RuleType> {
    if !input.contains(":") {
        return Ok(("", RuleType::Destination(input.to_string())));
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
    let out = Rule {
        part,
        count: count.parse().unwrap(),
        operator,
        destination: destination.to_string(),
    };
    Ok((input, RuleType::Rule(out)))
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
