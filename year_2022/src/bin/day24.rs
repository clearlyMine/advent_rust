use std::collections::{BTreeMap, VecDeque};
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day24.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input, res1);
    println!("Part 2: {}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let grid: BTreeMap<Coord, Tile> = parse_input(input);
    let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();

    const STARTING_POSITION: Coord = Coord { row: 0, col: 1 };
    let destination = grid
        .iter()
        .filter(|(coord, tile)| coord.row == max_row && **tile == Tile::Ground)
        .map(|(coord, _)| coord)
        .last()
        .unwrap();
    find_path_bfs(&grid, &STARTING_POSITION, destination, 0)
}

fn process_part_2(input: &str, time_to_destination: usize) -> usize {
    let grid: BTreeMap<Coord, Tile> = parse_input(input);
    let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();

    const STARTING_POSITION: Coord = Coord { row: 0, col: 1 };
    let destination = grid
        .iter()
        .filter(|(coord, tile)| coord.row == max_row && **tile == Tile::Ground)
        .map(|(coord, _)| coord)
        .last()
        .unwrap();
    let back_to_start = find_path_bfs(&grid, destination, &STARTING_POSITION, time_to_destination);
    find_path_bfs(&grid, &STARTING_POSITION, destination, back_to_start)
}

fn find_path_bfs(
    grid: &BTreeMap<Coord, Tile>,
    starting_position: &Coord,
    destination: &Coord,
    starting_round: usize,
) -> usize {
    let mut queue: VecDeque<State> = VecDeque::new();
    let initial_state = State::new(*starting_position, starting_round, 0);
    queue.push_front(initial_state);

    let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
    let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();

    let max_states = lcm(max_row - 1, max_col - 1);
    let mut states_processed: Vec<State> = vec![initial_state];

    while let Some(state) = queue.pop_front() {
        // dbg!(queue.len());
        let current_position = state.current_position;
        let round = state.round;
        let (row, col) = (current_position.row, current_position.col);
        for (nr, nc) in get_next_coords(row, col) {
            if (nr, nc) == (destination.row, destination.col) {
                return round + 1;
            }
            if (nr, nc) == (starting_position.row, starting_position.col) && state.wait < max_states
            {
                let new_state = State::new(
                    Coord::new(nr as usize, nc as usize),
                    round as usize + 1,
                    state.wait + 1,
                );
                if !is_state_processed(&states_processed, &new_state, max_states) {
                    queue.push_back(new_state);
                    states_processed.push(new_state);
                }
                continue;
            }
            if nr < max_row && nc < max_col {
                if no_blizzards(nr, nc, round, max_row, max_col, grid) {
                    let wait = if (nr, nc) == (row, col) {
                        state.wait + 1
                    } else {
                        0
                    };
                    if wait >= max_states {
                        continue;
                    }
                    let new_state = State::new(
                        Coord::new(nr as usize, nc as usize),
                        round as usize + 1,
                        wait,
                    );
                    if !is_state_processed(&states_processed, &new_state, max_states) {
                        queue.push_back(new_state);
                        states_processed.push(new_state);
                    }
                }
            }
        }
    }
    0
}

#[derive(Clone, Debug, Copy)]
struct State {
    current_position: Coord,
    round: usize,
    wait: usize,
}

impl State {
    fn new(current_position: Coord, round: usize, wait: usize) -> State {
        State {
            current_position,
            round,
            wait,
        }
    }

    fn is_same(&self, other: &State, max_rounds: usize) -> bool {
        self.current_position == other.current_position
            && self.round % max_rounds == other.round % max_rounds
    }
}

fn is_state_processed(states: &Vec<State>, to_compare: &State, max_rounds: usize) -> bool {
    states.iter().any(|cur| cur.is_same(to_compare, max_rounds))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn get_next_coords(row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = vec![(row + 1, col), (row, col + 1)];
    if col > 0 {
        out.push((row, col - 1));
    }
    if row > 0 {
        out.push((row - 1, col));
    }
    out.push((row, col));
    out
}

fn no_blizzards(
    row: usize,
    col: usize,
    round: usize,
    max_row: usize,
    max_col: usize,
    grid: &BTreeMap<Coord, Tile>,
) -> bool {
    let c = Coord::new(row, col);
    if !grid.contains_key(&c) {
        return false;
    }
    if grid.get(&c).unwrap() == &Tile::Wall {
        return false;
    }
    let nr = row as isize;
    let nc = col as isize;
    let round = round as isize + 1;
    let max_row = max_row - 1;
    let max_col = max_col - 1;

    let east = &Coord::new(nr as usize, get_wrapped_count(nc - round, max_col));
    let west = &Coord::new(nr as usize, get_wrapped_count(nc + round, max_col));
    let south = &Coord::new(get_wrapped_count(nr - round, max_row), nc as usize);
    let north = &Coord::new(get_wrapped_count(nr + round, max_row), nc as usize);
    grid.get(east).unwrap() != &Tile::Blizzard(Direction::East)
        && grid.get(west).unwrap() != &Tile::Blizzard(Direction::West)
        && grid.get(south).unwrap() != &Tile::Blizzard(Direction::South)
        && grid.get(north).unwrap() != &Tile::Blizzard(Direction::North)
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
}

fn get_wrapped_count(num: isize, max: usize) -> usize {
    if num < 1 {
        let max = max as isize;
        let range = max;
        let num = num + range * ((1 - num) / range + 1);
        return (1 + (num - 1) % range) as usize;
    }
    let range = max;
    return (1 as isize + ((num - 1) % range as isize)) as usize;
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
        let input = include_str!("../../inputs/day24_sample.txt");
        assert_eq!(process_part_1(input), 18);
    }

    #[test]
    fn test_process_part_2() {
        let input = include_str!("../../inputs/day24_sample.txt");
        assert_eq!(process_part_2(input, 18), 54);
    }
}
