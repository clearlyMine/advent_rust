fn main() {
    let input = include_str!("../inputs/day10.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {}", res1);
    let res2 = process_part_2(input);
    println!("Part 2: \n{}", res2);
}

fn process_part_1(input: &str) -> i32 {
    let lines = input.lines();
    let mut strengths: Vec<i32> = vec![];
    let mut clock_cycle_passed = 0;
    let mut sum = 1;
    for line in lines {
        let words: Vec<&str> = line.split(' ').collect();
        let c = 20 + strengths.len() as i32 * 40;
        if words[0] == "noop" {
            clock_cycle_passed += 1;
            if clock_cycle_passed == c {
                strengths.push(c * sum);
            }
            continue;
        }
        clock_cycle_passed += 2;
        if clock_cycle_passed == c || clock_cycle_passed == c + 1 {
            strengths.push(c * sum);
        }
        sum += words[1].parse::<i32>().unwrap();
        if strengths.len() == 6 {
            break;
        }
    }
    // println!("{:?}", strengths);
    strengths.iter().sum::<i32>()
}

fn process_part_2(input: &str) -> String {
    let lines = input.lines();
    let mut clock_cycle_passed = 1;
    let mut sprite_position = 1;
    let mut out: Vec<&str> = vec![];
    for line in lines {
        let words: Vec<&str> = line.split(' ').collect();
        let mut run_times: u8 = 1;
        if words[0] == "addx" {
            run_times = 2;
        }
        for _ in 0..run_times {
            if clock_cycle_passed % 40 - sprite_position == 0
                || clock_cycle_passed % 40 - sprite_position == 1
                || clock_cycle_passed % 40 - sprite_position == 2
            {
                out.push("#");
            } else {
                out.push(".");
            }
            clock_cycle_passed += 1;
        }
        if words[0] == "addx" {
            sprite_position += words[1].parse::<i32>().unwrap();
        }
    }
    let mut ret = "".to_string();
    for (i, c) in out.iter().enumerate() {
        ret += c;
        if (i + 1) % 40 == 0 {
            ret.push('\n');
        }
    }
    ret
}
