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
        let mut sum: i64 = 0;
        recurse(&mut lines, &mut sum, "/");
        println!("{}", sum);
    }
}

fn recurse<B: BufRead>(lines: &mut io::Lines<B>, sum: &mut i64, name: &str) -> i64 {
    let mut size: i64 = 0;
    while let Some(Ok(line)) = lines.next() {
        if line == "$ cd .." {
            break;
        }
        if line == "$ ls" {
            continue;
        }
        let words: Vec<&str> = line.split(' ').collect();
        if words[0..2] == ["$", "cd"] {
            size += recurse(lines, sum, words[2]);
            continue;
        }
        if words[0] != "dir" {
            size += words[0].parse::<i64>().unwrap();
        }
    }
    if size <= 100_000 {
        *sum += size;
    }
    size
}
