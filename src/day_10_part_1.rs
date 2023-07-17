fn main() {
    let b = include_str!("../input_day_10.txt");

    let lines = b.lines();
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
    println!("{:?}", strengths);
    println!("{:?}", strengths.iter().sum::<i32>());
}
