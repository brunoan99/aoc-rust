pub fn exec() -> () {
  let input = include_str!("input.txt");
  let output = process(input);
  println!("Day 11 - Part 02 - {output}");
}

#[derive(Debug, Clone, Copy)]
struct GalaxyLocation {
  row: usize,
  col: usize,
}

#[derive(Debug)]
struct UniverseImage {
  galaxies: Vec<GalaxyLocation>,
}

impl From<&str> for UniverseImage {
  fn from(value: &str) -> Self {
    let mut map: Vec<Vec<char>> = value
      .trim()
      .lines()
      .map(|line| line.chars().collect())
      .collect();

    let empty_rows: Vec<usize> = map
      .iter()
      .enumerate()
      .filter(|(_, line)| line.iter().all(|char| char == &'.'))
      .map(|(index, _)| index)
      .collect();

    for (count, row_index) in empty_rows.iter().enumerate() {
      map.insert(row_index + count, vec!['.'; map[0].len()])
    }

    let mut empty_cols: Vec<usize> = vec![];

    for col in 0..map[0].len() {
      let mut all_empty = true;
      for row in 0..map.len() {
        if map[row][col] == '#' {
          all_empty = false;
          break;
        }
      }
      if all_empty {
        empty_cols.push(col);
      }
    }

    for (count, col_index) in empty_cols.iter().enumerate() {
      for row in 0..map.len() {
        map[row].insert(col_index + count, '.');
      }
    }
    let mut galaxies: Vec<GalaxyLocation> = vec![];

    for row in 0..map.len() {
      for col in 0..map[row].len() {
        if map[row][col] == '#' {
          galaxies.push(GalaxyLocation { row, col })
        }
      }
    }

    UniverseImage { galaxies }
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

fn process(input: &str) -> usize {
  let universe_image = UniverseImage::from(input);
  let pairs = generate_pairs(universe_image.galaxies);
  pairs.into_iter().map(|(a, b)| calc_dist(a, b)).sum()
}

#[cfg(test)]
mod test_day11_part01_example {
  use super::*;

  #[test]
  pub fn example() {
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
    assert_eq!(process(input), 374);
  }
}
