use std::ops::Index;

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
    let b = include_bytes!("../input_day_8.txt");
    let lines: Vec<&[u8]> = b.split(|b| b == &b'\n').collect();

    let (rows, columns) = (lines[0].len(), lines.len());

    let mut grid_data: Vec<u8> = vec![];
    b.split(|b| b == &b'\n')
        .for_each(|line| line.iter().for_each(|c| grid_data.push(*c)));
    let grid: Matrix<u8> = Matrix::new(rows, columns, grid_data);

    let mut scores: Vec<i32> = vec![];
    for i in 1..rows - 1 {
        for j in 1..columns - 1 {
            scores.push(calculate_scenic_score(i, j, &grid));
        }
    }
    println!("{}", scores.iter().max().unwrap());
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
