pub fn exec() -> () {
  let input = include_str!("input.txt");
  let output = process(input);
  println!("Day 01 - Part 01 - {output}");
}
fn process(input: &str) -> usize {
  input
    .lines()
    .filter_map(|line| {
      let digits = line.chars().filter(|n| n.is_digit(10)).collect::<Vec<_>>();
      let first = digits.first()?;
      let last = digits.last()?;
      format!("{first}{last}").parse::<usize>().ok()
    })
    .sum()
}

#[cfg(test)]
mod testsday01part01 {
  use super::*;

  #[test]
  pub fn example() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(process(input), 142)
  }
}
