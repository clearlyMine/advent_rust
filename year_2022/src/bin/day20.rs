use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input = include_str!("../../inputs/day20.txt");
    let res1 = process_part_1(input);
    println!("Part 1: {}", res1);
    println!("Time: {}μs", time_start.elapsed().as_micros());

    let time_start = Instant::now();
    let res2 = process_part_2(input);
    println!("Part 2:{}", res2);
    println!("Time: {}μs", time_start.elapsed().as_micros());
}

// struct Node{
//     value:isize,
//     original_index:usize
// }
// impl Node{
//     fn get_empty()->Node{
//         Node{value:0,original_index:0}
//     }
//     fn new-(value:isize,original_index:usize)->Node{
//         Node{value,original_index}
//     }
// }
//
// struct Nav{
//     prev:Node,
//     next:Node
// }
// impl Nav{
//     fn get_empty()->Nav{
//         Nav{prev:Node::get_empty(),next:Node::get_empty()}
//     }
// }
//
// struct Item{
//     node:Node,
//     nav:Nav,
//     prev:Node,
//     next:Node
// }
// impl Item{
//     fn get_empty()->Item{
//         let empty_node = Node{value:0,original_index:0};
//         Item{node:Node::get_empty(),nav:Nav::get_empty()}
//     }
// }
//
// fn process_part_1(input:&str)->isize{
//     let original_numbers = parse_input(input);
//     let length = original_numbers.len();
//     let last_index = length-1;
//     let mut head: Item = Item::get_empty();
//     head.node = Node::new(original_numbers[0].1,original_numbers[0].0);
//     head.nav.next = Node::new(original_numbers[1].1,original_numbers[1].0);
//     head.nav.prev= Node::new(original_numbers[last_index].1,original_numbers[last_index].0);
//     for i in 2..last_index{
//
//     }
//     0
// }
//

fn process_part_1(input: &str) -> isize {
    let original_numbers = parse_input(input);
    // dbg!(original_numbers.len());
    let mut new_numbers: Vec<(usize, isize)> = vec![];
    for (i, num) in original_numbers.iter().enumerate() {
        new_numbers.push((i, *num));
    }
    let length = original_numbers.len();
    let last_index = length - 1;

    for (i, num) in original_numbers.iter().enumerate() {
        // dbg!((i, num));
        if *num == 0 {
            continue;
        }
        // let length = length as isize;
        // let last_index = last_index as isize;
        let cur = find_cur_position(&new_numbers, i);
        // let new_index = (last_index as isize+*num+cur as isize)%last_index;
        let new_index = if num.is_positive() {
            ((num + cur as isize) % last_index as isize) as usize
        } else {
            let n = num + cur as isize;
            let last_index = last_index as isize;
            ((last_index + n % last_index) % last_index) as usize
        };
        let cur = cur as usize;
        new_numbers.remove(cur);
        new_numbers.insert(new_index, (i, *num));
    }
    let mut zero_index: usize = 0;
    for (i, (_, number)) in new_numbers.clone().iter().enumerate() {
        if 0 == *number {
            zero_index = i;
            break;
        }
    }
    let mut next_index = (zero_index + 1000) % length;
    let mut sum = new_numbers[next_index].1;

    next_index = (zero_index + 2000) % length;
    sum += new_numbers[next_index].1;

    next_index = (zero_index + 3000) % length;
    sum += new_numbers[next_index].1;
    sum
}

fn find_cur_position(numbers: &[(usize, isize)], original_index: usize) -> usize {
    for (i, num) in numbers.iter().enumerate() {
        if num.0 == original_index {
            return i;
        }
    }
    panic!();
}

fn process_part_2(input: &str) -> isize {
    let original_numbers = parse_input(input);
    let length = original_numbers.len();
    let decryption_key = 811589153;
    // dbg!(original_numbers.len());
    let mut new_numbers: Vec<(usize, isize)> = vec![];
    let mut original_numbers = original_numbers;
    for i in 0..length {
        let num = original_numbers[i];
        let new_num = num * decryption_key;
        original_numbers[i] = new_num;
        new_numbers.push((i, new_num));
    }
    let last_index = length - 1;

    for _ in 0..10 {
        for (i, num) in original_numbers.iter().enumerate() {
            // dbg!((i, num));
            if *num == 0 {
                continue;
            }
            // let length = length as isize;
            // let last_index = last_index as isize;
            let cur = find_cur_position(&new_numbers, i);
            // let new_index = (last_index as isize+*num+cur as isize)%last_index;
            let new_index = if num.is_positive() {
                ((num + cur as isize) % last_index as isize) as usize
            } else {
                let n = num + cur as isize;
                let last_index = last_index as isize;
                ((last_index + n % last_index) % last_index) as usize
            };
            let cur = cur as usize;
            new_numbers.remove(cur);
            new_numbers.insert(new_index, (i, *num));
        }
    }
    let mut zero_index: usize = 0;
    for (i, (_, number)) in new_numbers.clone().iter().enumerate() {
        if 0 == *number {
            zero_index = i;
            break;
        }
    }
    let mut next_index = (zero_index + 1000) % length;
    let mut sum = new_numbers[next_index].1;

    next_index = (zero_index + 2000) % length;
    sum += new_numbers[next_index].1;

    next_index = (zero_index + 3000) % length;
    sum += new_numbers[next_index].1;
    sum
}

fn parse_input(input: &str) -> Vec<isize> {
    let mut numbers: Vec<isize> = vec![];
    for line in input.lines() {
        let number = line.parse::<isize>().unwrap();
        numbers.push(number);
    }
    numbers
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_process_part_1() {
        let input = "1
2
-3
3
-2
0
4";
        assert_eq!(process_part_1(input), 3);
        let input = "1
2
-3
3
-2
0
4
204
-102";
        //[1, 2, 0, -102, 204, 3, -3, 4, -2]
        assert_eq!(process_part_1(input), 4);
    }

    #[test]
    fn test_process_part_2() {
        let input = "1
2
-3
3
-2
0
4";
        assert_eq!(process_part_2(input), 0);
    }
}
