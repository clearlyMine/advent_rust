use std::time::Instant;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;
const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSS: i32 = 0;

const ROCK_OPPONENT_SYMBOL: char = 'A';
const PAPER_OPPONENT_SYMBOL: char = 'B';
const SCISSORS_OPPONENT_SYMBOL: char = 'C';
const ROCK_MY_SYMBOL: char = 'X';
const PAPER_MY_SYMBOL: char = 'Y';
const SCISSORS_MY_SYMBOL: char = 'Z';

const LOSS_SYMBOL: char = 'X';
const DRAW_SYMBOL: char = 'Y';
const WIN_SYMBOL: char = 'Z';

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day2.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> i32 {
    let lines = input.lines();
    let mut total_points = 0;
    for line in lines {
        if line.is_empty() {
            panic!("line is empty");
        }
        let result_points: i32;
        let played_points: i32;
        let mut letters = line.chars();
        let opponents_hand = letters.next().unwrap();
        let my_hand = letters.nth(1).unwrap();

        match opponents_hand {
            ROCK_OPPONENT_SYMBOL => match my_hand {
                ROCK_MY_SYMBOL => {
                    played_points = ROCK;
                    result_points = DRAW;
                }
                PAPER_MY_SYMBOL => {
                    played_points = PAPER;
                    result_points = WIN;
                }
                SCISSORS_MY_SYMBOL => {
                    played_points = SCISSORS;
                    result_points = LOSS;
                }
                _ => {
                    panic!("symbol is not recognised {}", my_hand);
                }
            },
            PAPER_OPPONENT_SYMBOL => match my_hand {
                ROCK_MY_SYMBOL => {
                    played_points = ROCK;
                    result_points = LOSS;
                }
                PAPER_MY_SYMBOL => {
                    played_points = PAPER;
                    result_points = DRAW;
                }
                SCISSORS_MY_SYMBOL => {
                    played_points = SCISSORS;
                    result_points = WIN;
                }
                _ => {
                    panic!("symbol is not recognised {}", my_hand);
                }
            },
            SCISSORS_OPPONENT_SYMBOL => match my_hand {
                ROCK_MY_SYMBOL => {
                    played_points = ROCK;
                    result_points = WIN;
                }
                PAPER_MY_SYMBOL => {
                    played_points = PAPER;
                    result_points = LOSS;
                }
                SCISSORS_MY_SYMBOL => {
                    played_points = SCISSORS;
                    result_points = DRAW;
                }
                _ => {
                    panic!("symbol is not recognised {}", my_hand);
                }
            },
            _ => {
                panic!("symbol is not recognised {}", my_hand);
            }
        }
        total_points += result_points + played_points;
    }
    return total_points;
}

fn process_part_2(input: &str) -> i32 {
    let lines = input.lines();
    let mut total_points = 0;
    for line in lines {
        if line.is_empty() {
            panic!("line is empty");
        }
        let result_points: i32;
        let played_points: i32;
        let mut letters = line.chars();
        let opponents_hand = letters.next().unwrap();
        let my_hand = letters.nth(1).unwrap();

        match opponents_hand {
            ROCK_OPPONENT_SYMBOL => match my_hand {
                DRAW_SYMBOL => {
                    played_points = ROCK;
                    result_points = DRAW;
                }
                WIN_SYMBOL => {
                    played_points = PAPER;
                    result_points = WIN;
                }
                LOSS_SYMBOL => {
                    played_points = SCISSORS;
                    result_points = LOSS;
                }
                _ => {
                    panic!("symbol is not recognised {}", my_hand);
                }
            },
            PAPER_OPPONENT_SYMBOL => match my_hand {
                LOSS_SYMBOL => {
                    played_points = ROCK;
                    result_points = LOSS;
                }
                DRAW_SYMBOL => {
                    played_points = PAPER;
                    result_points = DRAW;
                }
                WIN_SYMBOL => {
                    played_points = SCISSORS;
                    result_points = WIN;
                }
                _ => {
                    panic!("symbol is not recognised {}", my_hand);
                }
            },
            SCISSORS_OPPONENT_SYMBOL => match my_hand {
                WIN_SYMBOL => {
                    played_points = ROCK;
                    result_points = WIN;
                }
                LOSS_SYMBOL => {
                    played_points = PAPER;
                    result_points = LOSS;
                }
                DRAW_SYMBOL => {
                    played_points = SCISSORS;
                    result_points = DRAW;
                }
                _ => {
                    panic!("symbol is not recognised {}", my_hand);
                }
            },
            _ => {
                panic!("symbol is not recognised {}", my_hand);
            }
        }
        total_points += result_points + played_points;
    }
    total_points
}
