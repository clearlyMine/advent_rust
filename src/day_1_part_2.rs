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
        let top_3 = find_top_n_based_on_calories(elves_calories, 3);
        let mut total_top_3 = 0;
        for c in &top_3 {
            total_top_3 += c;
        }
        dbg!(total_top_3);
    }
}

fn find_top_n_based_on_calories(elves_calories: HashMap<i32, i32>, n: usize) -> Vec<i32> {
    let mut top_n: Vec<i32> = vec![0; n];
    //iterate over the map one by one
    //compare the value with the last value of the vector
    //if bigger
    //insert into the vector which inserts at the last (n-1) position then sorts the vector
    //else continue
    for (_k, v) in elves_calories {
        if v > top_n[0] {
            top_n[0] = v;
            top_n.sort();
        }
    }

    top_n
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
