// use std::collections::vec_deque;
// use std::sync::mpsc::sync_channel;
// use std::sync::Arc;
// use std::thread;
use std::collections::VecDeque;
fn main() {
    let b = include_str!(".../inputs/day13.txt");
    let mut count = 0;
    let mut out = 0;
    b.split("\n\n").for_each(|pair| {
        // if let Some(pair) = b.split("\n\n").nth(1) {
        count += 1;
        // println!("Pair {}", count);
        let mut p = pair.lines();
        let l = p.next().unwrap();
        let r = p.next().unwrap();

        let left = parse_pair(l);
        // println!("left={:?}\n\n", left);
        let right = parse_pair(r);
        // println!("right={:?}\n\n\n\n", right);

        let fl = &left[0];
        let fr = &right[0];
        let main_left_list: VecDeque<ValueType>;
        let main_right_list: VecDeque<ValueType>;
        match fl {
            ValueType::List(list) => {
                main_left_list = list.clone();
            }
            _ => {
                panic!();
            }
        }
        match fr {
            ValueType::List(list) => {
                main_right_list = list.clone();
            }
            _ => {
                panic!();
            }
        }
        if compare(main_left_list, main_right_list) {
            println!("Pair {} is equal\n\n\n", count);
            out += count;
        }
        // }
    });
    println!("{}", out);
}

fn compare(left_list: VecDeque<ValueType>, right_list: VecDeque<ValueType>) -> bool {
    // println!("Comparing \n{:?} \nand\n{:?}", left_list, right_list);
    if left_list.len() > right_list.len() {
        return false;
    }
    if left_list.is_empty() && right_list.is_empty() {
        return true;
    }
    for (i, l) in left_list.iter().enumerate() {
        // println!("running for {:?}", l.clone());
        match l {
            ValueType::List(ll) => {
                if ll.is_empty() {
                    continue;
                }
                let r = right_list[i].clone();
                // println!("r is {:?}", r);
                match r {
                    ValueType::List(rl) => {
                        if !compare(ll.clone(), rl.clone()) {
                            // println!("{:?}!={:?}", ll, rl.clone());
                            return false;
                        }
                    }
                    ValueType::Num(right_num) => {
                        if let ValueType::Num(first_left_num) = ll[0] {
                            if first_left_num > right_num {
                                // println!("{:?}!={:?}", l, r.clone());
                                return false;
                            }
                        }
                    }
                    ValueType::String(_) => {
                        panic!()
                    }
                }
            }
            ValueType::Num(left_num) => {
                let r = right_list[i].clone();
                match r {
                    ValueType::Num(right_num) => {
                        if *left_num > right_num {
                            // println!("{:?}!={:?}", l, r);
                            return false;
                        }
                    }
                    ValueType::List(right_list) => {
                        //number to list
                        if right_list.is_empty() {
                            // println!("{:?}!={:?}", l, right_list.clone());
                            return false;
                        }
                        if let ValueType::Num(first_right_num) = right_list[0] {
                            if *left_num > first_right_num {
                                // println!("{:?}!={:?}", l, right_list.clone());
                                return false;
                            }
                        }
                    }
                    ValueType::String(_) => {
                        panic!()
                    }
                }
            }
            ValueType::String(_) => {
                panic!();
            }
        }
    }
    // println!("{:?}=={:?}", left_list, right_list);

    true
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum ValueType {
    Num(u8),
    String(char),
    List(VecDeque<ValueType>),
}

fn parse_pair(left: &str) -> VecDeque<ValueType> {
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
            return queue;
        }
        if queue.len() == 0 {
            println!("queue is empty");
            break;
        }
    }
    let v: VecDeque<ValueType> = VecDeque::new();
    v
    // stack.push(x[0]);
    // let mut i = 1;
    // while !stack.is_empty() {
    //     let next = x[i];
    //
    //     if next.len() > 1 {
    //         let chars = next.chars();
    //         let l = chars.last().unwrap();
    //         if l == ']' {
    //             let new_list: List = List {
    //                 num: None,
    //                 list: None,
    //             };
    //             while !stack.is_empty() {
    //                 let last = stack.pop().unwrap();
    //                 let c = next.chars();
    //                 let p = c.last().unwrap();
    //                 if last.len() > 1 {
    //                     if p == '[' {
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     let last = stack.pop().unwrap();
    // }
}
