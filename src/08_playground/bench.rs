use aoc::playground;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_lobby(c: &mut Criterion) {
    let input = include_str!("input");

    c.bench_function("08_playground_1", |b| {
        b.iter(|| {
            playground::part_1(input);
        })
    });

    c.bench_function("08_playground_2", |b| {
        b.iter(|| {
            playground::part_2(input);
        })
    });
}

criterion_group!(benches, bench_lobby);
criterion_main!(benches);
