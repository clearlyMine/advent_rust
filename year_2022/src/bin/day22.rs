use nom::{
    character::{self, complete::alpha1},
    combinator, IResult,
};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day22.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let (grid, operations) = parse_input(input);
    // dbg!(grid.clone());
    // dbg!(operations.clone());

    let coord = get_first_non_empty_column(&grid, 1);
    // dbg!(coord);
    let mut current_position: (Coord, Directions) = (coord, Directions::Right);
    // dbg!(current_position);
    for oper in operations {
        // dbg!(oper);
        match oper {
            OperationTypes::Rotation(rot) => {
                current_position.1 = current_position.1.rotate(rot);
                // dbg!(current_position);
            }
            OperationTypes::Num(num) => {
                for _ in 0..num {
                    let next_coord =
                        get_next_coordinate(current_position.0, current_position.1, &grid);
                    // dbg!(next_coord);
                    if let Some(val) = grid.get(&next_coord) {
                        match val {
                            MoveType::Empty => panic!(),
                            MoveType::Wall => break,
                            MoveType::Possible => current_position.0 = next_coord,
                        }
                    } else {
                        panic!();
                    }
                    // dbg!(current_position);
                }
            }
        }
    }
    let facing_marks = match current_position.1 {
        Directions::Right => 0,
        Directions::Down => 1,
        Directions::Left => 2,
        Directions::Up => 3,
    };
    current_position.0.row * 1000 + current_position.0.col * 4 + facing_marks
}

fn process_part_2(input: &str) -> usize {
    let (grid, operations) = parse_input(input);
    // dbg!(grid.clone());
    // dbg!(operations.clone());

    let coord = get_first_non_empty_column(&grid, 1);
    // dbg!(coord);
    let mut current_position: (Coord, Directions) = (coord, Directions::Right);
    // dbg!(current_position);
    let size = get_size_of_cube(&grid);
    for oper in operations {
        // dbg!(oper);
        // println!("Operation {:?}", oper);
        match oper {
            OperationTypes::Rotation(rot) => {
                current_position.1 = current_position.1.rotate(rot);
                // dbg!(current_position);
                // println!("current = {:?}", current_position);
            }
            OperationTypes::Num(num) => {
                // println!("moving {} steps towards {:?}", num, current_position.1);
                for _ in 0..num {
                    let (next_coord, new_direction) =
                        get_next_coordinate_2(current_position.0, current_position.1, &grid, size);
                    // dbg!(next_coord);
                    // println!("next_coord = {:?}", next_coord);
                    // println!("next direction = {:?}", new_direction);
                    if let Some(val) = grid.get(&next_coord) {
                        match val {
                            MoveType::Empty => panic!(),
                            MoveType::Wall => break,
                            MoveType::Possible => {
                                current_position.0 = next_coord;
                                current_position.1 = new_direction;
                            }
                        }
                    } else {
                        panic!();
                    }
                    // dbg!(current_position);
                    // println!("current = {:?}", current_position);
                }
            }
        }
    }
    let facing_marks = match current_position.1 {
        Directions::Right => 0,
        Directions::Down => 1,
        Directions::Left => 2,
        Directions::Up => 3,
    };
    current_position.0.row * 1000 + current_position.0.col * 4 + facing_marks
}

fn get_size_of_cube(grid: &HashMap<Coord, MoveType>) -> usize {
    let mut max_row = grid.iter().map(|(coord, _)| coord.row).max().unwrap();
    let mut max_col = grid.iter().map(|(coord, _)| coord.col).max().unwrap();
    while max_col != 0 {
        let temp = max_col;
        max_col = max_row % max_col;
        max_row = temp;
    }
    max_row
}

fn get_next_coordinate_2(
    coord: Coord,
    direction: Directions,
    grid: &HashMap<Coord, MoveType>,
    size: usize,
) -> (Coord, Directions) {
    let row = coord.row;
    let col = coord.col;
    let mut coord = coord;
    let mut direction = direction;

    match direction {
        Directions::Up => {
            if row == get_min_non_empty_row(grid, col) {
                //at faces 4,1 or 2
                //for input
                if (1..size).contains(&col) {
                    //at face 4
                    //moves to 3 left
                    coord.row = size + col;
                    coord.col = size + 1;
                    direction = Directions::Right;
                } else if ((size + 1)..=size * 2).contains(&col) {
                    //at face 1
                    //moves to 5 left
                    coord.row = size * 2 + col;
                    coord.col = 1;
                    direction = Directions::Right;
                } else if ((size * 2 + 1)..size * 3).contains(&col) {
                    // at face 2
                    // moves to 5 bottom
                    coord.row = size * 4;
                    coord.col = col - size * 2;
                    direction = Directions::Up;
                }
            } else {
                coord.row = row - 1;
            }
        }
        Directions::Down => {
            if row == get_max_non_empty_row(grid, col) {
                //at faces 5, 6 or 2
                if (1..size).contains(&col) {
                    //at face 5
                    //moves to 2 top
                    coord.row = 1;
                    coord.col = size * 2 + col;
                    direction = Directions::Down;
                } else if ((size + 1)..size * 2).contains(&col) {
                    //at face 6
                    //moves to 5 right reversed
                    coord.row = size * 3 + (col - size);
                    coord.col = size;
                    direction = Directions::Left;
                } else if ((size * 2 + 1)..size * 3).contains(&col) {
                    //at face 2
                    //moves to 3 right
                    coord.row = col - size;
                    coord.col = size * 2;
                    direction = Directions::Left;
                }
            } else {
                coord.row = row + 1;
            }
        }
        Directions::Left => {
            if col == get_min_non_empty_col(grid, row) {
                //at faces 1, 3, 4, or 5
                if (1..size).contains(&row) {
                    //at face 1
                    //moves to 4 left reversed
                    coord.col = 1;
                    coord.row = size * 3 - row + 1;
                    direction = Directions::Right;
                } else if ((size + 1)..size * 2).contains(&row) {
                    //at face 3
                    //moves to 4 top
                    coord.row = size * 2 + 1;
                    coord.col = row - size;
                    direction = Directions::Down;
                } else if ((size * 2 + 1)..size * 3).contains(&row) {
                    //at face 4
                    //moves to 1 left reversed
                    coord.row = size - (row - size * 2) + 1;
                    coord.col = size + 1;
                    direction = Directions::Right;
                } else if ((size * 3 + 1)..size * 4).contains(&row) {
                    //at face 5
                    //moves to 1 top
                    coord.row = 1;
                    coord.col = size + (row - size * 3);
                    direction = Directions::Down;
                }
            } else {
                coord.col = col - 1;
            }
        }
        Directions::Right => {
            if col == get_max_non_empty_col(grid, row) {
                //at faces 2, 3, 6, or 5
                if (1..size).contains(&row) {
                    //at face 2
                    //moves to 6 right reversed
                    coord.col = size * 2;
                    coord.row = size * 3 - (row) + 1;
                    direction = Directions::Left;
                } else if ((size + 1)..size * 2).contains(&row) {
                    //at face 3
                    //moves to 2 bottom
                    coord.row = size;
                    coord.col = size * 2 + (row - size);
                    direction = Directions::Up;
                } else if ((size * 2 + 1)..size * 3).contains(&row) {
                    //at face 6
                    //moves to 2 right reversed
                    coord.row = size * 3 - row + 1;
                    coord.col = size * 3;
                    direction = Directions::Left;
                } else if ((size * 3 + 1)..size * 4).contains(&row) {
                    //at face 5
                    //moves to 6 bottom
                    coord.row = size * 3;
                    coord.col = size + (row - size * 3);
                    direction = Directions::Up;
                }
            } else {
                coord.col = col + 1;
            }
        }
    }

    (coord, direction)
}

fn get_next_coordinate(
    coord: Coord,
    direction: Directions,
    grid: &HashMap<Coord, MoveType>,
) -> Coord {
    let row = coord.row;
    let col = coord.col;
    let mut coord = coord;
    match direction {
        Directions::Up => {
            if row == get_min_non_empty_row(grid, col) {
                coord.row = get_max_non_empty_row(grid, col);
            } else {
                coord.row = row - 1;
            }
        }
        Directions::Down => {
            if row == get_max_non_empty_row(grid, col) {
                coord.row = get_min_non_empty_row(grid, col);
            } else {
                coord.row = row + 1;
            }
        }
        Directions::Left => {
            if col == get_min_non_empty_col(grid, row) {
                coord.col = get_max_non_empty_col(grid, row);
            } else {
                coord.col = col - 1;
            }
        }
        Directions::Right => {
            if col == get_max_non_empty_col(grid, row) {
                coord.col = get_min_non_empty_col(grid, row);
            } else {
                coord.col = col + 1;
            }
        }
    }
    coord
}

fn get_max_non_empty_row(grid: &HashMap<Coord, MoveType>, col: usize) -> usize {
    grid.iter()
        .filter(|(coord, move_type)| coord.col == col && **move_type != MoveType::Empty)
        .map(|(coord, _)| coord.row)
        .max()
        .unwrap()
}

fn get_min_non_empty_row(grid: &HashMap<Coord, MoveType>, col: usize) -> usize {
    grid.iter()
        .filter(|(coord, move_type)| coord.col == col && **move_type != MoveType::Empty)
        .map(|(coord, _)| coord.row)
        .min()
        .unwrap()
}

fn get_max_non_empty_col(grid: &HashMap<Coord, MoveType>, row: usize) -> usize {
    grid.iter()
        .filter(|(coord, move_type)| coord.row == row && **move_type != MoveType::Empty)
        .map(|(coord, _)| coord.col)
        .max()
        .unwrap()
}

fn get_min_non_empty_col(grid: &HashMap<Coord, MoveType>, row: usize) -> usize {
    grid.iter()
        .filter(|(coord, move_type)| coord.row == row && **move_type != MoveType::Empty)
        .map(|(coord, _)| coord.col)
        .min()
        .unwrap()
}

fn get_first_non_empty_column(grid: &HashMap<Coord, MoveType>, row: usize) -> Coord {
    let min = grid
        .iter()
        .filter(|(coord, move_type)| coord.row == row && **move_type != MoveType::Empty)
        .map(|(coord, _)| coord.col)
        .min()
        .unwrap();
    Coord { row, col: min }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MoveType {
    Empty,
    Wall,
    Possible,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {
    fn rotate(&self, rotation: Rotation) -> Directions {
        match rotation {
            Rotation::Clockwise => match self {
                Directions::Up => Directions::Right,
                Directions::Down => Directions::Left,
                Directions::Left => Directions::Up,
                Directions::Right => Directions::Down,
            },
            Rotation::CounterClockWise => match self {
                Directions::Up => Directions::Left,
                Directions::Down => Directions::Right,
                Directions::Left => Directions::Down,
                Directions::Right => Directions::Up,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

fn parse_input(input: &str) -> (HashMap<Coord, MoveType>, Vec<OperationTypes>) {
    let mut grid: HashMap<Coord, MoveType> = HashMap::new();
    let mut split = input.split("\n\n");
    // dbg!(split.clone());
    let grid_input = split.next().unwrap();
    // dbg!(grid_input);
    let mut row = 1;
    for line in grid_input.lines() {
        let move_types = parse_grid_line(line);
        for i in 1..=move_types.len() {
            let coords = Coord { row, col: i };
            grid.insert(coords, move_types[i - 1]);
        }
        row += 1;
    }

    let mut operations = split.next().unwrap();
    // dbg!(&operations.chars().last());
    if operations.chars().last() == Some('\n') {
        operations = &operations[..operations.len() - 1];
    }
    let Ok((_, operations)) = parse_operations(operations) else {
        panic!()
    };
    (grid, operations)
}

fn parse_grid_line(input: &str) -> Vec<MoveType> {
    let chars = input.chars();
    let mut moves: Vec<MoveType> = vec![];
    for char in chars {
        match char {
            ' ' => moves.push(MoveType::Empty),
            '.' => moves.push(MoveType::Possible),
            '#' => moves.push(MoveType::Wall),
            _ => panic!(),
        }
    }
    moves
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Rotation {
    Clockwise,
    CounterClockWise,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum OperationTypes {
    Num(usize),
    Rotation(Rotation),
}

fn parse_operations(input: &str) -> IResult<&str, Vec<OperationTypes>> {
    let mut operations: Vec<OperationTypes> = vec![];
    let mut input = input;
    while !input.is_empty() {
        let moves: usize;
        (input, moves) =
            combinator::map_res(character::complete::digit1, str::parse::<usize>)(input)?;
        operations.push(OperationTypes::Num(moves));
        if !input.is_empty() {
            let rotation: &str;
            (input, rotation) = alpha1(input)?;
            match rotation {
                "L" => operations.push(OperationTypes::Rotation(Rotation::CounterClockWise)),
                "R" => operations.push(OperationTypes::Rotation(Rotation::Clockwise)),
                _ => panic!(),
            }
        }
    }
    Ok(("", operations))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part_1() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        assert_eq!(process_part_1(input), 6032);
    }

    #[test]
    fn test_process_part_2() {
        let input = include_str!("../../inputs/day22.txt");
        assert_eq!(process_part_2(input), 55364);
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
