pub fn exec() -> () {
  let input = include_str!("input1.txt");
  let output = process(input);
  println!("Day 01 - Part 01 - {output}");
}

fn process(input: &str) -> String {
  "The solution should be here".into()
}

#[cfg(test)]
mod tests {
  // use super::*;
}
