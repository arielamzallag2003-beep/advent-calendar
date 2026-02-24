use advent_calendar::d3::Day03;
use advent_calendar::days::Day;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_d03(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day_03.txt").unwrap_or_default();
    let mut g = c.benchmark_group("Jour 03");
    g.bench_function("partie1", |b| b.iter(|| Day03.partie1(black_box(&input))));
    g.bench_function("partie2", |b| b.iter(|| Day03.partie2(black_box(&input))));
    g.finish();
}

criterion_group!(benches, bench_d03);
criterion_main!(benches);
