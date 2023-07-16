use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(mut lines) = read_lines("./input_day_7.txt") {
        lines.next();
        let mut directory_sizes: Vec<u64> = vec![];
        let total_size = recurse(&mut lines, &mut directory_sizes);
        let remaining = 70_000_000 - total_size;
        let needed = 30_000_000 - remaining;
        println!("needed space {}", needed);
        println!(
            "{}",
            directory_sizes
                .iter()
                .filter(|i| i > &&needed)
                .min()
                .unwrap()
        );
    }
}

fn recurse<B: BufRead>(lines: &mut io::Lines<B>, directory_sizes: &mut Vec<u64>) -> u64 {
    let mut size: u64 = 0;
    while let Some(Ok(line)) = lines.next() {
        if line == "$ cd .." {
            break;
        }
        if line == "$ ls" {
            continue;
        }
        let words: Vec<&str> = line.split(' ').collect();
        if words[0..2] == ["$", "cd"] {
            size += recurse(lines, directory_sizes);
            continue;
        }
        if words[0] != "dir" {
            size += words[0].parse::<u64>().unwrap();
        }
    }
    // println!("Size of {} is {}", name, size);
    directory_sizes.push(size);
    size
}
