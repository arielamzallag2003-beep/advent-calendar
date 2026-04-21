use criterion::{criterion_group, criterion_main, Criterion};

// Add benchmark functions here, e.g.:
//
// fn bench_day01(c: &mut Criterion) {
//     c.bench_function("day01 partie1", |b| {
//         let input = include_str!("../inputs/day_01.txt");
//         b.iter(|| advent_calendar::d1::Day01.partie1(input));
//     });
// }
//
// criterion_group!(benches, bench_day01);

fn bench_placeholder(_c: &mut Criterion) {}

criterion_group!(benches, bench_placeholder);
criterion_main!(benches);
