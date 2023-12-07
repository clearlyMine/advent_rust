use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/day07.txt");

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn new(number: char) -> Card {
        match number {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!(),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = *self as isize;
        let y = *other as isize;
        x.cmp(&y)
    }
}

#[derive(Eq, PartialEq, PartialOrd, Clone, Copy, Debug)]
enum HandType {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = *self as isize;
        let y = *other as isize;
        x.cmp(&y)
    }
}

fn get_hand_type(map: HashMap<char, usize>) -> HandType {
    let jokers_present = *map.get(&'J').unwrap_or(&0);
    match map.len() {
        1 => HandType::Five,
        2 => {
            let mut x: HandType = HandType::Four;
            for (_, val) in map {
                if val == 4 || val == 1 {
                    x = match jokers_present {
                        1 | 4 => HandType::Five,
                        _ => HandType::Four,
                    };
                    break;
                } else {
                    x = match jokers_present {
                        2 | 3 => HandType::Five,
                        _ => HandType::Full,
                    };
                    break;
                }
            }
            x
        }
        3 => {
            let mut x: HandType = HandType::Two;
            for (_, val) in map {
                if val == 3 {
                    x = match jokers_present {
                        1 | 3 => HandType::Four,
                        _ => HandType::Three,
                    };
                    break;
                } else if val == 2 {
                    x = match jokers_present {
                        1 => HandType::Full,
                        2 => HandType::Four,
                        _ => HandType::Two,
                    };
                    break;
                }
            }
            x
        }
        4 => match jokers_present {
            1 | 2 => HandType::Three,
            _ => HandType::One,
        },
        5 => match jokers_present {
            1 => HandType::One,
            _ => HandType::High,
        },
        _ => panic!(),
    }
}

fn compare_hands(first: &str, second: &str) -> std::cmp::Ordering {
    if first == second {
        return Ordering::Equal;
    }

    let cards_self = first.chars().collect::<Vec<char>>();
    let mut cards_first: HashMap<char, usize> = HashMap::new();
    for card in cards_self.clone() {
        cards_first
            .entry(card)
            .and_modify(|num| *num += 1)
            .or_insert(1);
    }

    let cards_other = second.chars().collect::<Vec<char>>();
    let mut cards_second: HashMap<char, usize> = HashMap::new();
    for card in cards_other.clone() {
        cards_second
            .entry(card)
            .and_modify(|num| *num += 1)
            .or_insert(1);
    }

    let first_type = get_hand_type(cards_first);
    let second_type = get_hand_type(cards_second);

    if first_type > second_type {
        return Ordering::Greater;
    }
    if first_type < second_type {
        return Ordering::Less;
    }

    for i in 0..5 {
        if cards_self[i] != cards_other[i] {
            let a = Card::new(cards_self[i]);
            let b = Card::new(cards_other[i]);
            return a.cmp(&b);
        }
    }

    Ordering::Equal
}

fn process_part_2(input: &str) -> usize {
    let mut map: Vec<(&str, usize)> = vec![];
    input.lines().for_each(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid = bid.parse::<usize>().unwrap();
        map.push((hand, bid));
    });
    map.sort_by(|a, b| compare_hands(&a.0, &b.0));
    map.into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}
