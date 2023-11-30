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
    //recursive function with an option return type
    //recursively go through all currently available spots
    //changing the current position to the new position
    //if a blizzard moves in the current position then return None
    //if nothing happens then continue recursively moving until the end_point is
    //reached
    //for all the returned counts keep comparing the Some values and return the lowest score
    let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
    // let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();

    let starting_position: Vec<&Coord> = grid
        .iter()
        .filter(|(coord, tile)| coord.row == 0 && **tile == Tile::Ground)
        .map(|(coord, _)| coord)
        .collect();
    if starting_position.len() > 1 {
        panic!();
    }
    let starting_position = starting_position.iter().last().unwrap();
    let destination: Vec<&Coord> = grid
        .iter()
        .filter(|(coord, tile)| coord.row == max_row && **tile == Tile::Ground)
        .map(|(coord, _)| coord)
        .collect();
    if destination.len() > 1 {
        panic!();
    }
    let destination = destination.iter().last().unwrap();
    // dbg!(starting_position);
    // dbg!(destination);
    find_path_bfs(&grid, **starting_position, *destination)
}

#[derive(Clone, Debug)]
struct State {
    current_position: Coord,
    grid: BTreeMap<Coord, Tile>,
    round: usize,
}

impl State {
    fn new(current_position: Coord, grid: &BTreeMap<Coord, Tile>, round: usize) -> State {
        State {
            current_position,
            grid: grid.clone(),
            round,
        }
    }

    fn is_valid(&self) -> bool {
        let current_position = self.current_position;
        let grid = &self.grid;

        if !grid.contains_key(&current_position) {
            return false;
        }
        let tile = grid.get(&current_position).unwrap();
        if *tile != Tile::Ground {
            return false;
        }
        true
    }

    fn is_same(&self, new: &State) -> bool {
        self.current_position == new.current_position && self.grid == new.grid
    }
}

fn states_contains(states: &Vec<State>, check: &State) -> bool {
    states.iter().filter(|state| state.is_same(check)).count() == 1
}

fn find_path(grid: &BTreeMap<Coord, Tile>, starting_position: Coord, destination: &Coord) -> usize {
    let mut stack: VecDeque<State> = VecDeque::new();
    let mut path_lengths: Vec<usize> = vec![];
    stack.push_front(State::new(starting_position, grid, 0));
    let max_blizzard_moves = grid
        .iter()
        .flat_map(|(coord, _)| vec![coord.row, coord.col])
        .max()
        .unwrap()
        - 2;
    let mut wait_at_start = 0;
    let mut states_explored: Vec<State> = vec![];

    while !stack.is_empty() {
        let state = stack.pop_front().unwrap();
        let current_position = state.current_position;
        let round = state.round;
        if current_position == *destination {
            path_lengths.push(round);
            continue;
        }
        let grid = state.grid;

        //Move all blizzards
        let new_grid: BTreeMap<Coord, Tile> = make_blizzards_move(grid);

        // display_grid(&new_grid);
        //No Move
        //as no blizzard ever moves to the starting postion, this will never move
        if current_position == starting_position {
            //waiting at max for all the blizzards to move back to the same positions as at the
            //start
            if wait_at_start <= max_blizzard_moves {
                let new_state = State::new(current_position, &new_grid, round + 1);
                if !states_contains(&states_explored, &new_state) {
                    stack.push_front(new_state.clone());
                    states_explored.push(new_state);
                    wait_at_start += 1;
                }
            } else {
                wait_at_start = 0;
            }
        } else {
            let new_state = State::new(current_position, &new_grid, round + 1);
            if new_state.is_valid() && !states_contains(&states_explored, &new_state) {
                stack.push_front(new_state.clone());
                states_explored.push(new_state);
            }
        }
        //Move North
        if current_position.row != 0 {
            let new_position = Coord::new(current_position.row - 1, current_position.col);
            let new_state = State::new(new_position, &new_grid, round + 1);
            if new_state.is_valid() && !states_contains(&states_explored, &new_state) {
                stack.push_front(new_state.clone());
                states_explored.push(new_state);
            }
        }
        //Move South
        let new_position = Coord::new(current_position.row + 1, current_position.col);
        let new_state = State::new(new_position, &new_grid, round + 1);
        if new_state.is_valid() && !states_contains(&states_explored, &new_state) {
            stack.push_front(new_state.clone());
            states_explored.push(new_state);
        }
        //Move West
        if current_position.col != 0 {
            let new_position = Coord::new(current_position.row, current_position.col - 1);
            let new_state = State::new(new_position, &new_grid, round + 1);
            if new_state.is_valid() && !states_contains(&states_explored, &new_state) {
                stack.push_front(new_state.clone());
                states_explored.push(new_state);
            }
        }
        //Move East
        let new_position = Coord::new(current_position.row, current_position.col + 1);
        let new_state = State::new(new_position, &new_grid, round + 1);
        if new_state.is_valid() && !states_contains(&states_explored, &new_state) {
            stack.push_front(new_state.clone());
            states_explored.push(new_state);
        }
    }
    if path_lengths.len() == 0 {
        panic!();
    }

    let shortest_length = path_lengths.iter().min();
    if shortest_length.is_none() {
        panic!();
    }
    *shortest_length.unwrap()
}

fn find_path_bfs(
    grid: &BTreeMap<Coord, Tile>,
    starting_position: Coord,
    destination: &Coord,
) -> usize {
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_front(State::new(starting_position, grid, 0));
    let max_blizzard_moves = grid
        .iter()
        .flat_map(|(coord, _)| vec![coord.row, coord.col])
        .max()
        .unwrap()
        - 2;
    let mut wait_at_start = 0;
    let mut states_explored: Vec<State> = vec![];

    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();
        let current_position = state.current_position;
        let round = state.round;
        if current_position == *destination {
            return round;
        }
        let grid = state.grid;

        //Move all blizzards
        let new_grid: BTreeMap<Coord, Tile> = make_blizzards_move(grid);

        // display_grid(&new_grid);
        //No Move
        //as no blizzard ever moves to the starting postion, this will never move
        if current_position == starting_position {
            //waiting at max for all the blizzards to move back to the same positions as at the
            //start
            if wait_at_start <= max_blizzard_moves {
                let new_state = State::new(current_position, &new_grid, round + 1);
                if !states_contains(&states_explored, &new_state) {
                    queue.push_back(new_state.clone());
                    states_explored.push(new_state);
                    wait_at_start += 1;
                }
            } else {
                wait_at_start = 0;
            }
        } else {
            let new_state = State::new(current_position, &new_grid, round + 1);
            if new_state.is_valid() && !states_contains(&states_explored, &new_state) {
                queue.push_back(new_state.clone());
                states_explored.push(new_state);
            }
        }
        //Move North
        if current_position.row != 0 {
            let new_position = Coord::new(current_position.row - 1, current_position.col);
            let new_state = State::new(new_position, &new_grid, round + 1);
            if new_state.is_valid() && !states_contains(&states_explored, &new_state) {
                queue.push_back(new_state.clone());
                states_explored.push(new_state);
            }
        }
        //Move South
        let new_position = Coord::new(current_position.row + 1, current_position.col);
        let new_state = State::new(new_position, &new_grid, round + 1);
        if new_state.is_valid() && !states_contains(&states_explored, &new_state) {
            queue.push_back(new_state.clone());
            states_explored.push(new_state);
        }
        //Move West
        if current_position.col != 0 {
            let new_position = Coord::new(current_position.row, current_position.col - 1);
            let new_state = State::new(new_position, &new_grid, round + 1);
            if new_state.is_valid() && !states_contains(&states_explored, &new_state) {
                queue.push_back(new_state.clone());
                states_explored.push(new_state);
            }
        }
        //Move East
        let new_position = Coord::new(current_position.row, current_position.col + 1);
        let new_state = State::new(new_position, &new_grid, round + 1);
        if new_state.is_valid() && !states_contains(&states_explored, &new_state) {
            queue.push_back(new_state.clone());
            states_explored.push(new_state);
        }
    }
    panic!()
}

fn make_blizzards_move(grid: BTreeMap<Coord, Tile>) -> BTreeMap<Coord, Tile> {
    let mut grid = grid;
    let mut proposed_moves: Vec<(Coord, Coord, Direction)> = vec![];
    for (coord, tile) in grid.clone() {
        match tile {
            Tile::Blizzard(directions) => {
                directions.iter().for_each(|direction| {
                    let next_coord = get_next_coord(coord, *direction, &grid);
                    proposed_moves.push((coord, next_coord, *direction));
                });
            }
            _ => continue,
        }
    }
    for (from, to, direction) in proposed_moves {
        let tile_from = grid.remove(&from).unwrap();
        let tile_to = grid.remove(&to).unwrap();
        let mut directions_from = match tile_from {
            Tile::Blizzard(directions) => directions,
            _ => panic!(),
        };
        if let Some(index) = directions_from.iter().position(|&t| t == direction) {
            directions_from.remove(index);
        } else {
            panic!();
        }
        let tile_to_insert = if directions_from.is_empty() {
            Tile::Ground
        } else {
            Tile::Blizzard(directions_from)
        };
        grid.insert(from, tile_to_insert);

        match tile_to {
            Tile::Wall => panic!(),
            Tile::Ground => {
                grid.insert(to, Tile::Blizzard(vec![direction]));
            }
            Tile::Blizzard(directions) => {
                let mut directions = directions;
                directions.push(direction);
                grid.insert(to, Tile::Blizzard(directions));
            }
        }
    }

    grid
}

fn get_next_coord(current: Coord, direction: Direction, grid: &BTreeMap<Coord, Tile>) -> Coord {
    let max_non_wall_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap() - 1;
    let min_non_wall_row = 1;

    let max_non_wall_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap() - 1;
    let min_non_wall_col = 1;

    let row = current.row;
    let col = current.col;

    let (new_row, new_col) = {
        match direction {
            Direction::North => {
                if row == min_non_wall_row {
                    (max_non_wall_row, col)
                } else {
                    (row - 1, col)
                }
            }
            Direction::South => {
                if row == max_non_wall_row {
                    (min_non_wall_row, col)
                } else {
                    (row + 1, col)
                }
            }
            Direction::East => {
                if col == max_non_wall_col {
                    (row, min_non_wall_col)
                } else {
                    (row, col + 1)
                }
            }
            Direction::West => {
                if col == min_non_wall_col {
                    (row, max_non_wall_col)
                } else {
                    (row, col - 1)
                }
            }
        }
    };
    Coord::new(new_row, new_col)
}

fn process_part_2(input: &str) -> usize {
    let grid = parse_input(input);
    0
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Ground,
    Blizzard(Vec<Direction>),
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
                    Tile::Blizzard(directions) => {
                        if directions.len() > 1 {
                            print!("{}", directions.len());
                            continue;
                        }
                        directions.iter().for_each(|direction| match direction {
                            Direction::North => print!("^"),
                            Direction::South => print!("v"),
                            Direction::East => print!(">"),
                            Direction::West => print!("<"),
                        })
                    }
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
                '^' => Tile::Blizzard(vec![Direction::North]),
                'v' => Tile::Blizzard(vec![Direction::South]),
                '>' => Tile::Blizzard(vec![Direction::East]),
                '<' => Tile::Blizzard(vec![Direction::West]),
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
