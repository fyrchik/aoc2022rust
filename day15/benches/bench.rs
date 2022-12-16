use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day15::{part1, part2};

pub fn part1_bench(c: &mut Criterion) {
    let input = include_str!("../input");
    c.bench_function("part 1", |b| {
        b.iter(|| part1(black_box(input), black_box(2_000_000)))
    });
}

pub fn part2_bench(c: &mut Criterion) {
    let input = include_str!("../input");
    c.bench_function("part 2", |b| {
        b.iter(|| part2(black_box(input), black_box(4_000_000)))
    });
}

criterion_group!(benches, part1_bench, part2_bench);
criterion_main!(benches);
