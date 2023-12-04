use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/day03.txt");

    let time_start = Instant::now();
    let res1 = process_part_1(input);
    println!("Part 1: {:?}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2: {:?}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

fn process_part_1(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut total_sum = 0;
    for (row, line) in lines.iter().enumerate() {
        let chars = line.chars().collect::<Vec<char>>();
        for (col, char) in chars.clone().into_iter().enumerate() {
            if char != '.' && !char.is_numeric() {
                //symbol encountered
                //check above
                if row > 0 {
                    let line_above = lines[row - 1].chars().collect::<Vec<char>>();
                    //check NE
                    if col > 0 {
                        if line_above[col - 1].is_numeric() {
                            let mut start = col - 1;
                            while start > 0 {
                                if line_above[start - 1].is_numeric() {
                                    start -= 1;
                                } else {
                                    break;
                                }
                            }
                            let mut end = col - 1;
                            while end < line.len() - 1 {
                                if line_above[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            total_sum += &line_above[start..=end]
                                .iter()
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap();
                        }
                    }
                    //check North
                    if col > 0 && !line_above[col - 1].is_numeric() {
                        if line_above[col].is_numeric() {
                            let start = col;

                            let mut end = col;
                            while end < line.len() - 1 {
                                if line_above[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            total_sum += &line_above[start..=end]
                                .iter()
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap();
                        }
                    }
                    //check NW
                    if !line_above[col].is_numeric() {
                        if line_above[col + 1].is_numeric() {
                            let start = col + 1;

                            let mut end = col + 1;
                            while end < line.len() - 1 {
                                if line_above[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            total_sum += &line_above[start..=end]
                                .iter()
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap();
                        }
                    }
                }
                //check East
                if col > 0 && chars[col - 1].is_numeric() {
                    let end = col - 1;
                    let mut start = col - 1;
                    while start > 0 {
                        if chars[start - 1].is_numeric() {
                            start -= 1;
                        } else {
                            break;
                        }
                    }

                    total_sum += &chars[start..=end]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                }
                //check West
                if col < chars.len() - 1 && chars[col + 1].is_numeric() {
                    let start = col + 1;
                    let mut end = col + 1;
                    while end < chars.len() - 1 {
                        if chars[end + 1].is_numeric() {
                            end += 1;
                        } else {
                            break;
                        }
                    }
                    total_sum += &chars[start..=end]
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                }
                //check below
                if row < lines.len() - 1 {
                    let line_below = lines[row + 1].chars().collect::<Vec<char>>();
                    //check SE
                    if col > 0 {
                        if line_below[col - 1].is_numeric() {
                            let mut start = col - 1;
                            while start > 0 {
                                if line_below[start - 1].is_numeric() {
                                    start -= 1;
                                } else {
                                    break;
                                }
                            }
                            let mut end = col - 1;
                            while end < line.len() - 1 {
                                if line_below[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            total_sum += &line_below[start..=end]
                                .iter()
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap();
                        }
                    }
                    //check South
                    if col > 0 && !line_below[col - 1].is_numeric() {
                        if line_below[col].is_numeric() {
                            let start = col;

                            let mut end = col;
                            while end < line.len() - 1 {
                                if line_below[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            total_sum += &line_below[start..=end]
                                .iter()
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap();
                        }
                    }
                    //check SW
                    if !line_below[col].is_numeric() {
                        if line_below[col + 1].is_numeric() {
                            let start = col + 1;

                            let mut end = col + 1;
                            while end < line.len() - 1 {
                                if line_below[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            total_sum += &line_below[start..=end]
                                .iter()
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap();
                        }
                    }
                }
            }
        }
    }
    total_sum
}

fn process_part_2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut total_sum = 0;
    for (row, line) in lines.iter().enumerate() {
        let chars = line.chars().collect::<Vec<char>>();
        for (col, char) in chars.clone().into_iter().enumerate() {
            if char == '*' {
                //symbol encountered
                let mut values: Vec<usize> = vec![];
                //check above
                if row > 0 {
                    let line_above = lines[row - 1].chars().collect::<Vec<char>>();
                    //check NE
                    if col > 0 {
                        if line_above[col - 1].is_numeric() {
                            let mut start = col - 1;
                            while start > 0 {
                                if line_above[start - 1].is_numeric() {
                                    start -= 1;
                                } else {
                                    break;
                                }
                            }
                            let mut end = col - 1;
                            while end < line.len() - 1 {
                                if line_above[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            values.push(
                                line_above[start..=end]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<usize>()
                                    .unwrap(),
                            );
                        }
                    }
                    //check North
                    if col > 0 && !line_above[col - 1].is_numeric() {
                        if line_above[col].is_numeric() {
                            let start = col;

                            let mut end = col;
                            while end < line.len() - 1 {
                                if line_above[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            values.push(
                                line_above[start..=end]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<usize>()
                                    .unwrap(),
                            );
                        }
                    }
                    //check NW
                    if !line_above[col].is_numeric() {
                        if line_above[col + 1].is_numeric() {
                            let start = col + 1;

                            let mut end = col + 1;
                            while end < line.len() - 1 {
                                if line_above[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            values.push(
                                line_above[start..=end]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<usize>()
                                    .unwrap(),
                            );
                        }
                    }
                }
                //check East
                if col > 0 && chars[col - 1].is_numeric() {
                    let end = col - 1;
                    let mut start = col - 1;
                    while start > 0 {
                        if chars[start - 1].is_numeric() {
                            start -= 1;
                        } else {
                            break;
                        }
                    }

                    values.push(
                        chars[start..=end]
                            .iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap(),
                    );
                }
                //check West
                if col < chars.len() - 1 && chars[col + 1].is_numeric() {
                    let start = col + 1;
                    let mut end = col + 1;
                    while end < chars.len() - 1 {
                        if chars[end + 1].is_numeric() {
                            end += 1;
                        } else {
                            break;
                        }
                    }
                    values.push(
                        chars[start..=end]
                            .iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap(),
                    );
                }
                //check below
                if row < lines.len() - 1 {
                    let line_below = lines[row + 1].chars().collect::<Vec<char>>();
                    //check SE
                    if col > 0 {
                        if line_below[col - 1].is_numeric() {
                            let mut start = col - 1;
                            while start > 0 {
                                if line_below[start - 1].is_numeric() {
                                    start -= 1;
                                } else {
                                    break;
                                }
                            }
                            let mut end = col - 1;
                            while end < line.len() - 1 {
                                if line_below[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            values.push(
                                line_below[start..=end]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<usize>()
                                    .unwrap(),
                            );
                        }
                    }
                    //check South
                    if col > 0 && !line_below[col - 1].is_numeric() {
                        if line_below[col].is_numeric() {
                            let start = col;

                            let mut end = col;
                            while end < line.len() - 1 {
                                if line_below[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            values.push(
                                line_below[start..=end]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<usize>()
                                    .unwrap(),
                            );
                        }
                    }
                    //check SW
                    if !line_below[col].is_numeric() {
                        if line_below[col + 1].is_numeric() {
                            let start = col + 1;

                            let mut end = col + 1;
                            while end < line.len() - 1 {
                                if line_below[end + 1].is_numeric() {
                                    end += 1;
                                } else {
                                    break;
                                }
                            }
                            // dbg!(start, end);
                            values.push(
                                line_below[start..=end]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<usize>()
                                    .unwrap(),
                            );
                        }
                    }
                }
                if values.len() == 2 {
                    total_sum += values.into_iter().product::<usize>();
                }
            }
        }
    }
    total_sum
}
