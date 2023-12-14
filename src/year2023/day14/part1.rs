pub fn exec(input: &str) -> String {
  process(input).to_string()
}

struct Puzzle {
  reflector: Reflector,
}

impl From<&str> for Puzzle {
  fn from(value: &str) -> Self {
    let reflector = value.trim().into();

    Self { reflector }
  }
}

struct Reflector {
  map: Vec<Vec<char>>,
}

impl From<&str> for Reflector {
  fn from(value: &str) -> Self {
    let map = value
      .lines()
      .map(|line| line.trim().chars().collect())
      .collect();

    Reflector { map }
  }
}

impl Reflector {
  fn tilt_north(&mut self) {
    // println!("");
    // println!("Start Tilt North");
    let map = &mut self.map;
    for row in 0..map.len() {
      for col in 0..map[row].len() {
        if map[row][col] == 'O' {
          // println!("  0 found in {row}-{col}");
          let mut northest_able = row;
          while northest_able > 0 {
            if let 'O' | '#' = map[northest_able - 1][col] {
              break;
            }
            northest_able -= 1;
          }
          if northest_able != row {
            map[row][col] = '.';
            map[northest_able][col] = 'O';
          }
        }
      }
    }
  }

  fn eval_load(&self) -> usize {
    self
      .map
      .iter()
      .map(|line| line.iter().filter(|&c| c == &'O').count())
      .enumerate()
      .map(|(index, count)| (self.map.len() - index) * count)
      .sum()
  }
}

fn process(input: &str) -> usize {
  let mut puzzle: Puzzle = input.into();
  // println!("Reflector: ");
  // for row in puzzle.reflector.map.iter() {
  //   println!("      {row:?}")
  // }
  // println!("");
  puzzle.reflector.tilt_north();
  // println!("Reflector Tiltend: ");
  // for row in puzzle.reflector.map.iter() {
  //   println!("      {row:?}")
  // }
  puzzle.reflector.eval_load()
}

#[cfg(test)]
mod test_day14_part01_example {
  use super::*;

  #[test]
  pub fn example() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(process(input), 136);
  }
}
