use std::collections::{BTreeMap, VecDeque};

fn main() {
    let input = include_str!("../inputs/day24.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {}", res1);
    let res2 = process_part_2(input, res1);
    println!("Part 2: {}", res2);
}

fn process_part_1(input: &str) -> usize {
    let grid: BTreeMap<Coord, Tile> = parse_input(input);
    // display_grid(&grid);
    let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
    // let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();

    const STARTING_POSITION: Coord = Coord { row: 0, col: 1 };
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
    find_path_bfs(&grid, &STARTING_POSITION, *destination, 0)
}

fn process_part_2(input: &str, time_to_destination: usize) -> usize {
    let grid: BTreeMap<Coord, Tile> = parse_input(input);
    // display_grid(&grid);
    let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
    // let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();

    const STARTING_POSITION: Coord = Coord { row: 0, col: 1 };
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
    let back_to_start = find_path_bfs(&grid, *destination, &STARTING_POSITION, time_to_destination);
    find_path_bfs(&grid, &STARTING_POSITION, *destination, back_to_start)
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
            && self.wait == other.wait
    }
}

fn is_state_been_processed(states: &Vec<State>, to_compare: &State, max_rounds: usize) -> bool {
    states
        .iter()
        .filter(|cur| cur.is_same(to_compare, max_rounds))
        .collect::<Vec<_>>()
        .len()
        > 0
}

fn find_path_bfs(
    grid: &BTreeMap<Coord, Tile>,
    starting_position: &Coord,
    destination: &Coord,
    starting_round: usize,
) -> usize {
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_front(State::new(*starting_position, starting_round, 0));

    let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
    let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();
    //find lcm
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

    let max_states = lcm(max_row - 1, max_col - 1);
    let mut states_processed: Vec<State> = vec![];

    while !queue.is_empty() {
        // dbg!(queue.len());
        let state = queue.pop_front().unwrap();
        let current_position = state.current_position;
        let round = state.round;
        // dbg!(round);
        let wait = state.wait;
        if current_position == *destination {
            return round;
        }
        let round = round + 1;
        current_position
            .get_valid_neighbours(grid, round)
            .iter()
            .for_each(|c| {
                let new_state = State::new(*c, round, 0);
                if !is_state_been_processed(&states_processed, &new_state, max_states) {
                    queue.push_back(new_state);
                    states_processed.push(new_state);
                }
            });
        if wait < max_states && current_position.is_valid(grid, round) {
            queue.push_back(State::new(current_position, round, wait + 1));
        }
    }
    panic!()
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

    fn neighbours(&self) -> Vec<Coord> {
        let row = self.row;
        let col = self.col;
        let mut nebs = vec![];

        //south
        nebs.push(Coord { row: row + 1, col });
        //east
        nebs.push(Coord { row, col: col + 1 });
        //west
        if col > 0 {
            nebs.push(Coord { row, col: col - 1 });
        }
        //north
        if row > 0 {
            nebs.push(Coord { row: row - 1, col });
        }
        nebs
    }

    fn get_valid_neighbours(&self, grid: &BTreeMap<Coord, Tile>, round: usize) -> Vec<Coord> {
        let n = self.neighbours();
        n.into_iter().filter(|x| x.is_valid(grid, round)).collect()
    }

    fn is_valid(&self, grid: &BTreeMap<Coord, Tile>, round: usize) -> bool {
        let current_position = self;
        if !grid.contains_key(&current_position) {
            return false;
        }
        let tile = grid.get(&current_position).unwrap();
        if *tile == Tile::Wall {
            return false;
        }
        let round = round as isize;

        let row = current_position.row as isize;
        let col = current_position.col as isize;
        //look around the grid current_position in every direction
        let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap() - 1;
        let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap() - 1;
        let max_row = max_row;
        let max_col = max_col;
        let round = round as isize;

        //from Up 'v'
        let north_row = get_wrapped_count(max_row, row - round);
        let north: Coord = Coord::new(north_row, col as usize);
        if *grid.get(&north).unwrap() == Tile::Blizzard(Direction::South) {
            return false;
        }

        //from Down '^'
        let south_row = get_wrapped_count(max_row, row + round);
        let south: Coord = Coord::new(south_row as usize, col as usize);
        if *grid.get(&south).unwrap() == Tile::Blizzard(Direction::North) {
            return false;
        }

        //from West '>'
        let west_col = get_wrapped_count(max_col, col - round);
        let west: Coord = Coord::new(row as usize, west_col as usize);
        if *grid.get(&west).unwrap() == Tile::Blizzard(Direction::East) {
            return false;
        }

        //from East '<'
        let east_col = get_wrapped_count(max_col, col + round);
        let east: Coord = Coord::new(row as usize, east_col as usize);
        if *grid.get(&east).unwrap() == Tile::Blizzard(Direction::West) {
            return false;
        }

        true
    }
}

fn get_wrapped_count(max: usize, num: isize) -> usize {
    if num < 1 as isize {
        let max = max as isize;
        let range = max;
        let num = num + range * ((1 - num) / range + 1);
        return (1 + (num - 1) % range) as usize;
    }
    let range = max;
    return (1 as isize + ((num - 1) % range as isize)) as usize;
}

// fn display_grid(grid: &BTreeMap<Coord, Tile>) {
//     let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
//     let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();
//     for row in 0..=max_row {
//         for col in 0..=max_col {
//             let coord = Coord { row, col };
//             if let Some(tile) = grid.get(&coord) {
//                 match tile {
//                     Tile::Ground => print!("."),
//                     Tile::Wall => print!("#"),
//                     Tile::Blizzard(direction) => match direction {
//                         Direction::North => print!("^"),
//                         Direction::South => print!("v"),
//                         Direction::East => print!(">"),
//                         Direction::West => print!("<"),
//                     },
//                 }
//             } else {
//                 break;
//             }
//         }
//         println!();
//     }
// }

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
        let input = include_str!("../inputs/day24_sample.txt");
        assert_eq!(process_part_1(input), 18);
    }

    #[test]
    fn test_process_part_2() {
        let input = include_str!("../inputs/day24_sample.txt");
        assert_eq!(process_part_2(input, 18), 54);
    }
}
