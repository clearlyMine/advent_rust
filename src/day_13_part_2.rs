use std::collections::VecDeque;
fn main() {
    let b = include_str!("../input_day_13.txt");
    let mut x: VecDeque<ValueType> = b
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(parse_pair)
        .collect();

    let decoder_one = parse_pair("[[2]]");
    let decoder_two = parse_pair("[[6]]");
    x.push_back(decoder_one.clone());
    x.push_back(decoder_two.clone());
    // println!("unsorted list");
    // for e in x.clone() {
    //     println!("{:?}", e);
    // }

    let y = merge_sort(x);
    // println!("\n\nsorted list");
    // for e in y.clone() {
    //     println!("{:?}", e);
    // }
    let mut decoder_one_index: i32 = -1;
    let mut decoder_two_index: i32 = -1;
    for (i, p) in y.iter().enumerate() {
        if *p == decoder_one {
            decoder_one_index = i as i32 + 1;
        }
        if *p == decoder_two {
            decoder_two_index = i as i32 + 1;
            break;
        }
    }
    println!("{}", decoder_one_index * decoder_two_index);
}
#[derive(PartialEq, Eq)]
enum Results {
    Greater,
    Smaller,
    Equal,
}
fn new_compare(left_list: ValueType, right_list: ValueType) -> bool {
    // println!(
    //     "\n\nnew_compare called with\nLEFT\n{:?}\nRIGHT\n{:?}",
    //     left_list, right_list
    // );
    let res = is_order_correct(left_list.clone(), right_list.clone());
    match res {
        Results::Equal => {
            // println!("lists are equal\n{:?}\n{:?}", left_list, right_list);
            // println!("-----------------------------------------------------------------");
            true
        }
        Results::Smaller => {
            // println!("RIGHT IS BIGGER");
            // println!("-----------------------------------------------------------------");
            true
        }
        Results::Greater => {
            // println!("LEFT IS BIGGER");
            // println!("-----------------------------------------------------------------");
            false
        }
    }
}
fn is_order_correct(left: ValueType, right: ValueType) -> Results {
    // println!(
    //     "\n\nis_order_correct called with\nLEFT\n{:?}\nRIGHT\n{:?}",
    //     left, right
    // );
    match left {
        ValueType::Num(left_num) => match right {
            ValueType::Num(right_num) => {
                if left_num < right_num {
                    return Results::Smaller;
                }
                if left_num > right_num {
                    return Results::Greater;
                }
                Results::Equal
            }
            ValueType::List(right_list) => {
                let mut temp: VecDeque<ValueType> = VecDeque::new();
                temp.push_back(ValueType::Num(left_num));
                is_order_correct(ValueType::List(temp), ValueType::List(right_list))
            }

            ValueType::String(_) => {
                panic!();
            }
        },
        ValueType::List(left_list) => match right {
            ValueType::List(right_list) => {
                for (i, l) in left_list.iter().enumerate() {
                    if right_list.len() <= i {
                        return Results::Greater;
                    }
                    let r = &right_list[i];
                    let res = is_order_correct(l.clone(), r.clone());
                    if res == Results::Equal {
                        continue;
                    }
                    return res;
                }
                if left_list.len() < right_list.len() {
                    return Results::Smaller;
                }
                Results::Equal
            }
            ValueType::Num(right_num) => {
                let mut temp: VecDeque<ValueType> = VecDeque::new();
                temp.push_back(ValueType::Num(right_num));
                is_order_correct(ValueType::List(left_list), ValueType::List(temp))
            }
            ValueType::String(_) => {
                panic!();
            }
        },
        ValueType::String(_) => {
            panic!();
        }
    }
}

fn merge_sort(m: VecDeque<ValueType>) -> VecDeque<ValueType> {
    if m.len() <= 1 {
        return m;
    }
    let mut left: VecDeque<ValueType> = VecDeque::new();
    let mut right: VecDeque<ValueType> = VecDeque::new();
    for (i, x) in m.iter().enumerate() {
        if i < (m.len()) / 2 {
            left.push_back(x.clone());
        } else {
            right.push_back(x.clone());
        }
    }
    left = merge_sort(left);
    right = merge_sort(right);
    merge(&mut left, &mut right)
}

fn merge(left: &mut VecDeque<ValueType>, right: &mut VecDeque<ValueType>) -> VecDeque<ValueType> {
    let mut out: VecDeque<ValueType> = VecDeque::new();
    loop {
        if left.is_empty() || right.is_empty() {
            break;
        }
        let fl = left.pop_front().unwrap();
        let fr = right.pop_front().unwrap();

        if new_compare(fl.clone(), fr.clone()) {
            // println!("second bigger");
            out.push_back(fl);
            right.push_front(fr);
        } else {
            // println!("first bigger");
            out.push_back(fr);
            left.push_front(fl);
        }
        // println!("out={:?}", out);
    }
    while let Some(l) = left.pop_front() {
        out.push_back(l);
    }
    while let Some(r) = right.pop_front() {
        out.push_back(r);
    }
    out
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum ValueType {
    Num(u8),
    String(char),
    List(VecDeque<ValueType>),
}

fn parse_pair(left: &str) -> ValueType {
    // let mut stack: Vec<&str> = vec![];
    let mut queue: VecDeque<ValueType> = VecDeque::new();

    let x: Vec<&str> = left.split(',').collect();
    let mut line_contents: VecDeque<String> = x.into_iter().map(|l| l.to_string()).collect();
    // println!("\nStarting line_contents {:?}", line_contents);
    loop {
        // println!("queue {:#?}", queue);
        // println!("line_contents {:?}", line_contents);
        let next = line_contents.pop_front().unwrap();
        if let Ok(num) = next.parse::<u8>() {
            // println!("Found number: {}", num);
            queue.push_back(ValueType::Num(num));
            continue;
        }
        //could be the start or end of new list
        let mut chars = next.chars();
        let first = chars.next().unwrap();
        if first == '[' {
            // println!("Found [");
            queue.push_back(ValueType::String('['));
            if next.len() > 1 {
                let s: String = chars.collect();
                line_contents.push_front(s);
            }
        } else if first == ']' {
            // println!("Found ]");
            //this for loop is for when there are multiple ]s
            for c in next.chars() {
                if c != ']' {
                    panic!();
                }

                let mut new_list: VecDeque<ValueType> = VecDeque::new();
                while let Some(last) = queue.pop_back() {
                    match last {
                        ValueType::String(l) => {
                            if l == '[' {
                                break;
                            } else {
                                panic!();
                            }
                        }
                        ValueType::Num(num) => {
                            new_list.push_front(ValueType::Num(num));
                        }
                        ValueType::List(list) => {
                            new_list.push_front(ValueType::List(list));
                        }
                    }
                }
                queue.push_back(ValueType::List(new_list));
            }
            // println!("queue:{:?}", queue);
        } else {
            //of the pattern number]
            // println!("Found pattern number]: {}", next);
            let mut y: String = "".to_string();
            let chars: Vec<char> = next.chars().collect();
            for c in chars.iter().take(chars.len() - 1) {
                y.push(*c);
            }
            line_contents.push_front("]".to_string());
            line_contents.push_front(y);
            continue;
        }
        // println!("queue:{:?}", queue);
        // println!("queue.len:{}", queue.len());
        if line_contents.is_empty() {
            return queue.pop_front().unwrap();
        }
        if queue.is_empty() {
            println!("queue is empty");
            panic!();
        }
    }
}
