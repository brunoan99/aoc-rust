use std::cmp::min;

pub fn exec(input: &str) -> String {
  process(input).to_string()
}

fn get_adjacents(matrix: &Vec<Vec<char>>, row: usize, col: usize, len: usize) -> Vec<char> {
  let mut chars: Vec<char> = vec![];
  let row_start = row.saturating_sub(1);
  let row_end = min(matrix.len(), row + 2);
  let col = if col == 0 { matrix[row].len() } else { col };
  let col_start = col.saturating_sub(len + 1);
  let col_end = min(matrix[row].len(), col + 1);
  for row in row_start..row_end {
    for col in col_start..col_end {
      chars.push(matrix[row][col]);
    }
  }
  chars
}

fn contain_symbols(chars: &Vec<char>) -> bool {
  chars.iter().any(|item| {
    item == &'@'
      || item == &'#'
      || item == &'$'
      || item == &'%'
      || item == &'&'
      || item == &'*'
      || item == &'/'
      || item == &'-'
      || item == &'+'
      || item == &'='
  })
}

fn process(input: &str) -> usize {
  let matrix = input
    .lines()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();
  let mut numbers: Vec<usize> = vec![];
  let mut number_buffer = String::new();
  for row in 0..(matrix.len()) {
    for col in 0..(matrix[row].len()) {
      if matrix[row][col].is_digit(10) {
        number_buffer.push(matrix[row][col]);
      } else if number_buffer.is_empty() {
        continue;
      } else {
        let adjacents = get_adjacents(&matrix, row, col, number_buffer.len());
        let contain = contain_symbols(&adjacents);
        if contain {
          let number = number_buffer.parse::<usize>().unwrap();
          numbers.push(number);
        }
        number_buffer = String::new();
      }
    }
  }
  numbers.iter().sum()
}

#[cfg(test)]
mod testsday03part01 {
  use super::*;

  #[test]
  pub fn example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(process(input), 4361)
  }
}
