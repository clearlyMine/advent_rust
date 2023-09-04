use std::ops::Index;

fn main() {
    let input = include_bytes!("../inputs/day8.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {:?}", res1);
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
}

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

fn process_part_1(input: &[u8]) -> usize {
    let lines: Vec<&[u8]> = input.split(|b| b == &b'\n').collect();

    let (rows, columns) = (lines[0].len(), lines.len());

    let mut grid_data: Vec<u8> = vec![];
    input
        .split(|b| b == &b'\n')
        .for_each(|line| line.iter().for_each(|c| grid_data.push(*c)));
    let grid: Matrix<u8> = Matrix::new(rows, columns, grid_data);

    let mut count = rows * 2 + columns * 2 - 4;
    for i in 1..rows - 1 {
        // println!("i={}", i + 1);
        for j in 1..columns - 1 {
            if check_if_visible(i, j, &grid) {
                count += 1;
                // println!("({},{})", i + 1, j + 1);
                continue;
            }
        }
    }
    count
}

fn check_if_visible(row: usize, col: usize, grid: &Matrix<u8>) -> bool {
    let height = grid.index((row, col));
    let mut visible = true;
    //checking from top
    for i in 0..row {
        if grid.index((i, col)) >= height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    visible = true;
    //checking from bottom
    for i in (row + 1)..grid.rows {
        if grid.index((i, col)) >= height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    visible = true;
    //checking from left
    for j in 0..col {
        if grid.index((row, j)) >= height {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    visible = true;
    //checking from right
    for j in (col + 1)..grid.cols {
        if grid.index((row, j)) >= height {
            visible = false;
            break;
        }
    }

    visible
}

fn process_part_2(input: &[u8]) -> i32 {
    let lines: Vec<&[u8]> = input.split(|b| b == &b'\n').collect();

    let (rows, columns) = (lines[0].len(), lines.len());

    let mut grid_data: Vec<u8> = vec![];
    input
        .split(|b| b == &b'\n')
        .for_each(|line| line.iter().for_each(|c| grid_data.push(*c)));
    let grid: Matrix<u8> = Matrix::new(rows, columns, grid_data);

    let mut scores: Vec<i32> = vec![];
    for i in 1..rows - 1 {
        for j in 1..columns - 1 {
            scores.push(calculate_scenic_score(i, j, &grid));
        }
    }
    *scores.iter().max().unwrap()
}

fn calculate_scenic_score(row: usize, col: usize, grid: &Matrix<u8>) -> i32 {
    let height = grid.index((row, col));
    let mut total = 1;
    let mut current = 0;
    //checking towards top
    for i in (0..row).rev() {
        current += 1;
        if grid.index((i, col)) >= height {
            break;
        }
    }
    total *= current;
    current = 0;
    //checking towards bottom
    for i in (row + 1)..grid.rows {
        current += 1;
        if grid.index((i, col)) >= height {
            break;
        }
    }
    total *= current;
    current = 0;
    //checking towards left
    for j in (0..col).rev() {
        current += 1;
        if grid.index((row, j)) >= height {
            break;
        }
    }
    total *= current;
    current = 0;
    for j in (col + 1)..grid.cols {
        current += 1;
        if grid.index((row, j)) >= height {
            break;
        }
    }
    total *= current;

    total
}
