pub fn exec(input: &str) -> String {
  process(input).to_string()
}

fn process(input: &str) -> usize {
  input
    .lines()
    .map(|line| {
      line
        .replace("zerone", "01")
        .replace("oneight", "18")
        .replace("twone", "21")
        .replace("threeight", "38")
        .replace("fiveight", "58")
        .replace("eightwo", "82")
        .replace("eighthree", "83")
        .replace("nineight", "98")
        .replace("zero", "0")
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
    })
    .filter_map(|line| {
      let digits = line.chars().filter(|n| n.is_digit(10)).collect::<Vec<_>>();
      let first = digits.first()?;
      let last = digits.last()?;
      format!("{first}{last}").parse::<usize>().ok()
    })
    .sum()
}

#[cfg(test)]
mod testsday01part02 {
  use super::*;

  #[test]
  pub fn example() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(process(input), 281)
  }
}
