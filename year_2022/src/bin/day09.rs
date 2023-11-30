use std::collections::HashSet;
use std::time::Instant;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day9.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let mut head_position: Coordinates = Coordinates { x: 0, y: 0 };
    let mut tail_position: Coordinates = Coordinates { x: 0, y: 0 };
    let mut tail_visits: HashSet<Coordinates> = HashSet::new();
    tail_visits.insert(tail_position.clone());
    let lines = input.lines();
    for line in lines {
        let mut words = line.split(' ');
        let m: &str = words.next().unwrap();
        let movement: i32 = words.next().unwrap().parse::<i32>().unwrap();
        match m {
            "L" => {
                for _ in 0..movement {
                    head_position.y -= 1;
                    if head_position.y - tail_position.y == -2 {
                        tail_position.x = head_position.x;
                        tail_position.y -= 1;
                    }
                    tail_visits.insert(tail_position.clone());
                }
            }
            "R" => {
                for _ in 0..movement {
                    head_position.y += 1;
                    if head_position.y - tail_position.y == 2 {
                        tail_position.x = head_position.x;
                        tail_position.y += 1;
                    }
                    tail_visits.insert(tail_position.clone());
                }
            }
            "U" => {
                for _ in 0..movement {
                    head_position.x += 1;
                    if head_position.x - tail_position.x == 2 {
                        tail_position.x += 1;
                        tail_position.y = head_position.y;
                    }
                    tail_visits.insert(tail_position.clone());
                }
            }
            "D" => {
                for _ in 0..movement {
                    head_position.x -= 1;
                    if head_position.x - tail_position.x == -2 {
                        tail_position.x -= 1;
                        tail_position.y = head_position.y;
                    }
                    tail_visits.insert(tail_position.clone());
                }
            }
            _ => {
                panic!("invalid movement");
            }
        }
    }
    tail_visits.len()
}

fn process_part_2(input: &str) -> usize {
    let mut head: Coordinates = Coordinates { x: 0, y: 0 };
    let mut knot_positions: [Coordinates; 9] = [Coordinates::new(); 9];
    let mut tail_visits: HashSet<Coordinates> = HashSet::new();
    tail_visits.insert(knot_positions[8].clone());
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let mut words = line.split(' ');
        // println!(
        //     "line[0]={},line[1]={},line[2]={}",
        //     line[0],
        //     line[1],
        //     line[2] - 48
        // );
        let direction: &str = words.next().unwrap();
        let movement: i32 = words.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "L" => {
                for _ in 0..movement {
                    head.y -= 1;
                    let mut current_head: Coordinates = head;
                    for i in 0..knot_positions.len() {
                        let current_tail = knot_positions[i];
                        current_head = get_new_tail_coordinates(current_head, current_tail);
                        knot_positions[i] = current_head;
                    }
                    tail_visits.insert(current_head);
                }
            }
            "R" => {
                for _ in 0..movement {
                    head.y += 1;
                    let mut current_head: Coordinates = head;
                    for i in 0..knot_positions.len() {
                        let current_tail = knot_positions[i];
                        current_head = get_new_tail_coordinates(current_head, current_tail);
                        knot_positions[i] = current_head;
                    }
                    tail_visits.insert(current_head);
                }
            }
            "U" => {
                for _ in 0..movement {
                    head.x += 1;
                    let mut current_head: Coordinates = head;
                    for i in 0..knot_positions.len() {
                        let current_tail = knot_positions[i];
                        current_head = get_new_tail_coordinates(current_head, current_tail);
                        knot_positions[i] = current_head;
                    }
                    tail_visits.insert(current_head);
                }
            }
            "D" => {
                for _ in 0..movement {
                    head.x -= 1;
                    let mut current_head: Coordinates = head;
                    for i in 0..knot_positions.len() {
                        let current_tail = knot_positions[i];
                        current_head = get_new_tail_coordinates(current_head, current_tail);
                        knot_positions[i] = current_head;
                    }
                    tail_visits.insert(current_head);
                }
            }
            _ => {
                panic!("invalid movement");
            }
        }
    }
    tail_visits.len()
}

fn get_new_tail_coordinates(head: Coordinates, tail: Coordinates) -> Coordinates {
    //assumes head has already moved

    let mut new_tail: Coordinates = Coordinates::new();
    let x_difference = head.x - tail.x;
    let y_difference = head.y - tail.y;

    if x_difference == -2 {
        if y_difference < 0 {
            new_tail.y = tail.y - 1;
        }
        if y_difference == 0 {
            new_tail.y = tail.y;
        }
        if y_difference > 0 {
            new_tail.y = tail.y + 1;
        }
        new_tail.x = tail.x - 1;
        return new_tail;
    }

    if x_difference == 2 {
        if y_difference < 0 {
            new_tail.y = tail.y - 1;
        }
        if y_difference == 0 {
            new_tail.y = tail.y;
        }
        if y_difference > 0 {
            new_tail.y = tail.y + 1;
        }
        new_tail.x = tail.x + 1;
        return new_tail;
    }
    if y_difference == -2 {
        if x_difference < 0 {
            new_tail.x = tail.x - 1;
        }
        if x_difference == 0 {
            new_tail.x = tail.x;
        }
        if x_difference > 0 {
            new_tail.x = tail.x + 1;
        }
        new_tail.y = tail.y - 1;
        return new_tail;
    }
    if y_difference == 2 {
        if x_difference < 0 {
            new_tail.x = tail.x - 1;
        }
        if x_difference == 0 {
            new_tail.x = tail.x;
        }
        if x_difference > 0 {
            new_tail.x = tail.x + 1;
        }
        new_tail.y = tail.y + 1;
        return new_tail;
    }
    tail
}
