use std::{iter::Sum, str::FromStr};

pub fn exec() -> () {
  let input = include_str!("input1.txt");
  let output = process(input);
  println!("Day 02 - Part 01 - {output}");
}

struct Game {
  id: usize,
  rounds: Vec<GameRound>,
}

impl FromStr for Game {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let line = s.trim();
    let (metadata, game_line) = line.split_once(':').ok_or("Invalid game")?;
    let (_, id) = metadata.split_once(' ').ok_or("Invalid game id")?;
    let rounds = game_line
      .split(';')
      .map(|round| GameRound::from_str(round))
      .collect::<Result<Vec<GameRound>, &'static str>>()?;
    Ok(Game {
      id: id.parse().map_err(|_| "Game Id not a number")?,
      rounds,
    })
  }
}

#[derive(Default)]
struct GameRound {
  red: usize,
  green: usize,
  blue: usize,
}

impl FromStr for GameRound {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(
      s.split(",")
        .filter_map(|hand| {
          let (amount, color) = hand.trim().split_once(' ')?;
          let amount = amount.parse::<usize>().ok()?;
          match color {
            "red" => Some(GameRound {
              red: amount,
              ..GameRound::default()
            }),
            "green" => Some(GameRound {
              green: amount,
              ..GameRound::default()
            }),
            "blue" => Some(GameRound {
              blue: amount,
              ..GameRound::default()
            }),
            _ => None,
          }
        })
        .sum(),
    )
  }
}

impl Sum for GameRound {
  fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
    iter.fold(GameRound::default(), |sum, game| GameRound {
      red: sum.red + game.red,
      green: sum.green + game.green,
      blue: sum.blue + game.blue,
    })
  }
}

fn process(input: &str) -> usize {
  input
    .lines()
    .filter_map(|line| Game::from_str(line).ok())
    .filter(|game| {
      game
        .rounds
        .iter()
        .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
    })
    .map(|game| game.id)
    .sum::<usize>()
}

#[cfg(test)]
mod testsday02part01 {
  use super::*;

  #[test]
  pub fn example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(process(input), 8)
  }
}
