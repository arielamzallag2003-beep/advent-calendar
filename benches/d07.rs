use advent_calendar::d7::Day07;
use advent_calendar::days::Day;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_d07(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day_07.txt").unwrap_or_default();
    let mut g = c.benchmark_group("Jour 07");
    g.bench_function("partie1", |b| b.iter(|| Day07.partie1(black_box(&input))));
    g.bench_function("partie2", |b| b.iter(|| Day07.partie2(black_box(&input))));
    g.finish();
}

criterion_group!(benches, bench_d07);
criterion_main!(benches);
