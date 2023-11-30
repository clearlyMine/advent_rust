use std::collections::{BTreeMap, VecDeque};

fn main() {
    let input = include_str!(".../inputs/day24.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {}", res1);
    let res2 = process_part_2(input);
    println!("Part 2: {}", res2);
}

fn process_part_1(input: &str) -> usize {
    let grid: BTreeMap<Coord, Tile> = parse_input(input);
    // display_grid(&grid);
    let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
    // let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();

    let starting_position: Vec<&Coord> = grid
        .iter()
        .filter(|(coord, tile)| coord.row == 0 && **tile == Tile::Ground)
        .map(|(coord, _)| coord)
        .collect();
    if starting_position.len() > 1 {
        panic!("More than 1 starting positions");
    }
    let starting_position = starting_position.iter().last().unwrap();
    let destination: Vec<&Coord> = grid
        .iter()
        .filter(|(coord, tile)| coord.row == max_row && **tile == Tile::Ground)
        .map(|(coord, _)| coord)
        .collect();
    if destination.len() > 1 {
        panic!("More than 1 destinations");
    }
    let destination = destination.iter().last().unwrap();
    // dbg!(starting_position);
    // dbg!(destination);
    find_path_bfs(&grid, **starting_position, *destination)
}

#[derive(Clone, Debug)]
struct State {
    current_position: Coord,
    round: usize,
}

impl State {
    fn new(current_position: Coord, round: usize) -> State {
        State {
            current_position,
            round,
        }
    }

    fn is_valid(&self, grid: &BTreeMap<Coord, Tile>) -> bool {
        let current_position = self.current_position;
        if !grid.contains_key(&current_position) {
            return false;
        }
        let tile = grid.get(&current_position).unwrap();
        if *tile == Tile::Wall {
            return false;
        }
        let round = self.round as isize;

        let row = current_position.row as isize;
        let col = current_position.col as isize;
        //look around the grid current_position in every direction
        let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap() - 1;
        let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap() - 1;
        let max_row = max_row as isize;
        let max_col = max_col as isize;

        //from Up 'v'
        let mut north_row: isize = row - round % max_row;
        if north_row <= 0 {
            north_row = max_row + north_row;
            if north_row <= 0 {
                panic!();
            }
        }
        let north: Coord = Coord::new(north_row as usize, col as usize);
        if *grid.get(&north).unwrap() == Tile::Blizzard(Direction::South) {
            return false;
        }

        //from Down '^'
        let mut south_row = row + round % max_row;
        if south_row > max_row {
            south_row -= max_row;
            if south_row <= 0 {
                panic!();
            }
        }
        let south: Coord = Coord::new(south_row as usize, col as usize);
        if *grid.get(&south).unwrap() == Tile::Blizzard(Direction::North) {
            return false;
        }

        //from West '>'
        let mut west_col = col - round % max_col;
        if west_col <= 0 {
            west_col = max_col + west_col;
            if west_col <= 0 {
                panic!();
            }
        }
        let west: Coord = Coord::new(row as usize, west_col as usize);
        if *grid.get(&west).unwrap() == Tile::Blizzard(Direction::East) {
            return false;
        }

        //from East '<'
        let mut east_col = col + round % max_col;
        if east_col > max_col {
            east_col -= max_col;
            if east_col <= 0 {
                panic!();
            }
        }
        let east: Coord = Coord::new(row as usize, east_col as usize);
        if *grid.get(&east).unwrap() == Tile::Blizzard(Direction::West) {
            return false;
        }

        true
    }
}

fn find_path_bfs(
    grid: &BTreeMap<Coord, Tile>,
    starting_position: Coord,
    destination: &Coord,
) -> usize {
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_front(State::new(starting_position, 0));
    let max_blizzard_moves = grid
        .iter()
        .flat_map(|(coord, _)| vec![coord.row, coord.col])
        .max()
        .unwrap()
        - 2;
    let mut wait_at_start = 0;

    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        let current_position = state.current_position;
        let round = state.round;
        if current_position == *destination {
            return round;
        }

        // display_grid(&new_grid);
        //No Move
        //as no blizzard ever moves to the starting postion, this will never move
        if current_position == starting_position {
            //waiting at max for all the blizzards to move back to the same positions as at the
            //start
            if wait_at_start <= max_blizzard_moves {
                let new_state = State::new(current_position, round + 1);
                queue.push_back(new_state.clone());
                wait_at_start += 1;
            } else {
                wait_at_start = 0;
            }
        } else {
            let new_state = State::new(current_position, round + 1);
            if new_state.is_valid(grid) {
                queue.push_back(new_state.clone());
            }
        }
        //Move North
        if current_position.row != 0 {
            let new_position = Coord::new(current_position.row - 1, current_position.col);
            let new_state = State::new(new_position, round + 1);
            if new_state.is_valid(grid) {
                queue.push_back(new_state.clone());
            }
        }
        //Move South
        let new_position = Coord::new(current_position.row + 1, current_position.col);
        let new_state = State::new(new_position, round + 1);
        if new_state.is_valid(grid) {
            queue.push_back(new_state.clone());
        }
        //Move West
        if current_position.col != 0 {
            let new_position = Coord::new(current_position.row, current_position.col - 1);
            let new_state = State::new(new_position, round + 1);
            if new_state.is_valid(grid) {
                queue.push_back(new_state.clone());
            }
        }
        //Move East
        let new_position = Coord::new(current_position.row, current_position.col + 1);
        let new_state = State::new(new_position, round + 1);
        if new_state.is_valid(grid) {
            queue.push_back(new_state.clone());
        }
    }
    panic!()
}

fn process_part_2(input: &str) -> usize {
    let grid = parse_input(input);
    0
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Ground,
    Blizzard(Direction),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(row: usize, col: usize) -> Coord {
        Coord { row, col }
    }

    // fn get_next_coord(&self, direction: Direction, grid: &mut BTreeMap<Coord, Vec<Tile>>) -> Coord {
    //     todo!()
    // }
}

fn display_grid(grid: &BTreeMap<Coord, Tile>) {
    let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
    let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();
    for row in 0..=max_row {
        for col in 0..=max_col {
            let coord = Coord { row, col };
            if let Some(tile) = grid.get(&coord) {
                match tile {
                    Tile::Ground => print!("."),
                    Tile::Wall => print!("#"),
                    Tile::Blizzard(direction) => match direction {
                        Direction::North => print!("^"),
                        Direction::South => print!("v"),
                        Direction::East => print!(">"),
                        Direction::West => print!("<"),
                    },
                }
            } else {
                break;
            }
        }
        println!();
    }
}

fn parse_input(input: &str) -> BTreeMap<Coord, Tile> {
    let mut grid: BTreeMap<Coord, Tile> = BTreeMap::new();
    let mut row = 0;
    for line in input.lines() {
        let mut col = 0;
        for char in line.chars() {
            let coord = Coord::new(row, col);
            let tile = match char {
                '.' => Tile::Ground,
                '#' => Tile::Wall,
                '^' => Tile::Blizzard(Direction::North),
                'v' => Tile::Blizzard(Direction::South),
                '>' => Tile::Blizzard(Direction::East),
                '<' => Tile::Blizzard(Direction::West),
                _ => panic!(),
            };
            col += 1;
            grid.insert(coord, tile);
        }
        row += 1;
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part_1() {
        let input = include_str!(".../inputs/day24_sample.txt");
        assert_eq!(process_part_1(input), 18);
    }

    #[test]
    fn test_process_part_2() {
        let input = include_str!(".../inputs/day24_sample.txt");
        assert_eq!(process_part_2(input), 20);
    }
}
