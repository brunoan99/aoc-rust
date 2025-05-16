use aoc_rust::*;
use utils::ansi::*;

fn main() {
  let (year, day) = match std::env::args().nth(1) {
    Some(arg) => {
      let str = arg.as_str();
      match str.split_once("-") {
        Some((year_str, day_str)) => (year_str.parse::<u16>().ok(), day_str.parse::<u8>().ok()),
        _ => (None, None),
      }
    }
    None => (None, None),
  };

  let solutions: Vec<Solution> = std::iter::empty()
    .chain(year2023())
    .filter(|solution| year == Some(solution.year) || year.is_none())
    .filter(|solution| day == Some(solution.day) || day.is_none())
    .collect();

  for Solution {
    year,
    day,
    input,
    part1,
    part2,
  } in solutions
  {
    println!(
      "Year {BOLD}{YELLOW}{}{RESET} - Day {BOLD}{YELLOW}{}{RESET} - Part {BOLD}{YELLOW}01{RESET}  -  {BOLD}{GREEN}{} {RESET}",
      year,
      format!("{:02}", day),
      part1(input)
    );
    println!(
      "Year {BOLD}{YELLOW}{}{RESET} - Day {BOLD}{YELLOW}{}{RESET} - Part {BOLD}{YELLOW}02{RESET}  -  {BOLD}{GREEN}{} {RESET}",
      year,
      format!("{:02}", day),
      part2(input)
    );
    println!("")
  }
}

struct Solution {
  year: u16,
  day: u8,
  input: &'static str,
  part1: fn(&str) -> String,
  part2: fn(&str) -> String,
}

macro_rules! solution {
  ($year:tt, $day:tt) => {
    Solution {
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
        "../aoc-inputs/",
        stringify!($year),
        "/",
        stringify!($day),
        "_input.txt"
      ]),
      part1: |input: &str| {
        use $year::$day::part1::exec;
        exec(input)
      },
      part2: |input: &str| {
        use $year::$day::part2::exec;
        exec(input)
      },
    }
  };
}

fn year2023() -> Vec<Solution> {
  vec![
    solution!(year2023, day01),
    solution!(year2023, day02),
    solution!(year2023, day03),
    solution!(year2023, day04),
    solution!(year2023, day05),
    solution!(year2023, day06),
    solution!(year2023, day07),
    solution!(year2023, day08),
    solution!(year2023, day09),
    solution!(year2023, day10),
    solution!(year2023, day11),
    solution!(year2023, day12),
    solution!(year2023, day13),
    solution!(year2023, day14),
    solution!(year2023, day15),
    solution!(year2023, day16),
  ]
}
