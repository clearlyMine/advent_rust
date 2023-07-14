use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;
const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSS: i32 = 0;

const ROCK_OPPONENT_SYMBOL: char = 'A';
const PAPER_OPPONENT_SYMBOL: char = 'B';
const SCISSORS_OPPONENT_SYMBOL: char = 'C';
const LOSS_SYMBOL: char = 'X';
const DRAW_SYMBOL: char = 'Y';
const WIN_SYMBOL: char = 'Z';

fn main() {
    if let Ok(lines) = read_lines("./input_day_2.txt") {
        let mut total_points = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    panic!("line is empty");
                }
                let result_points: i32;
                let played_points: i32;
                let mut letters = ip.chars();
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
        }
        dbg!(total_points);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
