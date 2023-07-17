fn main() {
    let b = include_str!("../input_day_10.txt");

    let lines = b.lines();
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
    for (i, c) in out.iter().enumerate() {
        print!("{}", c);
        if (i + 1) % 40 == 0 {
            println!();
        }
    }
}
