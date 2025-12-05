use aoc::printing_department;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_lobby(c: &mut Criterion) {
    let input = include_str!("input");

    c.bench_function("04_printing_department_1", |b| {
        b.iter(|| {
            printing_department::part_1(input);
        })
    });

    c.bench_function("04_printing_department2", |b| {
        b.iter(|| {
            printing_department::part_2(input);
        })
    });
}

criterion_group!(benches, bench_lobby);
criterion_main!(benches);
