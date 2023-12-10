use itertools::Itertools;
use std::time::Instant;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day10.txt");
#[allow(dead_code)]
const SAMPLE_1: &'static str = include_str!("../../inputs/day10_sample.txt");
#[allow(dead_code)]
const SAMPLE_2: &'static str = include_str!("../../inputs/day10_sample_2.txt");
#[allow(dead_code)]
const SAMPLE_3: &'static str = include_str!("../../inputs/day10_sample_3.txt");
#[allow(dead_code)]
const SAMPLE_4: &'static str = include_str!("../../inputs/day10_sample_4.txt");
#[allow(dead_code)]
const SAMPLE_5: &'static str = include_str!("../../inputs/day10_sample_5.txt");
#[allow(dead_code)]
const SAMPLE_6: &'static str = include_str!("../../inputs/day10_sample_6.txt");

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
    find_loop(&grid).len() / 2
}

fn process_part_2(input: &str) -> isize {
    let grid = parse_grid(input);
    let loopity = find_loop(&grid);

    //shoelace formula
    let mut area: isize = 0;
    let n = loopity.len() as isize;
    for w in loopity.windows(2) {
        area += (w[0].row * w[1].col) as isize;
        area -= (w[0].col * w[1].row) as isize;
    }
    let area = isize::abs(area) / 2;

    //find number of tiles inside
    area - (n / 2) + 1
}

fn find_loop(grid: &Vec<Vec<Directions>>) -> Vec<Coord> {
    let (start_coord, start_shape) = determine_start_type(&grid);
    let mut loopity: Vec<Coord> = vec![start_coord];
    let (mut next, mut coming_from) = match start_shape {
        Directions::SE => (start_coord.get_east(), Directions::West),
        Directions::SW => (start_coord.get_south(), Directions::North),
        Directions::NS => (start_coord.get_north().unwrap(), Directions::South),
        _ => panic!("Unimplemented"),
    };
    loopity.push(next);

    while next != start_coord {
        let cur = grid[next.row][next.col];
        (next, coming_from) = match (cur, coming_from) {
            (Directions::NS, Directions::South) => (next.get_north().unwrap(), Directions::South),
            (Directions::NS, Directions::North) => (next.get_south(), Directions::North),
            (Directions::EW, Directions::West) => (next.get_east(), Directions::West),
            (Directions::EW, Directions::East) => (next.get_west().unwrap(), Directions::East),
            (Directions::NW, Directions::North) => (next.get_west().unwrap(), Directions::East),
            (Directions::NW, Directions::West) => (next.get_north().unwrap(), Directions::South),
            (Directions::NE, Directions::East) => (next.get_north().unwrap(), Directions::South),
            (Directions::NE, Directions::North) => (next.get_east(), Directions::West),
            (Directions::SE, Directions::South) => (next.get_east(), Directions::West),
            (Directions::SE, Directions::East) => (next.get_south(), Directions::North),
            (Directions::SW, Directions::West) => (next.get_south(), Directions::North),
            (Directions::SW, Directions::South) => (next.get_west().unwrap(), Directions::East),
            _ => panic!("Impossible"),
        };
        loopity.push(next);
    }
    loopity
}

fn determine_start_type(grid: &Vec<Vec<Directions>>) -> (Coord, Directions) {
    let mut start_pos: Coord = Coord { row: 0, col: 0 };
    'row: for (row, full_row) in grid.into_iter().enumerate() {
        for (col, char) in full_row.into_iter().enumerate() {
            if char == &Directions::Start {
                start_pos = Coord { row, col };
                break 'row;
            }
        }
    }

    let mut north_type: Option<Directions> = None;
    let mut south_type: Option<Directions> = None;
    let mut east_type: Option<Directions> = None;
    let mut west_type: Option<Directions> = None;
    if start_pos.row > 0 {
        let north = start_pos.get_north().unwrap();
        north_type = Some(grid[north.row][north.col]);
    }
    if start_pos.col > 0 {
        let west = start_pos.get_west().unwrap();
        west_type = Some(grid[west.row][west.col]);
    }
    if start_pos.row < grid.len() - 1 {
        let south = start_pos.get_south();
        south_type = Some(grid[south.row][south.col]);
    }
    if start_pos.col < grid[0].len() - 1 {
        let east = start_pos.get_east();
        east_type = Some(grid[east.row][east.col]);
    }

    let entry_from_north = if let Some(north_type) = north_type {
        north_type == Directions::NS || north_type == Directions::SE || north_type == Directions::SW
    } else {
        false
    };
    let entry_from_south = if let Some(south_type) = south_type {
        south_type == Directions::NS || south_type == Directions::NE || south_type == Directions::NW
    } else {
        false
    };
    let entry_from_west = if let Some(west_type) = west_type {
        west_type == Directions::EW || west_type == Directions::NE || west_type == Directions::SE
    } else {
        false
    };
    let entry_from_east = if let Some(east_type) = east_type {
        east_type == Directions::EW || east_type == Directions::NW || east_type == Directions::SW
    } else {
        false
    };

    if entry_from_north && entry_from_south {
        (start_pos, Directions::NS)
    } else if entry_from_north && entry_from_west {
        (start_pos, Directions::NW)
    } else if entry_from_north && entry_from_east {
        (start_pos, Directions::NE)
    } else if entry_from_south && entry_from_west {
        (start_pos, Directions::SW)
    } else if entry_from_south && entry_from_east {
        (start_pos, Directions::SE)
    } else if entry_from_west && entry_from_east {
        (start_pos, Directions::EW)
    } else {
        panic!("WTF!!!")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn get_south(&self) -> Coord {
        Coord {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn get_east(&self) -> Coord {
        Coord {
            row: self.row,
            col: self.col + 1,
        }
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Directions {
    North,
    South,
    East,
    West,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

fn parse_grid(input: &str) -> Vec<Vec<Directions>> {
    let lines = input.lines().collect_vec();
    let mut grid: Vec<Vec<Directions>> =
        vec![vec![Directions::Ground; lines[0].len()]; lines.len()];
    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, char)| {
            let dir = match char {
                '|' => Directions::NS,
                '-' => Directions::EW,
                'L' => Directions::NE,
                'J' => Directions::NW,
                '7' => Directions::SW,
                'F' => Directions::SE,
                'S' => Directions::Start,
                _ => Directions::Ground,
            };
            grid[row][col] = dir;
        });
    });
    grid
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample_1() {
        assert_eq!(process_part_1(SAMPLE_1), 4)
    }
    #[test]
    fn part_1_sample_2() {
        assert_eq!(process_part_1(SAMPLE_2), 8)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 6903)
    }

    #[test]
    fn part_2_sample_3() {
        assert_eq!(process_part_2(SAMPLE_3), 4)
    }

    #[test]
    fn part_2_sample_4() {
        assert_eq!(process_part_2(SAMPLE_4), 4)
    }

    #[test]
    fn part_2_sample_5() {
        assert_eq!(process_part_2(SAMPLE_5), 8)
    }

    #[test]
    fn part_2_sample_6() {
        assert_eq!(process_part_2(SAMPLE_6), 10)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), 265)
    }
}
