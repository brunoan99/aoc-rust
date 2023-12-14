pub fn exec(input: &str) -> String {
  process(input, 1_000_000).to_string()
}

#[derive(Debug, Clone, Copy)]
struct GalaxyLocation {
  row: usize,
  col: usize,
}

#[derive(Debug)]
struct UniverseImage {
  map: Vec<Vec<char>>,
  galaxies: Vec<GalaxyLocation>,
}

impl From<&str> for UniverseImage {
  fn from(value: &str) -> Self {
    let map: Vec<Vec<char>> = value
      .trim()
      .lines()
      .map(|line| line.chars().collect())
      .collect();
    let mut galaxies: Vec<GalaxyLocation> = vec![];

    for row in 0..map.len() {
      for col in 0..map[row].len() {
        if map[row][col] == '#' {
          galaxies.push(GalaxyLocation { row, col })
        }
      }
    }

    UniverseImage { map, galaxies }
  }
}

impl UniverseImage {
  fn expand(&mut self, fact: usize) {
    let map = &self.map;
    let factor = std::cmp::max(1, fact - 1);
    let mut empty_rows: Vec<bool> = vec![false; map.len()];
    let mut empty_cols: Vec<bool> = vec![false; map[0].len()];

    for row in 0..map.len() {
      for col in 0..map[row].len() {
        if map[row][col] == '#' {
          empty_rows[row] = true;
          empty_cols[col] = true;
        }
      }
    }
    let empty_rows = empty_rows
      .iter()
      .enumerate()
      .filter(|(_, &item)| !item)
      .map(|(index, _)| index)
      .collect::<Vec<usize>>();
    let empty_cols = empty_cols
      .iter()
      .enumerate()
      .filter(|(_, &item)| !item)
      .map(|(index, _)| index)
      .collect::<Vec<usize>>();

    for galaxy in self.galaxies.iter_mut() {
      let greater_rows = empty_rows.iter().filter(|row| &galaxy.row > row).count();
      let greater_cols = empty_cols.iter().filter(|col| &galaxy.col > col).count();
      galaxy.row = galaxy.row + greater_rows * factor;
      galaxy.col = galaxy.col + greater_cols * factor;
    }
  }
}

fn generate_pairs(galaxies: Vec<GalaxyLocation>) -> Vec<(GalaxyLocation, GalaxyLocation)> {
  let mut pairs = vec![];
  for a in 0..galaxies.len() {
    for b in (a + 1)..galaxies.len() {
      pairs.push((galaxies[a], galaxies[b]))
    }
  }
  pairs
}

fn calc_dist(a: GalaxyLocation, b: GalaxyLocation) -> usize {
  a.row.abs_diff(b.row) + a.col.abs_diff(b.col)
}

fn process(input: &str, expand: usize) -> usize {
  let mut universe_image = UniverseImage::from(input);
  universe_image.expand(expand);
  let pairs = generate_pairs(universe_image.galaxies);
  pairs.into_iter().map(|(a, b)| calc_dist(a, b)).sum()
}

#[cfg(test)]
mod test_day11_part02_example {
  use super::*;

  #[test]
  pub fn example_01() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(process(input, 1), 374);
  }

  #[test]
  pub fn example_02() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(process(input, 10), 1030);
  }

  #[test]
  pub fn example_03() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(process(input, 100), 8410);
  }
}
