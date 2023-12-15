use std::time::Instant;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day15.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day15_sample.txt");

fn main() {
    let time_start = Instant::now();
    let res1 = process_part_1(SAMPLE);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(INPUT);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    input
        .trim()
        .split(",")
        .map(|s| {
            s.chars().fold(0, |acc, c| {
                let mut acc = acc + c as usize;
                acc *= 17;
                acc % 256
            })
        })
        .sum()
}

fn process_part_2(input: &str) -> usize {
    let boxes = find_box_entries(input);
    // dbg!(hm.clone());
    calculate_score(boxes)
}

#[inline]
fn calculate_score(boxes: Vec<Vec<(String, usize)>>) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(box_index, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_index, (_, focal_length))| {
                    (box_index + 1) * (lens_index + 1) * focal_length
                })
                .sum::<usize>()
        })
        .sum()
}

#[inline]
fn find_box_entries(input: &str) -> Vec<Vec<(String, usize)>> {
    //HashMap is slower because of the small number of entries
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];
    input.trim().split(",").for_each(|s| {
        if s.contains("=") {
            let (label, focal_length) = s.split_once("=").unwrap();
            let focal_length = focal_length.parse::<usize>().unwrap();
            let box_num = calculate_box_num(label);

            if let Some(element) = boxes[box_num].iter_mut().find(|(lb, _)| lb == label) {
                *element = (label.to_string(), focal_length);
                // println!(
                //     "replacing {}:{} with {}:{} in box:{}",
                //     x.0, x.1, label, focal_length, box_num
                // );
            } else {
                // println!("adding {}:{} in box:{}", label, focal_length, box_num);
                boxes[box_num].push((label.to_string(), focal_length));
            }
        } else if s.contains("-") {
            let (label, _focal_length) = s.split_once("-").unwrap();
            let box_num = calculate_box_num(label);
            if let Some(index) = boxes[box_num].iter().position(|(lb, _)| lb == label) {
                // println!("removing {}:{} from box:{}", label, _focal_length, box_num);
                boxes[box_num].remove(index);
            }
        }
    });
    boxes
}

#[inline]
fn calculate_box_num(label: &str) -> usize {
    label.chars().fold(0, |acc, c| {
        let mut acc = acc + c as usize;
        acc *= 17;
        acc % 256
    })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 1320)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 516_469)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 145)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 221_627)
    }
}
