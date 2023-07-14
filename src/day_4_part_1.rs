use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Range {
    start: i32,
    end: i32,
}

fn main() {
    if let Ok(lines) = read_lines("./input_day_4.txt") {
        let mut count = 0;
        for line in lines.flatten() {
            if line.is_empty() {
                panic!("line is empty");
            }
            let ranges: Vec<&str> = line.split(',').collect();
            let first_range = ranges[0];
            let second_range = ranges[1];
            let x: Vec<&str> = first_range.split('-').collect();
            let y: Vec<&str> = second_range.split('-').collect();
            let mut first_full_range: Range = Range {
                start: i32::MIN,
                end: i32::MIN,
            };
            let mut second_full_range: Range = Range {
                start: i32::MIN,
                end: i32::MIN,
            };
            first_full_range.start = x[0].parse::<i32>().unwrap();
            first_full_range.end = x[1].parse::<i32>().unwrap();
            second_full_range.start = y[0].parse::<i32>().unwrap();
            second_full_range.end = y[1].parse::<i32>().unwrap();
            if (first_full_range.start >= second_full_range.start
                && first_full_range.end <= second_full_range.end)
                || (second_full_range.start >= first_full_range.start
                    && second_full_range.end <= first_full_range.end)
            {
                count += 1;
            }
        }
        dbg!(count);
    }
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
