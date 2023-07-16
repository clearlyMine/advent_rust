fn main() {
    let s = include_str!("../input_day_6.txt").lines().next().unwrap();
    let mut letters = s.chars();
    let mut out: String = "".to_string();
    for _ in 0..13 {
        out.push(letters.next().unwrap());
    }

    while let Some(c) = letters.next() {
        out.push(c);
        let last_fourteen_characters: Vec<char> = out.chars().rev().take(14).collect();
        if check_if_all_unique(last_fourteen_characters) {
            break;
        }
    }
    println!("{}", out.len());
}

fn check_if_all_unique(last_fourteen_characters: Vec<char>) -> bool {
    for i in 0..last_fourteen_characters.len() {
        for j in i + 1..last_fourteen_characters.len() {
            if last_fourteen_characters[i] == last_fourteen_characters[j] {
                return false;
            }
        }
    }
    true
}
