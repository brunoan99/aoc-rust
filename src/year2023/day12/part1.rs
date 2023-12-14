pub fn exec(input: &str) -> String {
  process(input).to_string()
}

#[derive(Debug, Clone)]
struct Line {
  record: String,
  groups: Vec<usize>,
}

impl From<&str> for Line {
  fn from(value: &str) -> Self {
    let (records, groups) = value.trim().split_once(' ').unwrap();

    let record = records.trim().to_owned() + ".";

    let groups = groups
      .trim()
      .split(',')
      .map(|c| c.parse::<usize>().unwrap())
      .collect();

    Self { record, groups }
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
      .collect::<Vec<Line>>();

    Self { lines }
  }
}

fn solve_patter_with_memo(
  i: usize,
  gi: usize,
  pattern: &str,
  groups: &Vec<usize>,
  memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
  // If we reach the end of groups, check if the remaining pattern is all '.' or '?'
  if gi == groups.len() {
    if (i..pattern.len()).any(|j| pattern.chars().nth(j).unwrap() == '#') {
      return 0;
    } else {
      return 1;
    }
  }

  // If we reach the end of pattern, return 0
  if i == pattern.len() {
    return 0;
  }

  if let Some(ans) = memo[i][gi] {
    return ans;
  }

  let mut ans = 0;

  // Try to fit the group at the next index
  if pattern.chars().nth(i).unwrap() != '#' {
    ans += solve_patter_with_memo(i + 1, gi, pattern, groups, memo);
  }

  // Try to fit the group at the current index
  if i + groups[gi] < pattern.len()
    && (i..i + groups[gi]).all(|j| pattern.chars().nth(j).unwrap() != '.')
    && pattern.chars().nth(i + groups[gi]).unwrap() != '#'
  {
    ans += solve_patter_with_memo(i + groups[gi] + 1, gi + 1, pattern, groups, memo);
  }

  memo[i][gi] = Some(ans);
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
mod test_day12_part01_example {
  use super::*;

  #[test]
  pub fn example() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    assert_eq!(process(input), 21);
  }
}
