use itertools::Itertools;
use std::hash::{Hash, Hasher};
use std::{collections::HashMap, collections::HashSet, time::Instant};

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
    let mut beaming: HashSet<Coord> = HashSet::new();

    let start_position = Coord::new(0, 0);
    beaming.insert(start_position);
    find_all_beaming(
        &grid,
        start_position,
        Direction::Right,
        &mut beaming,
        &mut HashSet::new(),
    );
    // dbg!(beaming.clone());
    beaming.len()
}

fn process_part_2(input: &str) -> usize {
    let grid = parse_grid(input);

    let mut max_energised = 0;
    let height = grid.len();
    let width = grid[0].len();
    for row in 0..height {
        for (col, direction) in [(0, Direction::Right), (width - 1, Direction::Left)] {
            let start_position = Coord::new(row, col);
            let mut beaming: HashSet<Coord> = HashSet::new();
            beaming.insert(start_position);

            find_all_beaming(
                &grid,
                start_position,
                direction,
                &mut beaming,
                &mut HashSet::new(),
            );
            max_energised = max_energised.max(beaming.len());
        }
    }
    for col in 0..width {
        for (row, direction) in [(0, Direction::Down), (height - 1, Direction::Up)] {
            let start_position = Coord::new(row, col);

            let mut beaming: HashSet<Coord> = HashSet::new();
            beaming.insert(start_position);

            find_all_beaming(
                &grid,
                start_position,
                direction,
                &mut beaming,
                &mut HashSet::new(),
            );
            max_energised = max_energised.max(beaming.len());
        }
    }
    max_energised
}

fn find_all_beaming(
    grid: &Vec<Vec<char>>,
    position: Coord,
    direction: Direction,
    beaming: &mut HashSet<Coord>,
    cache: &mut HashSet<(Coord, Direction)>,
) {
    if cache.contains(&(position, direction)) {
        return;
    }
    let width = grid[0].len();
    let height = grid.len();
    let mut position = position;
    let mut direction = direction;
    let mut cur = grid[position.row][position.col];
    loop {
        // dbg!(cur);
        match cur {
            '/' => match direction {
                Direction::Right => direction = Direction::Up,
                Direction::Left => direction = Direction::Down,
                Direction::Up => direction = Direction::Right,
                Direction::Down => direction = Direction::Left,
            },
            '\\' => match direction {
                Direction::Right => direction = Direction::Down,
                Direction::Left => direction = Direction::Up,
                Direction::Up => direction = Direction::Left,
                Direction::Down => direction = Direction::Right,
            },
            '-' => match direction {
                Direction::Up | Direction::Down => {
                    // println!("Splitting at {:?}", position);
                    // println!("Right split");
                    find_all_beaming(grid, position, Direction::Right, beaming, cache);
                    // println!("Left split at {:?}", position);
                    find_all_beaming(grid, position, Direction::Left, beaming, cache);
                    break;
                }
                _ => {}
            },
            '|' => match direction {
                Direction::Left | Direction::Right => {
                    // println!("Splitting at {:?}", position);
                    // println!("Up split");
                    find_all_beaming(grid, position, Direction::Up, beaming, cache);
                    // println!("Down split at {:?}", position);
                    find_all_beaming(grid, position, Direction::Down, beaming, cache);
                    break;
                }
                _ => {}
            },
            '.' => {}
            _ => panic! {"WTF"},
        }
        // dbg!(direction);
        let p = match direction {
            Direction::Up => position.get_north(),
            Direction::Left => position.get_west(),
            Direction::Down => position.get_south(),
            Direction::Right => position.get_east(),
        };
        // dbg!(p);
        if let Some(new_position) = p {
            if new_position.row < height && new_position.col < width {
                cur = grid[new_position.row][new_position.col];
                beaming.insert(new_position);
                position = new_position;
                if cache.contains(&(position, direction)) {
                    // println!("Already been here");
                    // println!("-------------------\n\n");
                    break;
                }
                cache.insert((position, direction));
            } else {
                // println!("Beam is out of grid");
                // println!("-------------------\n\n");
                break;
            }
        } else {
            // println!("Beam is out of grid");
            // println!("-------------------\n\n");
            break;
        }
        // print_beamed_grid(&beaming, width, height);
    }
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

#[derive(PartialEq, Eq, Hash)]
struct Grid(Vec<Vec<Land>>);

struct Row(Vec<Land>);

impl Hash for Row {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

fn tilt_east(grid: &mut Vec<Vec<Land>>, height: usize, width: usize) {
    for row in 0..height {
        let row_contents = &*grid[row].clone();
        let mut current_right = width - 1;
        for (i, char) in row_contents.iter().rev().enumerate() {
            let col = width - i - 1;
            match char {
                Land::MovableRock => {
                    grid[row][col] = Land::Ground;
                    grid[row][current_right] = Land::MovableRock;
                    if current_right == 0 {
                        break;
                    }
                    current_right -= 1;
                }
                Land::ImmovableRock => {
                    if col == 0 {
                        break;
                    }
                    current_right = col - 1;
                }
                _ => {}
            }
        }
    }
    // print_grid(&grid);
}

fn calculate_load_on_north_beam(grid: &[Vec<Land>]) -> usize {
    let height = grid.len();
    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|c| c == &&Land::MovableRock).count() * (height - i))
        .sum()
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

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Land>>) {
    let mut f: String = "".to_string();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = match grid[i][j] {
                Land::Ground => '.',
                Land::MovableRock => 'O',
                _ => '#',
            };
            f = format!("{}{}", f, c);
        }
        f = format!("{}\n", f);
    }
    println!("{}", f);
}

fn get_col(grid: &[Vec<Land>], col: usize) -> Vec<Land> {
    grid.iter().map(|row| row[col]).collect::<Vec<Land>>()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Land {
    Ground,
    MovableRock,
    ImmovableRock,
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines().collect_vec();
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
        assert_eq!(process_part_1(SAMPLE), 0)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 0)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 0)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 0)
    }
}
