use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input_day_3.txt").expect("Unable to open input file");
    let mut lines = io::BufReader::new(file).lines();

    let mut total_points = 0;

    while let (Some(line_first), Some(line_second), Some(line_third)) =
        (lines.next(), lines.next(), lines.next())
    {
        let first = line_first.unwrap();
        let second = line_second.unwrap();
        let third = line_third.unwrap();

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
    dbg!(total_points);
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
