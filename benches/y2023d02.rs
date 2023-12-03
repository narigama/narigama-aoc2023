use std::str::FromStr;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = narigama_aoc2023::util::get_input(2023, 2).unwrap();
    let data = input
        .lines()
        .map(narigama_aoc2023::year2023::day02::Game::from_str)
        .collect::<eyre::Result<Vec<_>>>()
        .unwrap();

    c.bench_function("y2023d02p01", |b| {
        b.iter(|| narigama_aoc2023::year2023::day02::part_one(black_box(&data)))
    });

    c.bench_function("y2023d02p02", |b| {
        b.iter(|| narigama_aoc2023::year2023::day02::part_two(black_box(&data)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
