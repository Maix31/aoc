use criterion::{criterion_group, criterion_main, Criterion};
use aoc::movie_theater;

fn bench_lobby(c: &mut Criterion) {
    let input = include_str!("input");

    c.bench_function("09_movie_theater_1", |b| {
        b.iter(|| {
            movie_theater::part_1(input);
        })
    });

    c.bench_function("09_movie_theater_2", |b| {
        b.iter(|| {
            movie_theater::part_2(input);
        })
    });
}

criterion_group!(benches, bench_lobby);
criterion_main!(benches);
