use std::str::FromStr;

pub fn exec(input: &str) -> String {
  process(input).to_string()
}

struct Scratchcard {
  _id: usize,
  win: Vec<usize>,
  hand: Vec<usize>,
}

impl FromStr for Scratchcard {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let line = s.trim();
    let (metadata, game_line) = line.split_once(':').ok_or("Invalid Game")?;
    let (_, id) = metadata.split_once(' ').ok_or("Invalid game id")?;
    let (win_line, hand_line) = game_line.split_once('|').ok_or("Invalid Game Line")?;
    let win = win_line
      .split(' ')
      .filter(|&item| item != "")
      .map(|item| item.trim().parse::<usize>().unwrap())
      .collect::<Vec<_>>();
    let hand = hand_line
      .split(' ')
      .filter(|&item| item != "")
      .map(|item| item.trim().parse::<usize>().unwrap())
      .collect::<Vec<_>>();
    Ok(Scratchcard {
      _id: id.trim().parse().map_err(|_| "Game Id not a number")?,
      win,
      hand,
    })
  }
}

fn check_win(win: &Vec<usize>, hand: &Vec<usize>) -> usize {
  let mut wins = 0;
  'outer: for win_number in win {
    for hand_number in hand {
      if win_number == hand_number {
        wins += 1;
        continue 'outer;
      }
    }
  }
  wins
}

fn calc_points(wins: usize) -> usize {
  if wins == 0 {
    0
  } else {
    2_usize.pow(wins.saturating_sub(1).try_into().unwrap())
  }
}

fn process(input: &str) -> usize {
  input
    .lines()
    .map(|line| Scratchcard::from_str(line).unwrap())
    .map(|game| check_win(&game.win, &game.hand))
    .map(|wins| calc_points(wins))
    .sum::<usize>()
}

#[cfg(test)]
mod testsday04part01 {
  use super::*;

  #[test]
  pub fn example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(process(input), 13)
  }
}
