pub fn exec(input: &str) -> String {
  process(input).to_string()
}

#[derive(Debug)]
struct Puzzle {
  hashs: Vec<Vec<char>>,
}

impl From<&str> for Puzzle {
  fn from(value: &str) -> Self {
    let hashs = value
      .trim()
      .split(',')
      .map(|line| line.trim().chars().collect())
      .collect();
    Self { hashs }
  }
}

#[derive(Debug)]
struct PuzzleASCII {
  hashs: Vec<Vec<usize>>,
}

impl From<Puzzle> for PuzzleASCII {
  fn from(value: Puzzle) -> Self {
    let hashs = value
      .hashs
      .into_iter()
      .map(|line| {
        line
          .into_iter()
          .map(|char| char as usize)
          .collect::<Vec<usize>>()
      })
      .collect::<Vec<Vec<usize>>>();
    Self { hashs }
  }
}

impl PuzzleASCII {
  fn solve(self) -> usize {
    self
      .hashs
      .into_iter()
      .map(|hash| hash.into_iter().fold(0, |acc, v| ((acc + v) * 17) % 256))
      .sum()
  }
}

fn process(input: &str) -> usize {
  let puzzle = Puzzle::from(input);
  let puzzle_ascii = PuzzleASCII::from(puzzle);
  puzzle_ascii.solve()
}

#[cfg(test)]
mod test_day15_part01_example {
  use super::*;

  #[test]
  pub fn example_1() {
    let input = "HASH";
    assert_eq!(process(input), 52);
  }

  #[test]
  pub fn example_2() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(process(input), 1320);
  }
}
