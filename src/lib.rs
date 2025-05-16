pub mod utils {
  pub mod ansi;
}
macro_rules! module {
    ($year:ident, [$( $day:ident ),*]) => {
        pub mod $year {
            $(
                pub mod $day {
                    pub mod part1;
                    pub mod part2;
                }
            )*
        }
    };
}

module!(
  year2023,
  [
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16
  ]
);
