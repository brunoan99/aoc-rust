use std::cmp::min;

pub fn exec(input: &str) -> String {
  process(input).to_string()
}

fn get_full_number(row: &Vec<char>, pos: usize) -> usize {
  let mut buffer = String::new();
  let mut lpos: usize = pos.saturating_sub(1);
  let mut rpos: usize = pos;
  while rpos < row.len() && row[rpos].is_ascii_digit() {
    buffer.push(row[rpos]);
    rpos += 1;
  }
  while row[lpos].is_ascii_digit() {
    let prev_buff = buffer;
    buffer = String::from(row[lpos]);
    buffer.push_str(&prev_buff);
    if lpos == 0 {
      break;
    }
    lpos = lpos.saturating_sub(1);
  }
  buffer.parse::<usize>().unwrap()
}

fn get_numbers(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<usize> {
  let mut numbers: Vec<usize> = vec![];
  let row_start = row.saturating_sub(1);
  let row_end = min(matrix.len(), row + 2);
  let col = if col == 0 { matrix[row].len() } else { col };
  let col_start = col.saturating_sub(1);
  let col_end = min(matrix[row].len(), col + 2);
  for row in row_start..row_end {
    for col in col_start..col_end {
      if matrix[row][col].is_ascii_digit() {
        if col == matrix[row].len() - 1
          || col == col_end - 1
          || !matrix[row][col + 1].is_ascii_digit()
        {
          let n = get_full_number(&matrix[row], col);
          numbers.push(n);
        }
      }
    }
  }
  numbers
}

fn process(input: &str) -> usize {
  let matrix: Vec<Vec<char>> = input
    .lines()
    .map(|line| line.chars().collect::<_>())
    .collect::<_>();
  let mut gears: Vec<usize> = vec![];
  for row in 0..(matrix.len()) {
    for col in 0..(matrix[row].len()) {
      if matrix[row][col] == '*' {
        let numbers = get_numbers(&matrix, row, col);
        if numbers.len() == 2 {
          gears.push(numbers[0] * numbers[1]);
        }
      }
    }
  }
  gears.iter().sum()
}

#[cfg(test)]
mod testsday03part02 {
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
    assert_eq!(process(input), 467835)
  }
}
