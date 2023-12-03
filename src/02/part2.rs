pub fn exec() -> () {
  let input = include_str!("input1.txt");
  let output = process(input);
  println!("Day 02 - Part 0 - {output}");
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

fn get_max_of_rounds(line: &str) -> GameRound {
  let mut game_round = GameRound::new();
  for round in line.split(";") {
    for hand in round.split(",") {
      let hand_info: Vec<&str> = hand.trim().split_whitespace().collect();
      let amount = hand_info[0].parse::<usize>().unwrap();
      let color = hand_info[1];
      match color {
        "red" => {
          if game_round.red < amount {
            game_round.red = amount
          }
        }
        "green" => {
          if game_round.green < amount {
            game_round.green = amount
          }
        }
        "blue" => {
          if game_round.blue < amount {
            game_round.blue = amount
          }
        }
        _ => panic!("unknow color: {color}"),
      }
    }
  }
  game_round
}

fn process(input: &str) -> usize {
  input
    .lines()
    .map(|line| {
      let line = line.trim();
      let metadata_position = line.find(":").unwrap();
      let game_line = &line[(metadata_position + 1)..];
      let round = get_max_of_rounds(game_line);
      round.red * round.green * round.blue
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
    assert_eq!(process(input), 2286)
  }
}
