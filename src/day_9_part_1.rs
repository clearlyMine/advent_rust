use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

fn main() {
    let b = include_str!("../input_day_9.txt");

    let mut head_position: Coordinates = Coordinates { x: 0, y: 0 };
    let mut tail_position: Coordinates = Coordinates { x: 0, y: 0 };
    let mut tail_visits: HashSet<Coordinates> = HashSet::new();
    tail_visits.insert(tail_position.clone());
    let lines = b.lines();
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
    println!("{}", tail_visits.len());
}
