use std::process::Command;
fn main() {
    let cmd = Command::new("cargo")
        .args(["run", "--release", "--bin", "day07_part1"])
        .output()
        .unwrap();
    let output = String::from_utf8(cmd.stdout).unwrap();
    print!("{}", output);

    let cmd = Command::new("cargo")
        .args(["run", "--release", "--bin", "day07_part2"])
        .output()
        .unwrap();
    let output = String::from_utf8(cmd.stdout).unwrap();
    print!("{}", output);
}
