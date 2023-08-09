use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;
fn main() {
    let b = include_str!("../input_day_14.txt");
    let lines: Vec<&str> = b.lines().collect();
    let mut straight_lines: Vec<StraightLine> = vec![];
    let mut x_max = usize::MIN;
    let mut x_min = usize::MAX;
    let mut y_max = usize::MIN;
    for line_number in 0..lines.len() {
        let line = lines[line_number];
        let coords: VecDeque<&str> = line.split(" -> ").collect();

        for i in 0..coords.len() - 1 {
            let (a, b) = coords[i].split_once(',').unwrap();
            let (c, d) = coords[i + 1].split_once(',').unwrap();
            let (x1, y1) = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());
            let (x2, y2) = (c.parse::<usize>().unwrap(), d.parse::<usize>().unwrap());
            straight_lines.push(StraightLine {
                start_coords: (x1, y1),
                end_coords: (x2, y2),
            });
            x_max = max(x_max, max(x1, x2));
            y_max = max(y_max, max(y1, y2));
            x_min = min(x_min, min(x1, x2));
        }
    }
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    for sl in straight_lines {
        let (x1, y1) = (sl.start_coords.0, sl.start_coords.1);
        let (x2, y2) = (sl.end_coords.0, sl.end_coords.1);
        if x1 == x2 {
            for j in y1.min(y2)..(y1.max(y2) + 1) {
                grid[x1][j] = true;
            }
        } else {
            for i in x1.min(x2)..(x1.max(x2) + 1) {
                grid[i][y1] = true;
            }
        }
    }
    y_max += 2;
    let mut count = 0;
    let origin: (usize, usize) = (500, 0);
    for i in 0..grid.len() {
        grid[i][y_max] = true;
    }

    while !is_resting_at_origin(&mut grid, origin) {
        count += 1;
    }
    println!("{}", count + 1);
}

fn is_resting_at_origin(grid: &mut Vec<Vec<bool>>, origin: (usize, usize)) -> bool {
    let (mut x, mut y) = origin;
    'main: for new_y in origin.1.. {
        for new_x in [x, x - 1, x + 1].into_iter() {
            if new_x == grid.len() || new_x == 0 {
                return false;
            }
            if !grid[new_x][new_y] {
                x = new_x;
                y = new_y;
                continue 'main;
            }
        }
        if (x, y) == origin {
            return true;
        }
        grid[x][y] = true;
        return false;
    }
    true
}

#[derive(Debug)]
struct StraightLine {
    start_coords: (usize, usize),
    end_coords: (usize, usize),
}