pub fn exec() -> () {
  let input = include_str!("input.txt");
  let output = process(input);
  println!("Day 13 - Part 02 - {output}");
}

#[derive(Debug)]
struct Mirror {
  map: Vec<Vec<char>>,
}

impl From<Vec<&str>> for Mirror {
  fn from(value: Vec<&str>) -> Self {
    let map = value
      .into_iter()
      .map(|line| line.chars().collect())
      .collect();
    Self { map }
  }
}

fn compare_diff_rows(matrix: &Vec<Vec<char>>, ra: usize, rb: usize) -> usize {
  let mut diff = 0;
  for col in 0..matrix[0].len() {
    if matrix[ra][col] != matrix[rb][col] {
      diff += 1;
    }
  }
  diff
}

fn compare_diff_cols(matrix: &Vec<Vec<char>>, ca: usize, cb: usize) -> usize {
  let mut diff = 0;
  for row in 0..matrix.len() {
    if matrix[row][ca] != matrix[row][cb] {
      diff += 1;
    }
  }
  diff
}

impl Mirror {
  pub fn find_reflection_on_row(&self) -> Option<usize> {
    let map = &self.map;
    'outer: for row in 1..map.len() {
      let mut diff = compare_diff_rows(&map, row - 1, row);
      if diff <= 1 {
        let mut dist = 1;
        while row + dist < map.len() && row - dist > 0 {
          diff += compare_diff_rows(&map, row - 1 - dist, row + dist);
          if diff > 1 {
            continue 'outer;
          }
          dist += 1;
        }
        if diff == 1 {
          return Some(row);
        }
      }
    }

    None
  }

  pub fn find_reflection_on_col(&self) -> Option<usize> {
    let map = &self.map;
    'outer: for col in 1..map[0].len() {
      let mut diff = compare_diff_cols(&map, col - 1, col);
      if diff <= 1 {
        let mut dist = 1;
        while col + dist < map[0].len() && col - dist > 0 {
          diff += compare_diff_cols(&map, col - 1 - dist, col + dist);
          if diff > 1 {
            continue 'outer;
          }
          dist += 1;
        }
        if diff == 1 {
          return Some(col);
        }
      }
    }

    None
  }

  pub fn find_reflection(&self) -> Option<(usize, bool)> {
    // (position of reflection, found on row?)
    if let Some(row) = self.find_reflection_on_row() {
      return Some((row, true));
    }

    if let Some(col) = self.find_reflection_on_col() {
      return Some((col, false));
    }
    None
  }
}

#[derive(Debug)]
struct Puzzle {
  mirrors: Vec<Mirror>,
}

fn split_on_empty_lines(input: Vec<&str>) -> Vec<Vec<&str>> {
  let mut r = vec![];
  let mut block: Vec<&str> = vec![];

  for line in input.into_iter() {
    if line.is_empty() {
      r.push(block);
      block = vec![]
    } else {
      block.push(line);
    }
  }
  r.push(block);

  r
}

impl From<&str> for Puzzle {
  fn from(value: &str) -> Self {
    let lines: Vec<&str> = value.trim().lines().collect();
    let blocks: Vec<Vec<&str>> = split_on_empty_lines(lines);
    let mirrors: Vec<Mirror> = blocks
      .into_iter()
      .map(|block| Mirror::from(block))
      .collect();

    Self { mirrors }
  }
}

fn process(input: &str) -> usize {
  Puzzle::from(input)
    .mirrors
    .iter()
    .filter_map(|m| m.find_reflection())
    .map(|(p, r)| if r { p * 100 } else { p })
    .sum()
}

#[cfg(test)]
mod test_day13_part02_example {
  use super::*;

  #[test]
  pub fn example() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    assert_eq!(process(input), 400);
  }
}
