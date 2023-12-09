pub fn exec() -> () {
  let input = include_str!("input1.txt");
  let output = process(input);
  println!("Day 06 - Part 02 - {output}");
}

#[derive(Debug)]
struct Competition {
  race: Race,
}

impl From<&str> for Competition {
  fn from(value: &str) -> Self {
    let lines: Vec<&str> = value.trim().lines().collect();

    let time = lines[0]
      .split_once(':')
      .unwrap()
      .1
      .split(' ')
      .filter(|&item| item != "")
      .fold(String::new(), |acc, item| acc + item)
      .parse::<usize>()
      .unwrap();

    let distance = lines[1]
      .split_once(':')
      .unwrap()
      .1
      .split(' ')
      .filter(|&item| item != "")
      .fold(String::new(), |acc, item| acc + item)
      .parse::<usize>()
      .unwrap();

    Competition {
      race: Race { time, distance },
    }
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

fn calc_possibilities(race: &Race) -> usize {
  let time_is_even = race.time % 2 == 0;
  let max_dist_point = race.time.saturating_div(2);
  let mut from_bottom;
  let mut count = 0;

  loop {
    from_bottom = simulate_race(count, race.time) > race.distance;
    let from_top = simulate_race(max_dist_point - count, race.time) < race.distance;
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

pub fn process(input: &str) -> usize {
  calc_possibilities(&Competition::from(input).race)
}

#[cfg(test)]
mod test_day06_part02_example {
  use super::*;

  #[test]
  pub fn example() {
    let input = "Time:      7  15   30
    Distance:  9  40  200";
    assert_eq!(process(input), 71503);
  }
}
