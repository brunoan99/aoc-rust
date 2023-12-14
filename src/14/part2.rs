use std::collections::HashMap;

pub fn exec() -> () {
  let input = include_str!("input.txt");
  let output = process(input, 1000000000);
  println!("Day 14 - Part 02 - {output}");
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
    let map = &mut self.map;
    for row in 0..map.len() {
      for col in 0..map[row].len() {
        if map[row][col] == 'O' {
          let mut north = row;
          while north > 0 {
            if map[north - 1][col] == '.' {
              map[north - 1][col] = 'O';
              map[north][col] = '.';
              north -= 1;
            } else {
              break;
            }
          }
        }
      }
    }
  }

  fn titl_west(&mut self) {
    let map = &mut self.map;
    for row in 0..map.len() {
      for col in 0..map[row].len() {
        if map[row][col] == 'O' {
          let mut west = col;
          while west > 0 {
            if map[row][west - 1] == '.' {
              map[row][west - 1] = 'O';
              map[row][west] = '.';
              west -= 1;
            } else {
              break;
            }
          }
        }
      }
    }
  }

  fn titl_south(&mut self) {
    let map = &mut self.map;
    for row in (0..map.len()).rev() {
      for col in 0..map[row].len() {
        if map[row][col] == 'O' {
          let mut south = row;
          while south < map.len() - 1 {
            if map[south + 1][col] == '.' {
              map[south + 1][col] = 'O';
              map[south][col] = '.';
              south += 1;
            } else {
              break;
            }
          }
        }
      }
    }
  }

  fn titl_east(&mut self) {
    let map = &mut self.map;
    for row in 0..map.len() {
      for col in (0..map[row].len()).rev() {
        if map[row][col] == 'O' {
          let mut east = col;
          while east < map[row].len() - 1 {
            if map[row][east + 1] == '.' {
              map[row][east + 1] = 'O';
              map[row][east] = '.';
              east += 1;
            } else {
              break;
            }
          }
        }
      }
    }
  }

  fn cycle(&mut self) {
    self.tilt_north();
    self.titl_west();
    self.titl_south();
    self.titl_east();
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

fn process(input: &str, cycles: usize) -> usize {
  let mut reflector = Puzzle::from(input).reflector;
  let mut seen: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
  let mut hist = Vec::new();

  seen.insert(reflector.map.clone(), hist.len());
  hist.push(reflector.map.clone());

  let (start, end) = loop {
    reflector.cycle();

    if let Some(prev) = seen.get(&reflector.map) {
      break (*prev, hist.len());
    }

    seen.insert(reflector.map.clone(), hist.len());
    hist.push(reflector.map.clone())
  };

  let cycle_width = end - start;
  let offset = cycles - start;
  let remainder = offset % cycle_width;

  reflector.map = hist[start + remainder].clone();

  reflector.eval_load()
}

#[cfg(test)]
mod test_day14_part02_example {
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
    assert_eq!(process(input, 1000000000), 64);
  }
}
