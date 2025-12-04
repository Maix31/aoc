use aoc::lobby;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_lobby(c: &mut Criterion) {
    let input = include_str!("input");

    c.bench_function("03_lobby_part1", |b| {
        b.iter(|| {
            lobby::part_1(input);
        })
    });

    c.bench_function("03_lobby_part2", |b| {
        b.iter(|| {
            lobby::part_2(input);
        })
    });
}

criterion_group!(benches, bench_lobby);
criterion_main!(benches);
