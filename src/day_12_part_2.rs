use std::{collections::VecDeque, ops::Index};
#[derive(Clone)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(rows * cols, data.len());
        Self { rows, cols, data }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}
fn main() {
    let b = include_bytes!("../input_day_12.txt");
    let lines: Vec<&[u8]> = b.split(|b| b == &b'\n').collect();

    let (rows, columns) = (lines.len(), lines[0].len());

    let mut starting_points: Vec<(usize, usize)> = vec![];
    let mut grid_data: Vec<u8> = vec![];
    let (mut current_row, mut current_column) = (0, 0);
    b.split(|b| b == &b'\n').for_each(|line| {
        line.iter().for_each(|c| {
            if c == &b'S' || c == &b'a' {
                starting_points.push((current_row, current_column));
            }
            grid_data.push(*c);
            current_column += 1;
            if current_column == columns {
                current_column = 0;
                current_row += 1;
            }
        })
    });
    let grid: Matrix<u8> = Matrix::new(rows, columns, grid_data);
    // println!("Starting Grid:");
    // for r in 0..rows {
    //     for c in 0..columns {
    //         print!("{} ", grid.index((r, c)));
    //     }
    //     println!();
    // }
    let mut path_lengths: Vec<usize> = vec![];
    for start in starting_points {
        let p = bfs(grid.clone(), start);
        if p > 0 {
            path_lengths.push(p);
        }
    }
    path_lengths.sort_unstable();
    println!("{}", path_lengths[0]);
}

fn bfs(grid: Matrix<u8>, starting_point: (usize, usize)) -> usize {
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();

    queue.push_front((starting_point.0, starting_point.1, 0));
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid.cols]; grid.rows];
    while !queue.is_empty() {
        let current_position = queue.pop_front().unwrap();
        let mut current_height = grid.index((current_position.0, current_position.1));
        if current_height == &b'S' {
            current_height = &b'a';
        }
        if current_height == &b'E' {
            return current_position.2;
        }
        //move up
        if current_position.0 > 0 {
            let next_position = (current_position.0 - 1, current_position.1);
            if can_move(current_height, next_position, visited.clone(), grid.clone()) {
                queue.push_back((next_position.0, next_position.1, current_position.2 + 1));
                visited[next_position.0][next_position.1] = true;
            }
        }
        //move down
        if current_position.0 + 1 < grid.rows {
            let next_position = (current_position.0 + 1, current_position.1);
            if can_move(current_height, next_position, visited.clone(), grid.clone()) {
                queue.push_back((next_position.0, next_position.1, current_position.2 + 1));
                visited[next_position.0][next_position.1] = true;
            }
        }
        //move right
        if current_position.1 + 1 < grid.cols {
            let next_position = (current_position.0, current_position.1 + 1);
            if can_move(current_height, next_position, visited.clone(), grid.clone()) {
                queue.push_back((next_position.0, next_position.1, current_position.2 + 1));
                visited[next_position.0][next_position.1] = true;
            }
        }
        //move left
        if current_position.1 > 0 {
            let next_position = (current_position.0, current_position.1 - 1);
            if can_move(current_height, next_position, visited.clone(), grid.clone()) {
                queue.push_back((next_position.0, next_position.1, current_position.2 + 1));
                visited[next_position.0][next_position.1] = true;
            }
        }
    }
    0
}

fn can_move(
    current_height: &u8,
    next_position: (usize, usize),
    visited: Vec<Vec<bool>>,
    grid: Matrix<u8>,
) -> bool {
    let mut next_height = *grid.index(next_position);
    if next_height == b'E' {
        next_height = b'z';
    }
    if (next_position.0 < grid.rows && next_position.1 < grid.cols)
        && current_height + 1 >= next_height
        && !visited[next_position.0][next_position.1]
    {
        return true;
    }
    false
}
