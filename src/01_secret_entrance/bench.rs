use criterion::{criterion_group, criterion_main, Criterion};
use aoc::secret_entrance;

fn bench_secret_entrance(c: &mut Criterion) {
    let input = include_str!("input");

    c.bench_function("01_part1", |b| {
        b.iter(|| {
            secret_entrance::part_1(input);
        })
    });

    c.bench_function("01_part2", |b| {
        b.iter(|| {
            secret_entrance::part_2(input);
        })
    });
}

criterion_group!(benches, bench_secret_entrance);
criterion_main!(benches);
