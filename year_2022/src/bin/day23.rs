use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day23.txt");
    let res1 = process_part_1(input, 10);
    println!("Part 1: {}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str, rounds: usize) -> usize {
    let grid = parse_input(input);
    // dbg!(grid.clone());
    // display_grid(grid.clone());
    let mut grid = pad_grid(grid, rounds);
    // println!();
    // display_grid(grid.clone());

    let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
    let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();
    let mut directions = vec![
        Directions::North,
        Directions::South,
        Directions::West,
        Directions::East,
    ];
    for _ in 1..=rounds {
        let mut round_moves: Vec<(Coord, Coord)> = vec![];
        for row in 0..=max_row {
            'col: for col in 0..=max_col {
                let current_pos = Coord { row, col };
                let element = grid.get(&current_pos).unwrap();
                if element == &Tile::Empty {
                    continue;
                }
                // println!("Elf in {:?}", current_pos);
                //check around the Elf
                if is_elf_alone(&grid, Coord { row, col }) {
                    // println!("elf is alone");
                    continue;
                }

                //check every direction one-by-one
                for direction in &directions {
                    if let Some(next) = current_pos.can_move(*direction, &grid) {
                        // println!("elf can move {:?}", direction);
                        round_moves.push((current_pos, next));
                        continue 'col;
                    }
                }

                // println!("elf cannot move");
            }
        }
        // dbg!(round_moves.clone());
        directions = shuffle_directions(directions);
        //move the unique ones
        let mut next_with_counts: HashMap<Coord, usize> = HashMap::new();
        for (_, next) in round_moves.clone() {
            let n = next_with_counts.get(&next).unwrap_or(&0);
            next_with_counts.insert(next, n + 1);
        }
        let filter_next_moves: Vec<&Coord> = next_with_counts
            .iter()
            .filter(|(_, count)| **count == 1)
            .map(|(coord, _)| coord)
            .collect();
        for (cur, next) in round_moves {
            if filter_next_moves.contains(&&next) {
                grid.insert(cur, Tile::Empty);
                grid.insert(next, Tile::Elf);
            }
        }
    }
    // println!();
    // display_grid(grid.clone());

    let elf_tiles = grid.iter().filter(|(_, tile)| tile == &&Tile::Elf);
    let rows = elf_tiles.clone().map(|(coord, _)| coord.row);
    let min_row = rows.clone().min().unwrap();
    let max_row = rows.max().unwrap();
    let cols = elf_tiles.map(|(coord, _)| coord.col);
    let min_col = cols.clone().min().unwrap();
    let max_col = cols.max().unwrap();
    let mut count = 0;
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if grid.get(&Coord { row, col }).unwrap() == &Tile::Empty {
                count += 1;
            }
        }
    }
    count
}

fn process_part_2(input: &str) -> usize {
    let grid = parse_input(input);
    // dbg!(grid.clone());
    // display_grid(grid.clone());
    let mut grid = pad_grid(grid, 100);
    // println!();
    // display_grid(grid.clone());

    let mut directions = vec![
        Directions::North,
        Directions::South,
        Directions::West,
        Directions::East,
    ];
    let mut rounds = 1;
    loop {
        if rounds % 100 == 0 {
            grid = pad_grid(grid, 100);
            // println!("{}", rounds);
        }
        let elf_tiles = grid.iter().filter(|(_, tile)| tile == &&Tile::Elf);
        let rows = elf_tiles.clone().map(|(coord, _)| coord.row);
        let min_row = rows.clone().min().unwrap();
        let max_row = rows.max().unwrap();
        let cols = elf_tiles.map(|(coord, _)| coord.col);
        let min_col = cols.clone().min().unwrap();
        let max_col = cols.max().unwrap();
        let mut round_moves: Vec<(Coord, Coord)> = vec![];
        for row in min_row..=max_row {
            'col: for col in min_col..=max_col {
                let current_pos = Coord { row, col };
                let element = grid.get(&current_pos).unwrap();
                if element == &Tile::Empty {
                    continue;
                }
                // println!("Elf in {:?}", current_pos);
                //check around the Elf
                if is_elf_alone(&grid, Coord { row, col }) {
                    // println!("elf is alone");
                    continue;
                }

                //check every direction one-by-one
                for direction in &directions {
                    if let Some(next) = current_pos.can_move(*direction, &grid) {
                        // println!("elf can move {:?}", direction);
                        round_moves.push((current_pos, next));
                        continue 'col;
                    }
                }

                // println!("elf cannot move");
            }
        }
        // dbg!(round_moves.clone());
        directions = shuffle_directions(directions);
        //move the unique ones
        let mut next_with_counts: HashMap<Coord, usize> = HashMap::new();
        for (_, next) in round_moves.clone() {
            let n = next_with_counts.get(&next).unwrap_or(&0);
            next_with_counts.insert(next, n + 1);
        }
        let filter_next_moves: Vec<&Coord> = next_with_counts
            .iter()
            .filter(|(_, count)| **count == 1)
            .map(|(coord, _)| coord)
            .collect();
        for (cur, next) in round_moves {
            if filter_next_moves.contains(&&next) {
                grid.insert(cur, Tile::Empty);
                grid.insert(next, Tile::Elf);
            }
        }
        if filter_next_moves.is_empty() {
            break;
        }
        rounds += 1;
    }
    // println!();
    // display_grid(grid.clone());

    rounds
}

fn shuffle_directions(directions: Vec<Directions>) -> Vec<Directions> {
    vec![directions[1], directions[2], directions[3], directions[0]]
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Elf,
    Empty,
}

// impl Tile {
//     fn toggle(&self) -> Tile {
//         match self {
//             Tile::Elf => Tile::Empty,
//             Tile::Empty => Tile::Elf,
//         }
//     }
// }

fn is_elf_alone(grid: &HashMap<Coord, Tile>, current_position: Coord) -> bool {
    let row = current_position.row;
    let col = current_position.col;
    let ne = Coord::new(row - 1, col + 1);
    let se = Coord::new(row + 1, col + 1);
    let nw = Coord::new(row - 1, col - 1);
    let sw = Coord::new(row + 1, col - 1);
    let north = Coord::new(row - 1, col);
    let south = Coord::new(row + 1, col);
    let west = Coord::new(row, col - 1);
    let east = Coord::new(row, col + 1);

    grid.get(&ne).unwrap() == &Tile::Empty
        && grid.get(&north).unwrap() == &Tile::Empty
        && grid.get(&nw).unwrap() == &Tile::Empty
        && grid.get(&se).unwrap() == &Tile::Empty
        && grid.get(&south).unwrap() == &Tile::Empty
        && grid.get(&sw).unwrap() == &Tile::Empty
        && grid.get(&east).unwrap() == &Tile::Empty
        && grid.get(&west).unwrap() == &Tile::Empty
}

// fn display_grid(grid: HashMap<Coord, Tile>) {
//     let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
//     let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();
//     for row in 0..=max_row {
//         for col in 0..=max_col {
//             let coord = Coord { row, col };
//             if let Some(element) = grid.get(&coord) {
//                 match element {
//                     Tile::Empty => print!("."),
//                     Tile::Elf => print!("#"),
//                 }
//             } else {
//                 break;
//             }
//         }
//         println!();
//     }
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Directions {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

trait MoveCheck {
    fn can_move(&self, direction: Directions, grid: &HashMap<Coord, Tile>) -> Option<Coord>;
}

impl Coord {
    fn new(row: usize, col: usize) -> Coord {
        Coord { row, col }
    }
}

impl MoveCheck for Coord {
    fn can_move(&self, direction: Directions, grid: &HashMap<Coord, Tile>) -> Option<Coord> {
        let row = self.row;
        let col = self.col;
        let ne = &Coord::new(row - 1, col + 1);
        let north = &Coord::new(row - 1, col);
        let nw = &Coord::new(row - 1, col - 1);
        let se = &Coord::new(row + 1, col + 1);
        let south = &Coord::new(row + 1, col);
        let sw = &Coord::new(row + 1, col - 1);
        let east = &Coord::new(row, col + 1);
        let west = &Coord::new(row, col - 1);

        match direction {
            Directions::North => {
                if grid.get(ne).unwrap_or(&Tile::Empty) == &Tile::Empty
                    && grid.get(north).unwrap_or(&Tile::Empty) == &Tile::Empty
                    && grid.get(nw).unwrap_or(&Tile::Empty) == &Tile::Empty
                {
                    return Some(*north);
                }
            }

            Directions::South => {
                if grid.get(se).unwrap_or(&Tile::Empty) == &Tile::Empty
                    && grid.get(south).unwrap_or(&Tile::Empty) == &Tile::Empty
                    && grid.get(sw).unwrap_or(&Tile::Empty) == &Tile::Empty
                {
                    return Some(*south);
                }
            }

            Directions::West => {
                if grid.get(nw).unwrap_or(&Tile::Empty) == &Tile::Empty
                    && grid.get(west).unwrap_or(&Tile::Empty) == &Tile::Empty
                    && grid.get(sw).unwrap_or(&Tile::Empty) == &Tile::Empty
                {
                    return Some(*west);
                }
            }
            Directions::East => {
                if grid.get(ne).unwrap_or(&Tile::Empty) == &Tile::Empty
                    && grid.get(east).unwrap_or(&Tile::Empty) == &Tile::Empty
                    && grid.get(se).unwrap_or(&Tile::Empty) == &Tile::Empty
                {
                    return Some(*east);
                }
            }
        }
        None
    }
}

fn pad_grid(grid: HashMap<Coord, Tile>, padding: usize) -> HashMap<Coord, Tile> {
    let mut grid = grid;
    let max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
    let max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();
    for row in (0..=max_row).rev() {
        for col in (0..=max_col).rev() {
            let coord = Coord { row, col };
            let org = grid.get(&coord).unwrap();
            let coord = Coord {
                row: row + padding,
                col: col + padding,
            };
            grid.insert(coord, *org);
        }
    }
    let max_row = max_row + padding * 2;
    let max_col = max_col + padding * 2;

    for row in 0..padding {
        for col in 0..=max_col {
            let coord = Coord { row, col };
            grid.insert(coord, Tile::Empty);
        }
    }

    for row in (max_row - padding + 1)..=max_row {
        for col in 0..=max_col {
            let coord = Coord { row, col };
            grid.insert(coord, Tile::Empty);
        }
    }

    for col in 0..padding {
        for row in 0..=max_row {
            let coord = Coord { row, col };
            grid.insert(coord, Tile::Empty);
        }
    }

    for col in (max_col - padding + 1)..=max_col {
        for row in 0..=max_row {
            let coord = Coord { row, col };
            grid.insert(coord, Tile::Empty);
        }
    }

    grid
}

fn parse_input(input: &str) -> HashMap<Coord, Tile> {
    let mut grid: HashMap<Coord, Tile> = HashMap::new();
    let mut row = 0;
    for line in input.lines() {
        let chars = line.chars();
        let mut col = 0;
        for char in chars {
            let tile = if char == '.' { Tile::Empty } else { Tile::Elf };
            let coords = Coord { row, col };
            grid.insert(coords, tile);
            col += 1;
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
        let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        assert_eq!(process_part_1(input, 10), 110);
    }

    #[test]
    fn test_process_part_2() {
        let input = include_str!("../../inputs/day23_sample.txt");
        assert_eq!(process_part_2(input), 20);
    }
    //
    // #[test]
    // fn test_get_min_non_empty_col(){
    //     let input_grid: HashMap<Coord,MoveType> = HashMap::new();
    //     for row in 1..10{
    //         for col in 1..20{
    //             match col{
    //                 1=>{let coord = Coord{row,col};
    //                     let move_type =
}
