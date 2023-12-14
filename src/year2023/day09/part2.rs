pub fn exec(input: &str) -> String {
  process(input).to_string()
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
  pub fn previous_from_vec(v: &Vec<isize>) -> Vec<isize> {
    let mut new_v: Vec<isize> = v.clone();
    let mut diffs: Vec<isize> = vec![];

    for (index, value) in v.iter().enumerate().skip(1) {
      diffs.push(value - v[index.saturating_sub(1)]);
    }

    if diffs.iter().all(|value| value == &0) {
      new_v.insert(0, new_v[new_v.len().saturating_sub(1)]);
    } else {
      let new_diffs = History::previous_from_vec(&diffs);
      let new_value = new_v[0] - new_diffs[0];
      new_v.insert(0, new_value);
    }
    new_v
  }

  pub fn previous(&mut self) {
    self.numbers = History::previous_from_vec(&self.numbers);
  }
}

#[derive(Debug)]
struct Game {
  histories: Vec<History>,
}

impl Game {
  pub fn previous(&mut self) {
    for history in self.histories.iter_mut() {
      history.previous();
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
  game.previous();
  game
    .histories
    .iter()
    .map(|history| history.numbers[0])
    .sum()
}

#[cfg(test)]
mod test_day09_part02_example {
  use super::*;

  #[test]
  pub fn example() {
    let input = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
    // -3
    // 0
    // 6
    assert_eq!(process(input), 2);
  }
}
