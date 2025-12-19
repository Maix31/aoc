use criterion::{criterion_group, criterion_main, Criterion};
use aoc::factory;

fn bench_lobby(c: &mut Criterion) {
    let input = include_str!("input");

    c.bench_function("10_factory_1", |b| {
        b.iter(|| {
            factory::part_1(input);
        })
    });

    c.bench_function("10_factory_2", |b| {
        b.iter(|| {
            factory::part_2(input);
        })
    });
}

criterion_group!(benches, bench_lobby);
criterion_main!(benches);
