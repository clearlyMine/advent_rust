use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::RangeInclusive;
use std::time::Instant;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day22.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day22_sample.txt");

fn main() {
    let time_start = Instant::now();
    let res1 = process_part_1(INPUT);
    // let res1 = process_part_1(SAMPLE);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    // let res2 = process_part_2(SAMPLE);
    // println!("Part 2: {:?}", res2);
    let res2 = process_part_2(INPUT);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let bricks = parse_bricks(input);
    // dbg!(bricks.clone());
    //drop all bricks
    let bricks = drop_bricks(bricks);
    let (supported_by, supporting) = find_supports(&bricks);

    let mut s = 0;
    'main: for brick in bricks {
        let empty: Vec<Brick> = vec![];
        let supporting = supporting.get(&brick).unwrap_or(&empty);
        for sup in supporting {
            if supported_by.get(sup).unwrap().len() == 1 {
                continue 'main;
            }
        }
        s += 1;
    }

    s
}

fn process_part_2(input: &str) -> usize {
    let bricks = parse_bricks(input);
    // dbg!(bricks.clone());
    //drop all bricks
    let bricks = drop_bricks(bricks);
    let (supported_by, supporting) = find_supports(&bricks);

    let mut total = 0;

    // for brick in bricks {
    //     // println!("Breaking {}", brick.clone());
    //     let mut h = HashSet::new();
    //     h.insert(brick);
    //     total += get_broken_bricks_above(&supporting, &supported_by, h, &brick).len() - 1;
    // }
    for i in 0..bricks.len() {
        // let mut b = bricks.clone();
        // b.remove(i);
        let mut b = bricks[..i].to_vec();
        b.extend(bricks[i + 1..].to_vec());
        total += get_broken_above_2(b);
    }
    total

    // let mut out = 0;
    // for brick in bricks {
    //     dbg!(brick.clone());
    //     let empty: Vec<Brick> = vec![];
    //     let s = supporting.get(&brick).unwrap_or(&empty);
    //     if s.is_empty() {
    //         continue;
    //     }
    //
    //     let mut broken_bricks: Vec<Brick> = vec![brick];
    //     for sup in s {
    //         if supported_by.get(sup).unwrap().len() == 1 {
    //             broken_bricks.push(*sup);
    //         }
    //     }
    //
    //     loop {
    //         let mut newly_broken = 0;
    //         for i in 0..broken_bricks.len() {
    //             let brick = broken_bricks[i];
    //             let s = supporting.get(&brick).unwrap_or(&empty);
    //             if s.is_empty() {
    //                 continue;
    //             }
    //
    //             for sup in s {
    //                 if supported_by.get(sup).unwrap().len() == 1 {
    //                     broken_bricks.push(*sup);
    //                     newly_broken += 1;
    //                 }
    //             }
    //         }
    //         if newly_broken == 0 {
    //             break;
    //         }
    //     }
    //
    // let mut queue: Vec<Brick> = vec![];
    // queue.extend(s);
    // let mut visited: HashSet<Brick> = HashSet::new();
    //
    // dbg!(queue.clone());
    // while let Some(sup) = queue.pop() {
    //     if supported_by.get(sup).unwrap().len() > 1 {
    //         continue ;
    //     }
    //     if let Some(s) = supporting.get(&sup) {
    //         queue.extend(s);
    //         visited.extend(s);
    //     }
    //     dbg!(queue.clone(), out);
    //     // queue.sort_by(|a, b| a.end.z.cmp(&b.end.z));
    //     // queue.dedup();
    //     // queue.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    //     // queue.dedup();
    // }
    // out += visited.len();
    // }

    // out
}

fn get_broken_above_2(bricks: Vec<Brick>) -> usize {
    let mut bricks = bricks.clone();
    let mut total = 0;
    //drop bricks
    loop {
        let mut to_drop = vec![];
        for i in 0..bricks.len() {
            let brick = bricks[i];
            if brick.start.z == 1 {
                continue;
            }

            let mut brick_below = false;
            let z_below = brick.start.z - 1;
            let cur_x = brick.start.x..=brick.end.x;
            let cur_y = brick.start.y..=brick.end.y;
            for a in 0..bricks.len() {
                let below = bricks[a];
                if below.end.z == z_below {
                    if check_overlap(cur_x.clone(), below.start.x..=below.end.x)
                        && check_overlap(cur_y.clone(), below.start.y..=below.end.y)
                    {
                        brick_below = true;
                        break;
                    }
                }
            }
            if brick_below {
                continue;
            }
            to_drop.push(i);
        }
        if to_drop.len() == 0 {
            break;
        }
        total += to_drop.len();
        let mut removed = 0;
        for i in to_drop {
            bricks.remove(i - removed);
            removed += 1;
        }
        // dbg!(bricks.clone());
    }
    total
}

fn get_broken_bricks_above(
    supporting: &HashMap<Brick, Vec<Brick>>,
    supported_by: &HashMap<Brick, Vec<Brick>>,
    broken_bricks: HashSet<Brick>,
    last_broken: &Brick,
) -> HashSet<Brick> {
    // dbg!(last_broken.clone(), broken_bricks.clone(),);
    let mut broken_bricks = broken_bricks.clone();
    let s = supporting.get(last_broken);
    if s.is_none() {
        return broken_bricks;
    }
    let s = s.unwrap();

    let mut newly_broken = vec![];
    for sup in s {
        let sup_by = supported_by.get(&sup).unwrap();

        let len = sup_by.len();
        if sup_by
            .into_iter()
            .filter(|s| broken_bricks.contains(s))
            .count()
            == len
        {
            newly_broken.push(sup);
        }
    }
    broken_bricks.extend(newly_broken.clone());
    for nb in newly_broken {
        broken_bricks.extend(get_broken_bricks_above(
            supporting,
            supported_by,
            broken_bricks.clone(),
            &nb,
        ));
    }
    broken_bricks
}

fn find_supports(bricks: &Vec<Brick>) -> (HashMap<Brick, Vec<Brick>>, HashMap<Brick, Vec<Brick>>) {
    // dbg!(bricks.clone());
    let mut supported_by: HashMap<Brick, Vec<Brick>> = HashMap::new();
    let mut supporting: HashMap<Brick, Vec<Brick>> = HashMap::new();

    //find all supports
    for i in 0..bricks.len() {
        let brick = bricks[i];
        let z_above = brick.end.z + 1;
        let z_below = brick.start.z - 1;
        let cur_x = brick.start.x..=brick.end.x;
        let cur_y = brick.start.y..=brick.end.y;
        for j in 0..bricks.len() {
            if i == j {
                continue;
            }
            let next = bricks[j];
            if next.end.z == z_below
                && check_overlap(cur_x.clone(), next.start.x..=next.end.x)
                && check_overlap(cur_y.clone(), next.start.y..=next.end.y)
            {
                supported_by
                    .entry(brick)
                    .and_modify(|i| i.push(next))
                    .or_insert(vec![next]);
            }
            if next.start.z == z_above
                && check_overlap(cur_x.clone(), next.start.x..=next.end.x)
                && check_overlap(cur_y.clone(), next.start.y..=next.end.y)
            {
                supporting
                    .entry(brick)
                    .and_modify(|i| i.push(next))
                    .or_insert(vec![next]);
            }
        }
    }
    // dbg!(supporting.clone());
    // dbg!(supported_by.clone());
    (supported_by, supporting)
}

fn drop_bricks(bricks: Vec<Brick>) -> Vec<Brick> {
    let mut bricks = bricks;
    //drop bricks
    loop {
        let mut dropped_bricks = 0;
        // if bricks[0].start.z > 1 {
        //     bricks[0] = bricks[0].drop_1_below_z().unwrap();
        // }
        for i in 0..bricks.len() {
            let brick = bricks[i];
            if brick.start.z == 1 {
                continue;
            }

            let mut brick_below = false;
            let z_below = brick.start.z - 1;
            let cur_x = brick.start.x..=brick.end.x;
            let cur_y = brick.start.y..=brick.end.y;
            for a in (0..=i - 1).rev() {
                let below = bricks[a];
                if below.end.z == z_below {
                    if check_overlap(cur_x.clone(), below.start.x..=below.end.x)
                        && check_overlap(cur_y.clone(), below.start.y..=below.end.y)
                    {
                        brick_below = true;
                        break;
                    }
                }
            }
            if brick_below {
                continue;
            }
            // print!("Dropping {}", brick.clone());
            bricks[i] = brick.drop_1_below_z().unwrap();
            // println!(" to {}", bricks[i].clone());
            dropped_bricks += 1;
        }
        if dropped_bricks == 0 {
            break;
        }
        bricks.sort_by(|a, b| a.end.z.cmp(&b.end.z));
        bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
        // dbg!(bricks.clone());
    }
    bricks
}

fn check_overlap(first: RangeInclusive<usize>, second: RangeInclusive<usize>) -> bool {
    for i in first {
        for j in second.clone() {
            if i == j {
                return true;
            }
        }
    }
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord {
    fn new(x: usize, y: usize, z: usize) -> Coord {
        Coord { x, y, z }
    }

    // fn get_south(&self) -> Option<Coord> {
    //     Some(Coord {
    //         x: self.x + 1,
    //         y: self.y,
    //     })
    // }
    //
    // fn get_east(&self) -> Option<Coord> {
    //     Some(Coord {
    //         x: self.x,
    //         y: self.y + 1,
    //     })
    // }
    //
    // fn get_west(&self) -> Option<Coord> {
    //     if self.y == 0 {
    //         return None;
    //     }
    //     Some(Coord {
    //         x: self.x,
    //         y: self.y - 1,
    //     })
    // }
    //
    // fn get_north(&self) -> Option<Coord> {
    //     if self.x == 0 {
    //         return None;
    //     }
    //     Some(Coord {
    //         x: self.x - 1,
    //         y: self.y,
    //     })
    // }
    //
    // fn get_all_neighbours(&self) -> Vec<Coord> {
    //     let mut out: Vec<Coord> = vec![];
    //     if let Some(north) = self.get_north() {
    //         out.push(north);
    //     }
    //     if let Some(south) = self.get_south() {
    //         out.push(south);
    //     }
    //     if let Some(west) = self.get_west() {
    //         out.push(west);
    //     }
    //     if let Some(east) = self.get_east() {
    //         out.push(east);
    //     }
    //     out
    // }

    fn get_higher_z(&self) -> Option<Self> {
        Some(Coord {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        })
    }

    fn get_lower_z(&self) -> Option<Self> {
        if self.z == 0 {
            None
        } else {
            Some(Coord {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            })
        }
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Brick {
    start: Coord,
    end: Coord,
    label: usize,
}

impl Brick {
    fn drop_1_below_z(self) -> Option<Self> {
        let new_start = self.start.get_lower_z();
        let new_end = self.end.get_lower_z();
        if new_start.is_none() || new_end.is_none() {
            return None;
        }
        Some(Brick {
            start: new_start.unwrap(),
            end: new_end.unwrap(),
            label: self.label,
        })
    }
}

impl fmt::Debug for Brick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.start;
        let e = self.end;
        write!(
            f,
            "Brick #{:4} ({},{},{})->({},{},{})",
            self.label, s.x, s.y, s.z, e.x, e.y, e.z
        )
    }
}

impl fmt::Display for Brick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.start;
        let e = self.end;
        write!(
            f,
            "Brick #{:4} ({},{},{})->({},{},{})",
            self.label, s.x, s.y, s.z, e.x, e.y, e.z
        )
    }
}
fn parse_bricks(input: &str) -> Vec<Brick> {
    let lines = input.lines().collect_vec();

    let mut bricks: Vec<Brick> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let (brick_start, brick_end) = line.split_once("~").unwrap();
            let mut brick_start = brick_start.split(",");
            let brick_start = Coord::new(
                brick_start.next().unwrap().parse::<usize>().unwrap(),
                brick_start.next().unwrap().parse::<usize>().unwrap(),
                brick_start.next().unwrap().parse::<usize>().unwrap(),
            );

            let mut brick_end = brick_end.split(",");
            let brick_end = Coord::new(
                brick_end.next().unwrap().parse::<usize>().unwrap(),
                brick_end.next().unwrap().parse::<usize>().unwrap(),
                brick_end.next().unwrap().parse::<usize>().unwrap(),
            );
            let brick = Brick {
                start: brick_start,
                end: brick_end,
                label: i + 1,
            };
            brick
        })
        .collect();
    bricks.sort_by(|a, b| a.end.z.cmp(&b.end.z));
    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    bricks
    // for x in brick_start.x..brick_end.x {
    //     let coord = Coord::new(x, brick_start.y, brick_start.z);
    //     map.insert(coord, brick);
    // }
    // for x in brick_end.x..brick_start.x {
    //     let coord = Coord::new(x, brick_start.y, brick_start.z);
    //     map.insert(coord, brick);
    // }
    //
    // for y in brick_start.y..brick_end.y {
    //     let coord = Coord::new(brick_start.x, y, brick_start.z);
    //     map.insert(coord, brick);
    // }
    // for y in brick_end.y..brick_start.y {
    //     let coord = Coord::new(brick_start.x, y, brick_start.z);
    //     map.insert(coord, brick);
    // }
    //
    // for z in brick_start.z..brick_end.z {
    //     let coord = Coord::new(brick_start.x, brick_start.y, z);
    //     map.insert(coord, brick);
    // }
    // for z in brick_end.z..brick_start.z {
    //     let coord = Coord::new(brick_start.x, brick_start.y, z);
    //     map.insert(coord, brick);
    // }
    // map
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 5)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 457)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 7)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), usize::MAX)
    }
}
