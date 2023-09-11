use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day25.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> String {
    let dc: isize = input.lines().into_iter().map(to_decimal).sum();
    // dbg!(dc);
    let max_digits = input.lines().map(|line| line.len()).max().unwrap() as u128;
    let ranges = calculate_ranges(max_digits);
    to_snafu(dc, ranges)
}

fn to_snafu(decimal: isize, ranges: HashMap<u128, Range>) -> String {
    if decimal <= 2 {
        return decimal_to_snafu_char(decimal);
    }
    let remaining = decimal as i128;
    let mut str: String = "".to_string();
    let mut max_digits = 0;
    for (key, range) in ranges.clone() {
        if (range.min..=range.max).contains(&remaining) {
            max_digits = key + 1;
            break;
        }
    }
    let order: i128 = 5;
    let mut remaining = remaining as i128;
    let max_digits = max_digits as u32;
    dbg!(max_digits);
    for j in (1..max_digits).rev() {
        let range_of_next = ranges.get(&(j.saturating_sub(1) as u128));
        let last_max = range_of_next.unwrap().max as i128;
        if last_max.abs() >= remaining.abs() {
            str += "0";
            continue;
        }

        let to_check: i128 = order.pow(j);
        let cur_range = ranges.get(&(j as u128)).unwrap();
        let cur_max = cur_range.max as i128;
        let cur_min = cur_range.min as i128;
        let range_size = cur_max - cur_min + 1;
        if (-cur_max..=(-(cur_max - range_size / 2))).contains(&remaining) {
            str += "=";
            remaining += to_check * 2;
        } else if ((-(cur_min + range_size / 2) - 1)..=(-cur_min)).contains(&remaining) {
            str += "-";
            remaining += to_check;
        } else if (cur_min..=(cur_min + range_size / 2 - 1)).contains(&remaining) {
            str += "1";
            remaining -= to_check;
        } else if ((cur_max - range_size / 2)..=cur_max).contains(&remaining) {
            str += "2";
            remaining -= to_check * 2;
        }
    }
    str += &decimal_to_snafu_char(remaining as isize);

    str
}

fn decimal_to_snafu_char(decimal: isize) -> String {
    match decimal {
        -2 => "=".to_string(),
        -1 => "-".to_string(),
        _ => decimal.to_string(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Range {
    min: i128,
    max: i128,
}

impl Range {
    fn new(min: i128, max: i128) -> Range {
        Range { min, max }
    }
}

fn calculate_ranges(upto: u128) -> HashMap<u128, Range> {
    let mut map: HashMap<u128, Range> = HashMap::new();
    let order: u128 = 5;
    let mut last_max = 2;
    map.insert(0, Range::new(-2, 2));
    for i in 1..=upto {
        let p = order.pow(i as u32) as i128;
        let min = p - last_max;
        let max = p * 2 + last_max;
        last_max = max;
        map.insert(i, Range::new(min, max));
    }
    map
}

fn to_decimal(snafu: &str) -> isize {
    let len = snafu.len();
    let chars = snafu.chars();
    let mut decimal: isize = 0;
    for (i, char) in chars.into_iter().enumerate() {
        let multiple: usize = 5_usize.pow(len as u32 - i as u32 - 1);
        let d: isize = match char {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        };
        decimal += d * multiple as isize;
    }
    decimal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part_1() {
        let input = include_str!("../../inputs/day25_sample.txt");
        assert_eq!(process_part_1(input), "2=-1=0");
    }

    #[test]
    fn test_calculate_ranges() {
        let mut input = 0;
        let mut output: HashMap<u128, Range> = HashMap::new();
        output.insert(0, Range::new(-2, 2));
        let mut result = calculate_ranges(input);
        assert_eq!(output, result);

        input = 1;
        output.insert(1, Range::new(3, 12));
        result = calculate_ranges(input);
        assert_eq!(output, result);

        input = 2;
        output.insert(2, Range::new(13, 62));
        result = calculate_ranges(input);
        assert_eq!(output, result);
    }

    #[test]
    fn test_to_snafu() {
        let mut input = -2;
        let mut output = "=";
        let mut ranges = calculate_ranges(0);
        let mut result = to_snafu(input, ranges);
        assert_eq!(output, result);

        input = 3;
        output = "1=";
        ranges = calculate_ranges(1);
        result = to_snafu(input, ranges);
        assert_eq!(output, result);

        input = 8;
        output = "2=";
        ranges = calculate_ranges(1);
        result = to_snafu(input, ranges);
        assert_eq!(output, result);

        input = 12;
        output = "22";
        ranges = calculate_ranges(1);
        result = to_snafu(input, ranges);
        assert_eq!(output, result);

        input = 13;
        output = "1==";
        ranges = calculate_ranges(2);
        result = to_snafu(input, ranges);
        assert_eq!(output, result);

        input = 62;
        output = "222";
        ranges = calculate_ranges(2);
        result = to_snafu(input, ranges);
        assert_eq!(output, result);

        input = 63;
        output = "1===";
        ranges = calculate_ranges(3);
        result = to_snafu(input, ranges);
        assert_eq!(output, result);

        input = 173;
        output = "120=";
        ranges = calculate_ranges(3);
        result = to_snafu(input, ranges);
        assert_eq!(output, result);

        input = 312;
        output = "2222";
        ranges = calculate_ranges(3);
        result = to_snafu(input, ranges);
        assert_eq!(output, result);

        input = 313;
        output = "1====";
        ranges = calculate_ranges(4);
        result = to_snafu(input, ranges);
        assert_eq!(output, result);

        input = 4890;
        output = "2=-1=0";
        ranges = calculate_ranges(5);
        result = to_snafu(input, ranges);
        assert_eq!(output, result);
    }
}
