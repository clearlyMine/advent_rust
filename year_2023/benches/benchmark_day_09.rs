use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::process::Command;

fn run_day(day: &str) {
    let cmd = Command::new("cargo")
        .args(["run", "--release", "--bin", day])
        .output()
        .unwrap();
    let output = String::from_utf8(cmd.stdout).unwrap();
    black_box(output);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("100 samples");
    group.sample_size(100);
    group.sampling_mode(criterion::SamplingMode::Flat);
    group.bench_function(format!("Day {:02}", 9), |b| {
        b.iter(|| run_day(format!("day{:02}", 9).as_str()))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
