extern crate aoc_rust;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

struct Benchmark {
  year: u16,
  day: u8,
  input: &'static str,
  part1: fn() -> fn(&str) -> String,
  part2: fn() -> fn(&str) -> String,
}

macro_rules! benchmark {
  ($year:tt, $day:tt) => {
    Benchmark {
      year: stringify!($year)
        .split_once("year")
        .unwrap()
        .1
        .parse::<u16>()
        .unwrap(),
      day: stringify!($day)
        .split_once("day")
        .unwrap()
        .1
        .parse::<u8>()
        .unwrap(),
      input: include_str!(concat![
        "../src/",
        stringify!($year),
        "/",
        stringify!($day),
        "/input.txt"
      ]),
      part1: || {
        use aoc_rust::$year::$day::part1::exec;
        exec
      },
      part2: || {
        use aoc_rust::$year::$day::part2::exec;
        exec
      },
    }
  };
}

fn year2023() -> Vec<Benchmark> {
  vec![
    benchmark!(year2023, day01),
    benchmark!(year2023, day02),
    benchmark!(year2023, day03),
    benchmark!(year2023, day04),
    benchmark!(year2023, day05),
    benchmark!(year2023, day06),
    benchmark!(year2023, day07),
    benchmark!(year2023, day08),
    benchmark!(year2023, day09),
    benchmark!(year2023, day10),
    benchmark!(year2023, day11),
    benchmark!(year2023, day12),
    benchmark!(year2023, day13),
    benchmark!(year2023, day14),
    benchmark!(year2023, day15),
  ]
}

fn process_benchark(b: Benchmark, c: &mut Criterion) {
  let mut group = c.benchmark_group(format!("{}-{}", b.year, b.day));
  group.bench_function(&format!("{}-part1", b.day), |bencher| {
    bencher.iter(|| (b.part1)()(black_box(b.input)))
  });
  group.bench_function(&format!("{}-part2", b.day), |bencher| {
    bencher.iter(|| (b.part2)()(black_box(b.input)))
  });
}

fn bench_all(c: &mut Criterion) {
  let benches = std::iter::empty()
    .chain(year2023())
    .collect::<Vec<Benchmark>>();

  for benchmark in benches {
    process_benchark(benchmark, c);
  }
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
