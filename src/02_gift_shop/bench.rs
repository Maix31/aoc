use aoc::gift_shop;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_gift_shop(c: &mut Criterion) {
    let input = include_str!("input");

    c.bench_function("02_gift_shop_part1", |b| {
        b.iter(|| {
            gift_shop::part_1(input);
        })
    });

    c.bench_function("02_gift_shop_part2", |b| {
        b.iter(|| {
            gift_shop::part_2(input);
        })
    });
}

criterion_group!(benches, bench_gift_shop);
criterion_main!(benches);
