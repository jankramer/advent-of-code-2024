use advent_of_code_2024::day01;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = day01::parse(day01::INPUT);

    c.bench_function("day01_parse", |b| b.iter(|| day01::parse(day01::INPUT)));
    c.bench_function("day01_part_a", |b| b.iter(|| day01::solve_a(input.clone())));
    c.bench_function("day01_part_b", |b| b.iter(|| day01::solve_b(input.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
