use itertools::Itertools;
use std::fmt;
use std::{collections::HashMap, collections::HashSet, time::Instant};

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../../inputs/day23.txt");
#[allow(dead_code)]
const SAMPLE: &'static str = include_str!("../../inputs/day23_sample.txt");

fn main() {
    let time_start = Instant::now();
    let res1 = process_part_1(SAMPLE);
    println!("Part 1_sample: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res1 = process_part_1(INPUT);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(SAMPLE);
    println!("Part 2_sample: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(INPUT);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let grid = parse_grid(input);
    let start = Coord::new(
        0,
        grid[0]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '.')
            .unwrap()
            .0,
    );
    let end = Coord::new(
        grid.len() - 1,
        grid[grid.len() - 1]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '.')
            .unwrap()
            .0,
    );

    find_longest_path_slippery(&grid, start, end)
}

fn process_part_2(input: &str) -> usize {
    let grid = parse_grid(input);
    let start = Coord::new(
        0,
        grid[0]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '.')
            .unwrap()
            .0,
    );
    let end = Coord::new(
        grid.len() - 1,
        grid[grid.len() - 1]
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '.')
            .unwrap()
            .0,
    );

    // find_longest_path(&grid, start, end)

    // let width = grid[0].len();
    // let height = grid.len();
    // let mut visited = HashSet::new();
    // visited.insert(start);
    // dfs(
    //     grid,
    //     height,
    //     width,
    //     start,
    //     0,
    //     visited,
    //     end,
    //     &mut HashMap::new(),
    // )
    graph_theory(&grid, start, end)
}

fn find_longest_path_slippery(grid: &[Vec<char>], start: Coord, end: Coord) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut cache: HashMap<Coord, Vec<Coord>> = HashMap::new();
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut stack = vec![(start, 0, visited, vec![start])];
    while let Some((cur, steps, visited, path)) = stack.pop() {
        let neighbours = if grid[cur.row][cur.col] != '.' {
            let neighbour = match grid[cur.row][cur.col] {
                '>' => cur.get_east(),
                '<' => cur.get_west(),
                '^' => cur.get_north(),
                'v' => cur.get_south(),
                _ => panic!(),
            };
            if let Some(n) = neighbour {
                vec![n]
            } else {
                vec![]
            }
        } else {
            cur.get_all_neighbours()
        };

        for n in neighbours {
            if n.row >= height || n.col >= width || grid[n.row][n.col] == '#' {
                continue;
            }
            if !visited.contains(&n) {
                let mut visited = visited.clone();
                visited.insert(n.clone());

                let mut path = path.clone();
                path.push(n.clone());
                let steps = steps + 1;
                // dbg!(n, steps, visited.clone());
                if let Some(old_path) = cache.get_mut(&n) {
                    let path = &mut path;
                    if old_path == path {
                        continue;
                    }
                    if old_path.len() < path.len() {
                        *old_path = path.to_vec();
                    }
                } else {
                    cache.insert(n, path.clone());
                }
                stack.push((n, steps, visited, path));
            }
        }
    }
    cache.get(&end).unwrap().len() - 1
}

// fn find_longest_path(grid: &[Vec<char>], start: Coord, end: Coord) -> usize {
//     let width = grid[0].len();
//     let height = grid.len();
//     // print_grid(grid);
//
//     let mut cache: HashMap<Coord, Vec<Coord>> = HashMap::new();
//     let mut visited = HashSet::new();
//     visited.insert(start);
//     let mut stack = vec![(start, 0, visited, vec![start])];
//     // dbg!(queue.clone());
//     while let Some((cur, steps, visited, path)) = stack.pop() {
//         let neighbours = cur.get_all_neighbours();
//
//         for n in neighbours {
//             if n.row >= height || n.col >= width || grid[n.row][n.col] == '#' {
//                 continue;
//             }
//             if !visited.contains(&n) {
//                 let mut visited = visited.clone();
//                 visited.insert(n.clone());
//
//                 let mut path = path.clone();
//                 path.push(n.clone());
//                 let steps = steps + 1;
//                 // dbg!(n, steps, visited.clone());
//                 if let Some(old_path) = cache.get_mut(&n) {
//                     let path = &mut path;
//                     if old_path == path {
//                         continue;
//                     }
//                     if old_path.len() < path.len() {
//                         *old_path = path.to_vec();
//                     }
//                 } else {
//                     cache.insert(n, path.clone());
//                 }
//                 stack.push((n, steps, visited, path));
//             }
//         }
//         // if steps + 1 >= 90 {
//         //     dbg!(queue.clone());
//         // }
//     }
//     // dbg!(end, start);
//     cache.get(&end).unwrap().len() - 1
// }

// fn dfs(
//     grid: Vec<Vec<char>>,
//     height: usize,
//     width: usize,
//     node: Coord,
//     current_steps: usize,
//     visited: HashSet<Coord>,
//     target: Coord,
//     cache: &mut HashMap<(Coord, Coord), usize>,
// ) -> usize {
//     if node == target {
//         return current_steps;
//     }
//     let neighbours = node.get_all_neighbours();
//     let mut total_steps = vec![];
//     for next_node in neighbours {
//         if next_node.row >= height
//             || next_node.col >= width
//             || grid[next_node.row][next_node.col] == '#'
//         {
//             continue;
//         }
//         if !visited.contains(&next_node) {
//             if let Some(steps) = cache.get(&(node, next_node)) {
//                 total_steps.push(*steps+current_steps);
//                 continue;
//             }
//             let mut visited = visited.clone();
//             visited.insert(next_node.clone());
//
//             let steps = current_steps + 1;
//             // dbg!(n, steps, visited.clone());
//             total_steps.push(dfs(
//                 grid.clone(),
//                 height,
//                 width,
//                 next_node,
//                 steps,
//                 visited,
//                 target,
//                 cache,
//             ));
//         }
//     }
//     let max_steps = total_steps.iter().max().unwrap_or(&&0);
//     cache.
// }

struct WeightedGraph {
    edges: HashMap<String, HashSet<Edge>>,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Edge {
    destination: String,
    weight: u32,
    path: Vec<Coord>,
}

impl Edge {
    fn new(destination: String, weight: u32, path: Vec<Coord>) -> Edge {
        // let destination = get_node_name(destination);
        Edge {
            destination,
            weight,
            path,
        }
    }
}

impl WeightedGraph {
    fn new() -> Self {
        WeightedGraph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, node1: Coord, node2: Coord, weight: u32, path: Vec<Coord>) {
        let node1 = get_node_name(node1);
        let node2 = get_node_name(node2);
        self.edges
            .entry(node1.clone())
            .or_insert_with(HashSet::new)
            .insert(Edge::new(node2.clone(), weight, path.clone()));
        let mut path = path;
        path.reverse();
        self.edges
            .entry(node2)
            .or_insert_with(HashSet::new)
            .insert(Edge::new(node1, weight, path));
    }

    fn print(&self) {
        let mut ve: Vec<_> = self.edges.clone().into_iter().collect();
        ve.sort_by(|a, b| {
            let a = get_coord_from_name(&a.0.clone());
            let b = get_coord_from_name(&b.0.clone());
            a.row.cmp(&b.row).then_with(|| a.col.cmp(&b.col))
        });
        for (node, neighbors) in ve {
            print!("Node {}: ", node);
            for Edge {
                destination: neighbor,
                weight,
                path,
            } in neighbors
            {
                print!(
                    // "(to {} with weight {} along path {:?}) ",
                    // neighbor,
                    // weight,
                    // path
                    "(to {} with weight {}) ",
                    neighbor, weight,
                );
            }
            println!();
        }
    }
}

fn get_node_name(node: Coord) -> String {
    format!("{:2}{:2}", node.row, node.col)
}

fn get_coord_from_name(node: &String) -> Coord {
    let row = node[0..=1].trim().parse().unwrap();
    let col = node[2..=3].trim().parse().unwrap();
    Coord { row, col }
}

fn find_longest_path_between_nodes(grid: &[Vec<char>], start: Coord, end: Coord) -> Option<usize> {
    let width = grid[0].len();
    let height = grid.len();

    let mut cache: HashMap<Coord, Vec<Coord>> = HashMap::new();
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut stack = vec![(start, 0, visited, vec![start])];
    while let Some((cur, steps, visited, path)) = stack.pop() {
        let neighbours = cur
            .get_all_neighbours()
            .into_iter()
            .filter(|n| n.row < height && n.col < width && grid[n.row][n.col] != '#')
            .collect::<Vec<_>>();

        if neighbours.len() > 1 {
            continue;
        }

        let n = neighbours[0];

        if !visited.contains(&n) {
            let mut visited = visited.clone();
            visited.insert(n.clone());

            let mut path = path.clone();
            path.push(n.clone());
            let steps = steps + 1;
            // dbg!(n, steps, visited.clone());
            if let Some(old_path) = cache.get_mut(&n) {
                let path = &mut path;
                if old_path == path {
                    continue;
                }
                if old_path.len() < path.len() {
                    *old_path = path.to_vec();
                }
            } else {
                cache.insert(n, path.clone());
            }
            stack.push((n, steps, visited, path));
        }
    }
    if let Some(c) = cache.get(&end) {
        Some(c.len() - 1)
    } else {
        None
    }
}

fn graph_theory(grid: &[Vec<char>], start: Coord, end: Coord) -> usize {
    //convert to a graph where the start, end and junctions are the vertices and the connections
    //between them are edges
    let width = grid[0].len();
    let height = grid.len();
    let mut graph = WeightedGraph::new();
    let mut visited = HashSet::new();
    visited.insert(start);

    let next = start.get_south().unwrap();
    let mut queue = vec![(start, start, next, 1, vec![start, next])];
    //find all the nodes and then find the longest direct path between each pair of node, if one
    //exists

    while let Some((last_node, last, cur, length, path)) = queue.pop() {
        // dbg!(queue.clone());
        if cur == end {
            //target is supposed to be a node
            graph.add_edge(last_node, cur, length, path);
            //there is only one way to get to the target
            continue;
        }

        if let Some(temp) = graph.edges.get(&get_node_name(last_node)) {
            if temp
                .into_iter()
                .map(|edge| edge.path.clone())
                .any(|path| path.contains(&cur))
            {
                // println!("edge already added");
                //assuming only 1 way to reach another node from a node
                continue;
            }
        }

        let neighbours: Vec<Coord> = cur
            .get_all_neighbours()
            .into_iter()
            .filter(|n| last != *n && n.row < height && n.col < width && grid[n.row][n.col] != '#')
            .collect();

        if neighbours.len() > 1 {
            //junction found
            graph.add_edge(last_node, cur, length, path.clone());
            // println!("edge added");
            for n in neighbours {
                if !path.contains(&n) {
                    let p = vec![cur, n];
                    queue.push((cur, cur, n, 1, p));
                }
            }
        } else if neighbours.len() == 1 {
            let n = neighbours[0];
            if !path.contains(&n) {
                let mut p = path.clone();
                p.push(n);
                queue.push((last_node, cur, n, length + 1, p));
            }
        }
    }
    // graph.print();

    let start_node_name = get_node_name(start);
    let target_node_name = get_node_name(end);
    let mut visited = Vec::new();
    visited.push(start_node_name.clone());

    let mut queue = vec![(visited, start_node_name, 0)];
    // let mut paths = vec![];
    let mut longest_path = 0;
    while let Some((path, cur, weight)) = queue.pop() {
        if cur == target_node_name {
            longest_path = longest_path.max(weight);
            continue;
        }
        if let Some(neighbours) = graph.edges.get(&cur) {
            for n in neighbours {
                if !path.contains(&n.destination) {
                    let mut p = path.clone();
                    p.push(n.destination.clone());
                    queue.push((p, n.destination.clone(), n.weight + weight));
                }
            }
        }
    }
    // println!("All paths found");
    // let paths = paths
    //     .clone()
    //     .into_iter()
    //     .map(|p| {
    //         let mut path = vec![start];
    //         for temp in p.windows(2) {
    //             let (cur, dest) = (temp[0].clone(), temp[1].clone());
    //             let v = graph.edges.get(&cur).unwrap();
    //             let d = &v
    //                 .into_iter()
    //                 .filter(|e| e.destination == dest)
    //                 .take(1)
    //                 .collect::<Vec<_>>()[0]
    //                 .path;
    //             path.extend(d[1..].to_vec());
    //         }
    //         path
    //     })
    //     .collect::<Vec<_>>();
    //
    // // dbg!(paths.clone());
    // paths.into_iter().map(|p| p.len()).max().unwrap_or(0) - 1
    longest_path as usize
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    fn get_all_neighbours(&self) -> Vec<Coord> {
        let mut out: Vec<Coord> = vec![];
        if let Some(north) = self.get_north() {
            out.push(north);
        }
        if let Some(south) = self.get_south() {
            out.push(south);
        }
        if let Some(west) = self.get_west() {
            out.push(west);
        }
        if let Some(east) = self.get_east() {
            out.push(east);
        }
        out
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines().collect_vec();
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; lines[0].len()]; lines.len()];
    lines.iter().enumerate().for_each(|(row, line)| {
        for (col, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            // grid[row][col] = match char {
            //     _ => panic!("WTF"),
            // }
            grid[row][col] = char;
        }
    });
    grid
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<char>]) {
    let mut f: String = "".to_string();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            f = format!("{}{}", f, grid[row][col]);
        }
        f = format!("{}\n", f);
    }
    println!("{}", f);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(process_part_1(SAMPLE), 94)
    }

    #[test]
    fn part_1_input() {
        assert_eq!(process_part_1(INPUT), 2042)
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(process_part_2(SAMPLE), 154)
    }

    #[test]
    fn part_2_input() {
        assert_eq!(process_part_2(INPUT), usize::MAX)
    }
}
