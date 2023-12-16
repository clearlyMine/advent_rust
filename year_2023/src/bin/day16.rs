use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day16.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day16_sample.txt");

fn main() {
    let time_start = Instant::now();
    let res1 = process_part_1(INPUT);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(INPUT);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let grid = parse_grid(input);
    get_beaming_count(&grid, Coord::new(0, 0), Direction::Right)
}

fn process_part_2(input: &str) -> usize {
    let grid = parse_grid(input);
    let height = grid.len();
    let width = grid[0].len();
    let grid = Arc::new(grid);

    let mut handles = vec![];
    let mut results = vec![];

    for row in 0..height {
        for (col, direction) in [(0, Direction::Right), (width - 1, Direction::Left)] {
            let grid = Arc::clone(&grid);
            let handle = thread::spawn(move || {
                let start_position = Coord::new(row, col);
                get_beaming_count(&grid, start_position, direction)
            });
            handles.push(handle);
        }
    }
    for col in 0..width {
        for (row, direction) in [(0, Direction::Down), (height - 1, Direction::Up)] {
            let grid = Arc::clone(&grid);
            let handle = thread::spawn(move || {
                let start_position = Coord::new(row, col);
                get_beaming_count(&grid, start_position, direction)
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        results.push(handle.join().unwrap());
    }
    *results.iter().max().unwrap()
}

fn get_beaming_count(grid: &Vec<Vec<char>>, position: Coord, direction: Direction) -> usize {
    let mut queue: Vec<(Coord, Direction)> = vec![(position, direction)];

    let width = grid[0].len();
    let height = grid.len();

    let mut visited: HashSet<(Coord, Direction)> = HashSet::new();
    visited.insert((position, direction));

    while let Some((position, direction)) = queue.pop() {
        let new_directions: Vec<Direction> = match grid[position.row][position.col] {
            '/' => match direction {
                Direction::Right => vec![Direction::Up],
                Direction::Left => vec![Direction::Down],
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
            },
            '\\' => match direction {
                Direction::Right => vec![Direction::Down],
                Direction::Left => vec![Direction::Up],
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
            },
            '-' => match direction {
                Direction::Up | Direction::Down => vec![Direction::Right, Direction::Left],
                _ => vec![direction],
            },
            '|' => match direction {
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
                _ => vec![direction],
            },
            '.' => vec![direction],
            _ => panic! {"WTF"},
        };
        // dbg!(new_directions.clone());
        for direction in new_directions {
            let p = match direction {
                Direction::Up => position.get_north(),
                Direction::Left => position.get_west(),
                Direction::Down => position.get_south(),
                Direction::Right => position.get_east(),
            };
            // dbg!(p);
            if let Some(new_position) = p {
                if new_position.row < height && new_position.col < width {
                    if !visited.contains(&(new_position, direction)) {
                        visited.insert((new_position, direction));
                        queue.push((new_position, direction));
                    }
                }
            }
        }
        // print_beamed_grid(&beaming, width, height);
    }
    // dbg!(visited.clone());
    visited
        .into_iter()
        .map(|(p, _)| p)
        .collect::<HashSet<Coord>>()
        .len()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Coord {
        Coord { row, col }
    }

    fn get_south(&self) -> Option<Coord> {
        Some(Coord {
            row: self.row + 1,
            col: self.col,
        })
    }

    fn get_east(&self) -> Option<Coord> {
        Some(Coord {
            row: self.row,
            col: self.col + 1,
        })
    }

    fn get_west(&self) -> Option<Coord> {
        if self.col == 0 {
            return None;
        }
        Some(Coord {
            row: self.row,
            col: self.col - 1,
        })
    }

    fn get_north(&self) -> Option<Coord> {
        if self.row == 0 {
            return None;
        }
        Some(Coord {
            row: self.row - 1,
            col: self.col,
        })
    }
}

#[allow(dead_code)]
fn print_beamed_grid(beamed: &HashSet<Coord>, width: usize, height: usize) {
    let mut f: String = "".to_string();
    for i in 0..height {
        for j in 0..width {
            let c = if beamed.contains(&Coord::new(i, j)) {
                '#'
            } else {
                '.'
            };
            f = format!("{}{}", f, c);
        }
        f = format!("{}\n", f);
    }
    println!("{}", f);
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
    lines.iter().enumerate().for_each(|(row, line)| {
        for (col, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            grid[row][col] = char;
        }
    });
    grid
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 46)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 6902)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 51)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 7697)
    }
}
