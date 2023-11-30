use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day3.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> i32 {
    let lines = input.lines();
    let mut total_points = 0;
    for line in lines {
        if line.is_empty() {
            panic!("line is empty");
        }
        let (first, second) = line.split_at(line.len() / 2);
        let first_chars = first.chars();
        let second_chars = second.chars();
        'outer: for c in first_chars {
            for d in second_chars.clone() {
                if c == d {
                    total_points += get_priority_value(c);
                    break 'outer;
                }
            }
        }
    }
    total_points
}

fn process_part_2(input: &str) -> i32 {
    let mut lines = input.lines();

    let mut total_points = 0;

    while let (Some(line_first), Some(line_second), Some(line_third)) =
        (lines.next(), lines.next(), lines.next())
    {
        let first = line_first;
        let second = line_second;
        let third = line_third;

        let first_chars = first.chars();
        let second_chars = second.chars();
        let third_chars = third.chars();
        'outer: for c in first_chars {
            for d in second_chars.clone() {
                if c == d {
                    for e in third_chars.clone() {
                        if c == e {
                            total_points += get_priority_value(c);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    total_points
}

fn get_priority_value(c: char) -> i32 {
    let ascii = c as i32;
    if c.is_uppercase() {
        return ascii - 38;
    }
    ascii - 96
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_priority_value_lowercase() {
        let input = 'a';
        let expected = 1;
        let result = get_priority_value(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_priority_value_uppercase() {
        let input = 'A';
        let expected = 27;
        let result = get_priority_value(input);
        assert_eq!(result, expected);
    }
}
