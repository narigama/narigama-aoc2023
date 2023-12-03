use criterion::{black_box, Criterion, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = narigama_aoc2023::util::get_input(2023, 1).unwrap();

    c.bench_function("y2023d01p01", |b| {
        b.iter(|| narigama_aoc2023::year2023::day01::part_one(black_box(&input)))
    });

    c.bench_function("y2023d01p02", |b| {
        b.iter(|| narigama_aoc2023::year2023::day01::part_two(black_box(&input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
