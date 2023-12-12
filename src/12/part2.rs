pub fn exec() -> () {
  let input = include_str!("input.txt");
  let output = process(input);
  println!("Day 12 - Part 02 - {output}");
}

#[derive(Debug, Clone)]
struct Line {
  record: String,
  groups: Vec<usize>,
}

impl From<&str> for Line {
  fn from(value: &str) -> Self {
    let (records, groups) = value.trim().split_once(' ').unwrap();

    let record = records.trim().to_owned();

    let groups = groups
      .trim()
      .split(',')
      .map(|c| c.parse::<usize>().unwrap())
      .collect();

    Self { record, groups }
  }
}

fn unfold_line(line: Line, factor: usize) -> Line {
  let mut new_records = String::new();
  let mut new_groups = vec![];

  for _ in 0..factor {
    new_records.push_str(&line.record);
    new_records.push('?');
    new_groups.extend(&line.groups);
  }
  new_records.remove(new_records.len() - 1);
  new_records.push('.');

  Line {
    record: new_records,
    groups: new_groups,
  }
}

#[derive(Debug)]
struct Game {
  lines: Vec<Line>,
}

impl From<&str> for Game {
  fn from(value: &str) -> Self {
    let lines = value
      .trim()
      .lines()
      .map(|str| Line::from(str))
      .map(|line| unfold_line(line, 5))
      .collect::<Vec<Line>>();

    Self { lines }
  }
}

fn solve_patter_with_memo(
  index: usize,
  group_index: usize,
  pattern: &str,
  groups: &Vec<usize>,
  memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
  // If we reach the end of groups, check if the remaining pattern is all '.' or '?'
  if group_index == groups.len() {
    if (index..pattern.len()).any(|j| pattern.chars().nth(j).unwrap() == '#') {
      return 0;
    } else {
      return 1;
    }
  }

  // If we reach the end of pattern, return 0
  if index == pattern.len() {
    return 0;
  }

  if let Some(ans) = memo[index][group_index] {
    return ans;
  }

  let mut ans = 0;

  // Try to fit the group at the next index
  if pattern.chars().nth(index).unwrap() != '#' {
    ans += solve_patter_with_memo(index + 1, group_index, pattern, groups, memo);
  }

  // Try to fit the group at the current index
  if index + groups[group_index] < pattern.len()
    && (index..index + groups[group_index]).all(|j| pattern.chars().nth(j).unwrap() != '.')
    && pattern.chars().nth(index + groups[group_index]).unwrap() != '#'
  {
    ans += solve_patter_with_memo(
      index + groups[group_index] + 1,
      group_index + 1,
      pattern,
      groups,
      memo,
    );
  }

  memo[index][group_index] = Some(ans);
  ans
}

fn process(input: &str) -> usize {
  Game::from(input)
    .lines
    .into_iter()
    .map(|line| {
      let mut memo = vec![vec![None; line.groups.len()]; line.record.len()];
      solve_patter_with_memo(0, 0, &line.record, &line.groups, &mut memo)
    })
    .sum()
}

#[cfg(test)]
mod test_day12_part02_example {
  use super::*;

  #[test]
  pub fn example() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    assert_eq!(process(input), 525152);
  }
}
