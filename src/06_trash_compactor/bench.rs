use aoc::trash_compactor;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_lobby(c: &mut Criterion) {
    let input = include_str!("input");

    c.bench_function("06_trash_compactor_1", |b| {
        b.iter(|| {
            trash_compactor::part_1(input);
        })
    });

    c.bench_function("06_trash_compactor_2", |b| {
        b.iter(|| {
            trash_compactor::part_2(input);
        })
    });
}

criterion_group!(benches, bench_lobby);
criterion_main!(benches);
