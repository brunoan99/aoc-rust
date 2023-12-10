pub fn exec() -> () {
  let input = include_str!("input.txt");
  let output = process(input);
  println!("Day 09 - Part 01 - {output}");
}

#[derive(Debug)]
struct History {
  numbers: Vec<isize>,
}

impl From<&str> for History {
  fn from(value: &str) -> Self {
    let numbers = value
      .trim()
      .split(' ')
      .map(|value| value.parse().unwrap())
      .collect();
    History { numbers }
  }
}

impl History {
  pub fn next_from_vec(v: &Vec<isize>) -> Vec<isize> {
    let mut new_v: Vec<isize> = v.clone();
    let mut diffs: Vec<isize> = vec![];

    for (index, value) in v.iter().enumerate().skip(1) {
      diffs.push(value - v[index.saturating_sub(1)]);
    }

    if diffs.iter().all(|value| value == &0) {
      new_v.push(new_v[new_v.len().saturating_sub(1)]);
    } else {
      let new_diffs = History::next_from_vec(&diffs);
      let new_value =
        new_v[new_v.len().saturating_sub(1)] + new_diffs[new_diffs.len().saturating_sub(1)];
      new_v.push(new_value);
    }
    new_v
  }

  pub fn next(&mut self) {
    self.numbers = History::next_from_vec(&self.numbers);
  }
}

#[derive(Debug)]
struct Game {
  histories: Vec<History>,
}

impl Game {
  pub fn next(&mut self) {
    for history in self.histories.iter_mut() {
      history.next();
    }
  }
}

impl From<&str> for Game {
  fn from(value: &str) -> Self {
    let histories = value
      .trim()
      .lines()
      .map(|line| History::from(line))
      .collect();
    Game { histories }
  }
}

fn process(input: &str) -> isize {
  let mut game = Game::from(input);
  game.next();
  game
    .histories
    .iter()
    .map(|history| history.numbers.iter().last().unwrap().clone())
    .sum()
}

#[cfg(test)]
mod test_day09_part01_example {
  use super::*;

  #[test]
  pub fn example() {
    let input = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
    assert_eq!(process(input), 114);
  }
}
