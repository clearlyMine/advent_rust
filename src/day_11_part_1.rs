use std::collections::HashMap;
fn main() {
    let b = include_str!("../input_day_11.txt");

    let lines = b.lines();
    let mut starting_items: Vec<Vec<i32>> = Vec::new();
    let mut operations: Vec<(char, &str)> = vec![];
    let mut tests: Vec<i32> = vec![];
    let mut test_results: HashMap<(usize, bool), usize> = HashMap::new();
    let mut current_monkey: usize = 0;
    //parse input
    for line in lines {
        // println!("{}", line);
        if line.is_empty() {
            continue;
        }
        let words: Vec<&str> = line.split(' ').collect();
        if words[0] == "Monkey" {
            // println!("Monkey");
            current_monkey = words[1].chars().next().unwrap() as usize - 48;
            continue;
        }
        if words[2] == "Starting" {
            // println!("Starting");
            let mut starting: Vec<i32> = vec![];
            for word in words.iter().skip(4) {
                let letters = word.chars();
                let mut number = "".to_string();
                for letter in letters {
                    if letter != ',' {
                        number = format!("{}{}", number.to_owned(), letter);
                    }
                }
                starting.push(number.parse::<i32>().unwrap());
            }
            starting_items.push(starting);
            continue;
        }
        if words[2] == "Operation:" {
            // println!("Operation");
            operations.push((words[6].chars().next().unwrap(), words[7]));
            continue;
        }
        if words[2] == "Test:" {
            // println!("Test");
            tests.push(words[5].parse::<i32>().unwrap());
            continue;
        }
        if words[5] == "true:" {
            // println!("true");
            test_results.insert((current_monkey, true), words[9].parse::<usize>().unwrap());
            continue;
        }
        if words[5] == "false:" {
            // println!("false");
            test_results.insert((current_monkey, false), words[9].parse::<usize>().unwrap());
            continue;
        }
        // println!("couldn't parse line");
    }
    // println!("starting_items: {:?}", starting_items);
    // println!("operations: {:?}", operations);
    // println!("tests: {:?}", tests);
    // println!("test_results: {:?}", test_results);
    let mut items_inpect_number: [i32; 8] = [0; 8];

    for _ in 0..20 {
        for monkey_number in 0..starting_items.len() {
            // println!();
            // println!("Processing for monkey {}:", monkey_number);
            let mut count = 0;
            while let Some(worry) = starting_items[monkey_number].first().cloned() {
                // println!("Original Worry {}", worry);
                starting_items[monkey_number].remove(0);
                count += 1;
                let mut new_worry: i32;
                let ops = operations[monkey_number];
                if ops.0 == '*' {
                    if ops.1 == "old" {
                        new_worry = worry * worry;
                    } else {
                        new_worry = worry * ops.1.parse::<i32>().unwrap();
                    }
                } else if ops.0 == '+' {
                    if ops.1 == "old" {
                        new_worry = worry + worry;
                    } else {
                        new_worry = worry + ops.1.parse::<i32>().unwrap();
                    }
                } else {
                    panic!("unknown operation {:?}", ops);
                }
                // println!("Worry after operation {}", new_worry);
                new_worry /= 3;
                // println!("Worry after bored {}", new_worry);
                //perform test
                let next_monkey = test_results
                    .get(&(
                        monkey_number,
                        new_worry % tests.get(monkey_number).unwrap() == 0,
                    ))
                    .unwrap()
                    .to_owned();
                starting_items[next_monkey].push(new_worry);
                // println!("Pushing to monkey {}", next_monkey);
            }
            items_inpect_number[monkey_number] += count;
            // starting_items = new_starting_items;
        }
    }
    // println!("{:?}", starting_items);
    // println!("{:?}", items_inpect_number);
    items_inpect_number.sort_unstable();
    items_inpect_number.reverse();
    println!("{:?}", items_inpect_number[0] * items_inpect_number[1]);
}
