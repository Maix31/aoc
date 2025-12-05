use aoc::cafeteria;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_lobby(c: &mut Criterion) {
    let input = include_str!("input");

    c.bench_function("05_cafeteria_1", |b| {
        b.iter(|| {
            cafeteria::part_1(input);
        })
    });

    c.bench_function("05_cafeteria_2", |b| {
        b.iter(|| {
            cafeteria::part_2(input);
        })
    });
}

criterion_group!(benches, bench_lobby);
criterion_main!(benches);
