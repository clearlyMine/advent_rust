use nom::bytes::complete::{tag, take_until};
use nom::*;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day21.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());
    let res2 = process_part_2(input);
    println!("Part 2: {}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> isize {
    let mut monkeys = parse_input(input);
    perform_calculation(&"root".to_string(), &mut monkeys)
}

fn perform_calculation(name: &String, monkeys: &mut HashMap<String, Monkey>) -> isize {
    let current = monkeys.remove(name).unwrap();
    let val: isize;
    match &current.value {
        MonkeyTypes::Number(v) => val = *v,
        MonkeyTypes::Calculator(calc) => {
            let first = perform_calculation(&calc.first, monkeys);
            let second = perform_calculation(&calc.second, monkeys);
            match calc.operation {
                Operation::Plus => val = first + second,
                Operation::Minus => val = first - second,
                Operation::Multiply => val = first * second,
                Operation::Divide => val = first / second,
            }
        }
    }
    monkeys.insert(
        (*name.clone()).to_string(),
        Monkey {
            value: MonkeyTypes::Number(val),
        },
    );
    val
}

fn perform_calculation_part_2(
    name: &String,
    monkeys: &mut HashMap<String, Monkey>,
) -> Option<isize> {
    let current = monkeys.get(name)?.clone();

    let val: isize;
    match &current.value {
        MonkeyTypes::Number(v) => val = *v,
        MonkeyTypes::Calculator(calc) => {
            let first = perform_calculation_part_2(&calc.first, monkeys)?;
            let second = perform_calculation_part_2(&calc.second, monkeys)?;
            match calc.operation {
                Operation::Plus => val = first + second,
                Operation::Minus => val = first - second,
                Operation::Multiply => val = first * second,
                Operation::Divide => val = first / second,
            }
        }
    }
    monkeys.insert(
        (*name.clone()).to_string(),
        Monkey {
            value: MonkeyTypes::Number(val),
        },
    );
    Some(val)
}

fn perform_expected_calculation(
    name: &String,
    monkeys: &mut HashMap<String, Monkey>,
    expected_result: isize,
) -> isize {
    let current = monkeys.get(name);
    if current.is_none() {
        return expected_result;
    }
    let current = current.unwrap().clone();

    match &current.value {
        MonkeyTypes::Number(_) => panic!(),
        MonkeyTypes::Calculator(calc) => {
            let first = perform_calculation_part_2(&calc.first, monkeys);
            let second = perform_calculation_part_2(&calc.second, monkeys);
            let expected: isize;
            if first.is_none() && second.is_some() {
                let second = second.unwrap();
                match calc.operation {
                    Operation::Plus => expected = expected_result - second,
                    Operation::Minus => expected = expected_result + second,
                    Operation::Multiply => expected = expected_result / second,
                    Operation::Divide => expected = expected_result * second,
                }
                return perform_expected_calculation(&calc.first, monkeys, expected);
            } else if first.is_some() && second.is_none() {
                let first = first.unwrap();
                match calc.operation {
                    Operation::Plus => expected = expected_result - first,
                    Operation::Minus => expected = first - expected_result,
                    Operation::Multiply => expected = expected_result / first,
                    Operation::Divide => expected = first / expected_result,
                }
                return perform_expected_calculation(&calc.second, monkeys, expected);
            }
        }
    }
    expected_result
}

fn process_part_2(input: &str) -> isize {
    let mut monkeys = parse_input_part_2(input, "humn".to_string());
    let root_monkey = monkeys.remove("root").unwrap();
    let first_root: String;
    let second_root: String;
    match &root_monkey.value {
        MonkeyTypes::Calculator(val) => {
            first_root = val.first.clone();
            second_root = val.second.clone();
        }
        MonkeyTypes::Number(_) => panic!(),
    }
    let first = perform_calculation_part_2(&first_root, &mut monkeys);
    let second: Option<isize>;
    if first.is_none() {
        second = perform_calculation_part_2(&second_root, &mut monkeys);
        if second.is_none() {
            panic!("both are None");
        }
        return perform_expected_calculation(&first_root, &mut monkeys, second.unwrap());
    }
    perform_expected_calculation(&second_root, &mut monkeys, first.unwrap())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MonkeyTypes {
    Calculator(Calc),
    Number(isize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey {
    value: MonkeyTypes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Calc {
    first: String,
    second: String,
    operation: Operation,
}

fn parse_input(input: &str) -> HashMap<String, Monkey> {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for line in input.lines() {
        let Ok((_, (name, monkey))) = parse_line(line) else {
            panic!()
        };
        monkeys.insert(name, monkey);
    }
    monkeys
}

fn parse_input_part_2(input: &str, discard_monkey: String) -> HashMap<String, Monkey> {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for line in input.lines() {
        let Ok((_, (name, monkey))) = parse_line(line) else {
            panic!()
        };
        if name == discard_monkey {
            continue;
        }
        monkeys.insert(name, monkey);
    }
    monkeys
}

fn parse_line(input: &str) -> IResult<&str, (String, Monkey)> {
    let (input, name) = take_until(": ")(input)?;
    let (input, _) = tag(": ")(input)?;
    let value: MonkeyTypes = if input.len() > 4 {
        let (input, first_name) = take_until(" ")(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, operation) = take_until(" ")(input)?;
        let (second_name, _) = tag(" ")(input)?;
        let operation = match operation {
            "+" => Operation::Plus,
            "-" => Operation::Minus,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            &_ => panic!(),
        };
        MonkeyTypes::Calculator(Calc {
            first: first_name.to_string(),
            second: second_name.to_string(),
            operation,
        })
    } else {
        let (_, val) =
            combinator::map_res(character::complete::digit1, str::parse::<isize>)(input)?;
        MonkeyTypes::Number(val)
    };
    Ok(("", (name.to_string(), Monkey { value })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part_1() {
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        assert_eq!(process_part_1(input), 152);
    }

    #[test]
    fn test_process_part_2() {
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        assert_eq!(process_part_2(input), 301);
    }
}
