use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input_day_3.txt") {
        let mut total_points = 0;
        for line in lines.flatten() {
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
        dbg!(total_points);
    }
}

fn get_priority_value(c: char) -> i32 {
    let ascii = c as i32;
    if c.is_uppercase() {
        return ascii - 38;
    }
    ascii - 96
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
