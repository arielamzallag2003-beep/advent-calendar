use advent_calendar::d4::Day04;
use advent_calendar::days::Day;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_d04(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day_04.txt").unwrap_or_default();
    let mut g = c.benchmark_group("Jour 04");
    g.bench_function("partie1", |b| b.iter(|| Day04.partie1(black_box(&input))));
    g.bench_function("partie2", |b| b.iter(|| Day04.partie2(black_box(&input))));
    g.finish();
}

criterion_group!(benches, bench_d04);
criterion_main!(benches);
