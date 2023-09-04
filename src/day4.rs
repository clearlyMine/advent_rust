struct Range {
    start: i32,
    end: i32,
}

fn main() {
    let input = include_str!("../inputs/day4.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {}", res1);
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
}

fn process_part_1(input: &str) -> usize {
    let lines = input.lines();
    let mut count = 0;
    for line in lines {
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
    return count;
}

fn process_part_2(input: &str) -> usize {
    let lines = input.lines();
    let mut count = 0;
    for line in lines {
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
        if first_full_range.start <= second_full_range.end
            && second_full_range.start <= first_full_range.end
        {
            count += 1;
        }
    }
    count
}
