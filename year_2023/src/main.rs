use std::process::Command;

fn main() {
    for i in 1..=25 {
        let cmd = Command::new("cargo")
            .args(["run", "--release", "--bin", format!("day{:02}", i).as_str()])
            .output()
            .unwrap();
        let output = String::from_utf8(cmd.stdout).unwrap();
        if output.is_empty() {
            break;
        }
        println!("Day {:02}", i);
        println!("{}", output);
    }
}
