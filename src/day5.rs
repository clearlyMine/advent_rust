fn main() {
    let input = include_str!("../inputs/day5.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {:?}", res1);
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
}

fn process_part_1(input: &str) -> String {
    let lines = input.lines();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    let mut stacks_to_insert: Vec<Vec<char>> = vec![Vec::new(); 9];
    let mut line_count = 0;
    for line in lines {
        line_count += 1;
        if line_count == 9 || line_count == 10 {
            continue;
        }
        //read the first 9 lines and make stacks for each number
        if line_count <= 8 {
            let chars: Vec<char> = line.chars().collect::<Vec<_>>();
            // dbg!(chars.clone());
            if chars.get(1).unwrap() != &' ' {
                stacks_to_insert[0].push(*chars.get(1).unwrap());
            }
            if chars.get(5).unwrap() != &' ' {
                stacks_to_insert[1].push(*chars.get(5).unwrap());
            }
            if chars.get(9).unwrap() != &' ' {
                stacks_to_insert[2].push(*chars.get(9).unwrap());
            }
            if chars.get(13).unwrap() != &' ' {
                stacks_to_insert[3].push(*chars.get(13).unwrap());
            }
            if chars.get(17).unwrap() != &' ' {
                stacks_to_insert[4].push(*chars.get(17).unwrap());
            }
            if chars.get(21).unwrap() != &' ' {
                stacks_to_insert[5].push(*chars.get(21).unwrap());
            }
            if chars.get(25).unwrap() != &' ' {
                stacks_to_insert[6].push(*chars.get(25).unwrap());
            }
            if chars.get(29).unwrap() != &' ' {
                stacks_to_insert[7].push(*chars.get(29).unwrap());
            }
            if chars.get(33).unwrap() != &' ' {
                stacks_to_insert[8].push(*chars.get(33).unwrap());
            }
            continue;
        }
        for i in 0..9 {
            while !stacks_to_insert[i].is_empty() {
                stacks[i].push(stacks_to_insert[i].pop().unwrap());
            }
        }

        //then pop and push stuff into the stacks
        let moves: Vec<&str> = line.split(' ').collect();
        let (crates, from, to) = (
            moves[1].parse::<i32>().unwrap(),
            moves[3].parse::<usize>().unwrap(),
            moves[5].parse::<usize>().unwrap(),
        );
        for _i in 0..crates {
            let c: char = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
        //finally pop each stack to get the answer
    }
    let mut out: String = "".to_string();
    for i in 0..9 {
        out.push(stacks[i].pop().unwrap());
    }
    return out;
}

fn process_part_2(input: &str) -> String {
    let lines = input.lines();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    let mut stacks_to_insert: Vec<Vec<char>> = vec![Vec::new(); 9];
    let mut line_count = 0;
    for line in lines {
        line_count += 1;
        if line_count == 9 || line_count == 10 {
            continue;
        }
        //read the first 9 lines and make stacks for each number
        if line_count <= 8 {
            let chars: Vec<char> = line.chars().collect::<Vec<_>>();
            // dbg!(chars.clone());
            if chars.get(1).unwrap() != &' ' {
                stacks_to_insert[0].push(*chars.get(1).unwrap());
            }
            if chars.get(5).unwrap() != &' ' {
                stacks_to_insert[1].push(*chars.get(5).unwrap());
            }
            if chars.get(9).unwrap() != &' ' {
                stacks_to_insert[2].push(*chars.get(9).unwrap());
            }
            if chars.get(13).unwrap() != &' ' {
                stacks_to_insert[3].push(*chars.get(13).unwrap());
            }
            if chars.get(17).unwrap() != &' ' {
                stacks_to_insert[4].push(*chars.get(17).unwrap());
            }
            if chars.get(21).unwrap() != &' ' {
                stacks_to_insert[5].push(*chars.get(21).unwrap());
            }
            if chars.get(25).unwrap() != &' ' {
                stacks_to_insert[6].push(*chars.get(25).unwrap());
            }
            if chars.get(29).unwrap() != &' ' {
                stacks_to_insert[7].push(*chars.get(29).unwrap());
            }
            if chars.get(33).unwrap() != &' ' {
                stacks_to_insert[8].push(*chars.get(33).unwrap());
            }
            continue;
        }
        for i in 0..9 {
            while !stacks_to_insert[i].is_empty() {
                stacks[i].push(stacks_to_insert[i].pop().unwrap());
            }
        }

        //then pop and push stuff into the stacks
        let moves: Vec<&str> = line.split(' ').collect();
        let (crates, from, to) = (
            moves[1].parse::<i32>().unwrap(),
            moves[3].parse::<usize>().unwrap(),
            moves[5].parse::<usize>().unwrap(),
        );
        for _ in 0..crates {
            let c: char = stacks[from - 1].pop().unwrap();
            stacks_to_insert[to - 1].push(c);
        }
        for i in 0..9 {
            while !stacks_to_insert[i].is_empty() {
                stacks[i].push(stacks_to_insert[i].pop().unwrap());
            }
        }
        //finally pop each stack to get the answer
    }
    let mut out = "".to_string();
    for i in 0..9 {
        out.push(stacks[i].pop().unwrap());
    }
    out
}
