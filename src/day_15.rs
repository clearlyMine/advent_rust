use nom::character::complete::newline;
use nom::combinator::{map_res, opt, recognize};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::{character, *};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::RangeInclusive;
fn main() {
    let input = include_str!("../input_day_15.txt");
    let res1 = process_part_1(input, 2_000_000);
    println!("Part 1:{}", res1);
    let res2 = process_part_2(input);
    println!("Part 2:{}", res2);
}

fn process_part_1(input: &str, row: i32) -> u32 {
    let (_, lists) = parse_whole_input(input).unwrap();
    // dbg!(lists.clone());
    // let (sensor_positions, beacon_positions) = (lists[0], lists[1]);
    let mut no_beacons: HashSet<i32> = HashSet::new();
    for list in lists.clone() {
        // println!();
        let sensor_position = list.0;
        let beacon_position = list.1;
        //calculate manhattan distance
        let sx = sensor_position.0;
        let sy = sensor_position.1;
        let mx = sx - beacon_position.0;
        let my = sy - beacon_position.1;
        let manhattan_distance = mx.abs() + my.abs();
        // dbg!(manhattan_distance);
        // dbg!(sy - manhattan_distance..=sy + manhattan_distance);
        if !(sy - manhattan_distance..=sy + manhattan_distance).contains(&row) {
            continue;
        }
        // dbg!("row in range");
        // dbg!("------------");
        let dy = (sy - row).abs();
        // dbg!(-(manhattan_distance - dy).abs()..=(manhattan_distance - dy).abs());
        for dx in -(manhattan_distance - dy).abs()..=(manhattan_distance - dy).abs() {
            // println!("inserting {}", sx + dx);
            no_beacons.insert(sx + dx);
        }
    }
    // dbg!(no_beacons.clone().into_iter().sorted());
    for list in lists {
        let sensor_position = list.0;
        if sensor_position.1 == row {
            no_beacons.remove(&sensor_position.0);
        }
        let beacon_position = list.1;
        if beacon_position.1 == row {
            no_beacons.remove(&beacon_position.0);
        }
    }
    no_beacons.len() as u32
}

fn process_part_2(input: &str) -> u64 {
    let (_, lists) = parse_whole_input(input).unwrap();
    // dbg!(lists.clone());
    let max_coord = 4_000_000;
    let max_y = min(
        *lists.iter().map(|((_, sy), (_, _))| sy).max().unwrap(),
        max_coord,
    );
    let min_y = max(*lists.iter().map(|((_, sy), (_, _))| sy).min().unwrap(), 0);
    // dbg!(min_y);
    // dbg!(max_y);
    let mut distress_coord: (u64, u64) = (0, 0);
    for y in min_y..=max_y {
        // if y % 1_000_000 == 0 {
        //     dbg!(y);
        // }
        let mut ranges_x: Vec<RangeInclusive<i32>> = vec![];
        for ((sx, sy), (bx, by)) in lists.clone() {
            let distance = (sx - bx).abs() + (sy - by).abs();
            let dy = (sy - y).abs();
            if dy > distance {
                continue;
            }
            let dx = distance - dy;

            ranges_x.push(sx - dx..=sx + dx);
        }
        // dbg!(ranges_x.clone());
        let mut final_ranges = merge_ranges(ranges_x.clone());
        // dbg!(final_ranges.clone());
        if final_ranges.len() > 1 {
            for i in (0..final_ranges.len()).rev() {
                let r = final_ranges.get(i).expect("impossible");
                if (*r.start() < 0 && *r.end() < 0) || (*r.start() > max_y && *r.end() > max_y) {
                    println!("removed from final ranges {:?}", final_ranges.remove(i));
                }
            }
        }
        if final_ranges.len() > 1 {
            // dbg!(ranges_x.clone());
            // dbg!(final_ranges.clone());
            distress_coord = (final_ranges[0].end().to_owned() as u64 + 1, y as u64);
            break;
        }
    }
    // dbg!(distress_coord);
    distress_coord.0 * max_coord as u64 + distress_coord.1
}

fn merge_ranges(all_ranges: Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    if all_ranges.len() < 2 {
        return all_ranges;
    }

    let mut ranges = all_ranges.clone();
    ranges.sort_by_key(|x| *x.start());
    let l = ranges.len();
    let mut result: Vec<RangeInclusive<i32>> = vec![];

    let mut acc: RangeInclusive<i32> = ranges.get(0).unwrap().clone();
    for i in 1..l {
        let curr = ranges.get(i).unwrap();

        if *acc.end() >= (curr.start() - 1) {
            acc = acc.start().to_owned()..=i32::max(acc.end().to_owned(), curr.end().to_owned());
        } else {
            result.push(acc);
            acc = curr.clone();
        }
    }

    result.push(acc);

    result
}

fn parse_whole_input(input: &str) -> IResult<&str, Vec<((i32, i32), (i32, i32))>> {
    let (input, lists) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lists))
}

fn parse_line(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    let (input, _) = bytes::complete::tag("Sensor at x=")(input)?;
    let (input, sensor_x) = map_res(
        recognize(preceded(
            opt(bytes::complete::tag("-")),
            character::complete::digit1,
        )),
        |s: &str| s.parse::<i32>(),
    )(input)?;

    let (input, _) = bytes::complete::tag(", y=")(input)?;
    let (input, sensor_y) = map_res(
        recognize(preceded(
            opt(bytes::complete::tag("-")),
            character::complete::digit1,
        )),
        |s: &str| s.parse::<i32>(),
    )(input)?;

    let (input, _) = bytes::complete::tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = map_res(
        recognize(preceded(
            opt(bytes::complete::tag("-")),
            character::complete::digit1,
        )),
        |s: &str| s.parse::<i32>(),
    )(input)?;

    let (input, _) = bytes::complete::tag(", y=")(input)?;
    let (input, beacon_y) = map_res(
        recognize(preceded(
            opt(bytes::complete::tag("-")),
            character::complete::digit1,
        )),
        |s: &str| s.parse::<i32>(),
    )(input)?;

    Ok((input, ((sensor_x, sensor_y), (beacon_x, beacon_y))))
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_process_part_1() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        assert_eq!(process_part_1(input, 10), 26);
    }
    #[test]
    fn test_process_part_2() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        assert_eq!(process_part_2(input), 56000011);
    }

    #[test]
    fn test_find_ranges() {
        let mut input_1: Vec<RangeInclusive<i32>> = vec![0..=2, 0..=2];
        assert_eq!(merge_ranges(input_1), vec![0..=2]);

        input_1 = vec![0..=2, 0..=3];
        assert_eq!(merge_ranges(input_1), vec![0..=3]);

        input_1 = vec![0..=2, 2..=3];
        assert_eq!(merge_ranges(input_1), vec![0..=3]);

        input_1 = vec![0..=2, 3..=3];
        assert_eq!(merge_ranges(input_1), vec![0..=3]);

        input_1 = vec![0..=2, 4..=4];
        assert_eq!(merge_ranges(input_1), vec![0..=2, 4..=4]);

        input_1 = vec![0..=4, 5..=8, 10..=18, 0..=20];
        assert_eq!(merge_ranges(input_1), vec![0..=20]);

        input_1 = vec![12..=14, 6..=10, 0..=12, 14..=26];
        assert_eq!(merge_ranges(input_1), vec![0..=26]);
    }
}
