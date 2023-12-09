use std::iter::zip;

pub fn exec() -> () {
  let input = include_str!("input1.txt");
  let output = process(input);
  println!("Day 06 - Part 01 - {output}");
}

#[derive(Debug)]
struct Competition {
  races: Vec<Race>,
}

impl From<&str> for Competition {
  fn from(value: &str) -> Self {
    let lines: Vec<&str> = value.trim().lines().collect();

    let times: Vec<usize> = lines[0]
      .split_once(':')
      .unwrap()
      .1
      .split(' ')
      .filter_map(|item| item.parse::<usize>().ok())
      .collect();

    let distances: Vec<usize> = lines[1]
      .split_once(':')
      .unwrap()
      .1
      .split(' ')
      .filter_map(|item| item.parse::<usize>().ok())
      .collect();

    let races: Vec<Race> = zip(times, distances)
      .map(|(time, distance)| Race { time, distance })
      .collect();

    Competition { races }
  }
}

#[derive(Debug)]
struct Race {
  time: usize,
  distance: usize,
}

fn simulate_race(time_hold: usize, time_total: usize) -> usize {
  time_total * time_hold - time_hold * time_hold
}

#[cfg(test)]
mod test_day06_part01_simulate_race {
  use super::simulate_race;

  #[test]
  fn test_01() {
    assert_eq!(simulate_race(0, 7), 0);
    assert_eq!(simulate_race(1, 7), 6);
    assert_eq!(simulate_race(2, 7), 10);
    assert_eq!(simulate_race(3, 7), 12);
    assert_eq!(simulate_race(4, 7), 12);
    assert_eq!(simulate_race(5, 7), 10);
    assert_eq!(simulate_race(6, 7), 6);
    assert_eq!(simulate_race(7, 7), 0);
  }
}

fn calc_possibilities(race: &Race) -> usize {
  let time_is_even = race.time % 2 == 0;
  let max_dist_point = race.time.saturating_div(2);
  let mut from_bottom;
  let mut count = 0;

  loop {
    from_bottom = simulate_race(count, race.time) > race.distance;
    let from_top = simulate_race(max_dist_point - count, race.time) <= race.distance;
    if from_top || from_bottom {
      break;
    }

    count += 1
  }

  match (time_is_even, from_bottom) {
    (false, false) => 2 * count,
    (false, true) => 2 * (max_dist_point - count + 1),
    (true, false) => 2 * count - 1,
    (true, true) => 2 * (max_dist_point - count + 1) - 1,
  }
}

#[cfg(test)]
mod test_day06_part01_calc_possibilities {
  use super::{calc_possibilities, Race};

  #[test]
  #[rustfmt::skip]
  fn test_01() {
    let r1 = Race { time: 7, distance: 9 };
    let r2 = Race { time: 15, distance: 40 };
    let r3 = Race { time: 30, distance: 200 };
    assert_eq!(calc_possibilities(&r1), 4);
    assert_eq!(calc_possibilities(&r2), 8);
    assert_eq!(calc_possibilities(&r3), 9);
  }
}

pub fn process(input: &str) -> usize {
  Competition::from(input)
    .races
    .iter()
    .map(calc_possibilities)
    .fold(1, |acc, p| acc * p)
}

#[cfg(test)]
mod test_day06_part01_example {
  use super::*;

  #[test]
  pub fn example() {
    let input = "Time:      7  15   30
    Distance:  9  40  200";
    assert_eq!(process(input), 288);
  }
}
