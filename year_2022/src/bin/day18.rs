use std::collections::{HashSet, VecDeque};
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day18.txt");
    let res1 = process_part_1(input);
    println!("Part 1:{}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2:{}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Point {
        Point { x, y, z }
    }

    fn neighbours(&self) -> HashSet<Point> {
        HashSet::from([
            Point {
                x: self.x - 1,
                ..*self
            },
            Point {
                y: self.y - 1,
                ..*self
            },
            Point {
                z: self.z - 1,
                ..*self
            },
            Point {
                x: self.x + 1,
                ..*self
            },
            Point {
                y: self.y + 1,
                ..*self
            },
            Point {
                z: self.z + 1,
                ..*self
            },
        ])
    }
}

fn process_part_1(input: &str) -> usize {
    let cubes_coords: Vec<Point> = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(',').collect();
            let x = split[0].parse::<isize>().unwrap();
            let y = split[1].parse::<isize>().unwrap();
            let z = split[2].parse::<isize>().unwrap();
            Point::new(x, y, z)
        })
        .collect();
    let covered: usize = cubes_coords
        .iter()
        .map(|cube| {
            cube.neighbours()
                .iter()
                .filter(|n| cubes_coords.contains(n))
                .count()
        })
        .sum();
    6 * cubes_coords.len() - covered
}

fn process_part_2(input: &str) -> usize {
    let mut cubes_coords: HashSet<Point> = parse_input(input);
    let ((_, mut max_x), (_, mut max_y), (_, mut max_z)) =
        get_max_min_in_all_dimensions(&cubes_coords);
    //add 1 cube to each dimension on both sides
    max_x += 2;
    max_y += 2;
    max_z += 2;
    cubes_coords = cubes_coords
        .into_iter()
        .map(|coords| Point::new(coords.x + 1, coords.y + 1, coords.z + 1))
        .collect();

    let starting_point = Point::new(0, 0, 0);
    let mut outside: HashSet<Point> = HashSet::new();

    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(starting_point);
    while let Some(coords) = queue.pop_front() {
        if is_inside(max_x, max_y, max_z, &coords)
            && !cubes_coords.contains(&coords)
            && !outside.contains(&coords)
        {
            outside.insert(coords);
            coords.neighbours().iter().for_each(|n| queue.push_back(*n));
        }
    }
    cubes_coords
        .iter()
        .map(|cube| {
            cube.neighbours()
                .iter()
                .filter(|neighbour| outside.contains(neighbour))
                .count()
        })
        .sum()
}

fn is_inside(max_x: isize, max_y: isize, max_z: isize, coords: &Point) -> bool {
    coords.x >= 0
        && coords.x <= max_x
        && coords.y >= 0
        && coords.y <= max_y
        && coords.z >= 0
        && coords.z <= max_z
}

fn get_max_min_in_all_dimensions(
    cubes_coords: &HashSet<Point>,
) -> ((isize, isize), (isize, isize), (isize, isize)) {
    let min_x = cubes_coords
        .clone()
        .iter()
        .map(|cube| cube.x)
        .min()
        .unwrap();
    let max_x = cubes_coords
        .clone()
        .iter()
        .map(|cube| cube.x)
        .max()
        .unwrap();
    let min_y = cubes_coords
        .clone()
        .iter()
        .map(|cube| cube.y)
        .min()
        .unwrap();
    let max_y = cubes_coords
        .clone()
        .iter()
        .map(|cube| cube.y)
        .max()
        .unwrap();
    let min_z = cubes_coords
        .clone()
        .iter()
        .map(|cube| cube.z)
        .min()
        .unwrap();
    let max_z = cubes_coords
        .clone()
        .iter()
        .map(|cube| cube.z)
        .max()
        .unwrap();
    ((min_x, max_x), (min_y, max_y), (min_z, max_z))
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Coords {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> HashSet<Point> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(',').collect();
            let x = split[0].parse::<isize>().unwrap();
            let y = split[1].parse::<isize>().unwrap();
            let z = split[2].parse::<isize>().unwrap();
            Point::new(x, y, z)
        })
        .collect::<HashSet<Point>>()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_process_part_1() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(process_part_1(input), 64);
    }

    #[test]
    fn test_process_part_2() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(process_part_2(input), 58);
    }
}
