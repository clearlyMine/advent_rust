use itertools::Itertools;
use std::{error::Error, fs, process::Command};

fn extract_microseconds(output: &str) -> usize {
    let start_index = "Time: ".len();
    output
        .lines()
        .into_iter()
        .filter(|t| t.starts_with("Time: "))
        .map(|t| {
            let last = if t.ends_with("ms") {
                t.len() - 2
            } else {
                t.len() - 3
            };
            let x = t.get(start_index..last).unwrap().parse::<usize>().unwrap();
            if t.ends_with("sec") {
                x * 1_000_000
            } else if t.ends_with("ms") {
                x * 1_000
            } else {
                x
            }
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin"))?
        .filter_map(|p| p.ok()?.path().file_stem()?.to_str().map(str::to_string))
        .sorted()
        .collect::<Vec<_>>();
    let mut total_time = 0;
    for day in &days {
        let cmd = Command::new("cargo")
            .args(["run", "--release", "--bin", day])
            .output()?;
        let output = String::from_utf8(cmd.stdout)?;
        println!("{}:\n{}", day, output);
        total_time += extract_microseconds(&output);
    }
    println!("Total time: {}sec", total_time / 1000 / 1000);
    Ok(())
}
