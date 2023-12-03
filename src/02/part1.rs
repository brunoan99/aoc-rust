pub fn exec() -> () {
  let input = include_str!("input1.txt");
  let output = process(input);
  println!("Day 02 - Part 01 - {output}");
}

#[derive(Debug)]
struct GameRound {
  blue: usize,
  green: usize,
  red: usize,
}

impl GameRound {
  pub const fn new() -> Self {
    Self {
      blue: 0,
      green: 0,
      red: 0,
    }
  }
}

fn get_rounds(line: &str) -> Vec<GameRound> {
  let mut rounds: Vec<GameRound> = vec![];
  for round in line.split(";") {
    let mut game_round = GameRound::new();
    for hand in round.split(",") {
      let hand_info: Vec<&str> = hand.trim().split_whitespace().collect();
      let amount = hand_info[0].parse::<usize>().unwrap();
      let color = hand_info[1];
      match color {
        "red" => game_round.red = amount,
        "green" => game_round.green = amount,
        "blue" => game_round.blue = amount,
        _ => panic!("unknow color: {color}"),
      }
    }
    rounds.push(game_round);
  }
  rounds
}

fn valid_rounds(rounds: Vec<GameRound>) -> bool {
  rounds
    .iter()
    .all(|round| !(round.red > 12 || round.green > 13 || round.blue > 14))
}

fn process(input: &str) -> usize {
  input
    .lines()
    .filter_map(|line| {
      let line = line.trim();
      let metadata_position = line.find(":").unwrap();
      let game_line = &line[(metadata_position + 1)..];
      let rounds = get_rounds(game_line);
      let valid = valid_rounds(rounds);
      if valid {
        let id = (&line[5..metadata_position]).parse::<usize>().unwrap();
        Some(id)
      } else {
        None
      }
    })
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
