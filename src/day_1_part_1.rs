use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut elves_calories: HashMap<i32, i32> = HashMap::new();
        // Consumes the iterator, returns an (Optional) String
        let mut elves_count = 1;
        let mut current_elves_calories = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    elves_calories.insert(elves_count, current_elves_calories);
                    elves_count += 1;
                    current_elves_calories = 0;
                    continue;
                }
                current_elves_calories += ip.parse::<i32>().unwrap();
            }
        }
        dbg!(elves_calories
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(_k, v)| v));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
