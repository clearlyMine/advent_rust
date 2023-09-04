use std::collections::HashMap;

fn main() {
    let input = include_str!("../inputs/day1.txt");

    let mut elves_calories: HashMap<i32, i32> = HashMap::new();
    // Consumes the iterator, returns an (Optional) String
    let mut elves_count = 1;
    let mut current_elves_calories = 0;

    let lines = input.lines();
    for line in lines {
        if line.is_empty() {
            elves_calories.insert(elves_count, current_elves_calories);
            elves_count += 1;
            current_elves_calories = 0;
            continue;
        }
        current_elves_calories += line.parse::<i32>().unwrap();
    }
    let res1 = process_part_1(elves_calories.clone());
    println!("Part 1: {:?}", res1);
    let res2 = process_part_2(elves_calories);
    println!("Part 2: {:?}", res2);
}

fn process_part_1(elves_calories: HashMap<i32, i32>) -> i32 {
    *elves_calories
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(_k, v)| v)
        .unwrap()
}

fn process_part_2(elves_calories: HashMap<i32, i32>) -> i32 {
    let top_3 = find_top_n_based_on_calories(elves_calories, 3);
    let mut total_top_3 = 0;
    for c in &top_3 {
        total_top_3 += c;
    }
    total_top_3
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
