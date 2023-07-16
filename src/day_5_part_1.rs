use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input_day_5.txt") {
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
        let mut stacks_to_insert: Vec<Vec<char>> = vec![Vec::new(); 9];
        let mut line_count = 0;
        for line in lines.flatten() {
            line_count += 1;
            if line_count == 9 || line_count == 10 {
                continue;
            }
            //read the first 9 lines and make stacks for each number
            if line_count <= 8 {
                let chars: Vec<char> = line.chars().collect::<Vec<_>>();
                dbg!(chars.clone());
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
        for i in 0..9 {
            println!("{}->{}", i + 1, stacks[i].pop().unwrap());
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
