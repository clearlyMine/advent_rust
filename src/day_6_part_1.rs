fn main() {
    let s = include_str!("../input_day_6.txt").lines().next().unwrap();
    let mut letters = s.chars();
    let mut out: String = "".to_string();
    out.push(letters.next().unwrap());
    out.push(letters.next().unwrap());
    out.push(letters.next().unwrap());

    while let Some(c) = letters.next() {
        out.push(c);
        let last_four_characters: Vec<char> = out.chars().rev().take(4).collect();
        if check_if_all_unique(last_four_characters) {
            break;
        }
    }
    println!("{}", out.len());
}

fn check_if_all_unique(last_four_characters: Vec<char>) -> bool {
    for i in 0..last_four_characters.len() {
        for j in i + 1..last_four_characters.len() {
            if last_four_characters[i] == last_four_characters[j] {
                return false;
            }
        }
    }
    true
}
