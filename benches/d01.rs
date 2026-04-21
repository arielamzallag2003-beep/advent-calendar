use std::fs;

use advent_calendar::d1::{v2, v3};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_d1(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day_01.txt")
        .unwrap_or_else(|e| panic!("Impossible de lire inputs/day_01.txt: {e}"));

    let mut group = c.benchmark_group("day1");

    // v2
    group.bench_function("v2_partie1", |b| {
        b.iter(|| v2::partie1(black_box(&input)))
    });
    group.bench_function("v2_partie2", |b| {
        b.iter(|| v2::partie2(black_box(&input)))
    });

    // v3
    group.bench_function("v3_partie1", |b| {
        b.iter(|| v3::partie1(black_box(&input)))
    });
    group.bench_function("v3_partie2", |b| {
        b.iter(|| v3::partie2(black_box(&input)))
    });

    group.finish();
}

criterion_group!(benches, bench_d1);
criterion_main!(benches);