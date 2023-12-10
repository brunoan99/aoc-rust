use std::collections::HashMap;

pub fn exec() -> () {
  let input = include_str!("input.txt");
  let output = process(input);
  println!("Day 08 - Part 02 - {output}");
}

#[derive(Debug)]
enum Nav {
  Left,
  Right,
}

impl From<char> for Nav {
  fn from(value: char) -> Self {
    match value {
      'L' => Nav::Left,
      'R' => Nav::Right,
      _ => panic!("Unknow Nav direction"),
    }
  }
}

#[derive(Debug)]
struct NavGuide(Vec<Nav>);

impl From<&str> for NavGuide {
  fn from(value: &str) -> Self {
    let navs: Vec<Nav> = value.chars().map(Nav::from).collect();
    Self(navs)
  }
}

#[derive(Debug, Clone)]
struct Node {
  value: String,
  left: String,
  right: String,
}

impl From<&str> for Node {
  fn from(value: &str) -> Self {
    let (value, nav) = value.split_once('=').unwrap();
    let value = value.trim().into();

    let nav = nav.replace("(", "").replace(")", "");
    let (left, right) = nav.split_once(',').unwrap();
    let left = left.trim().into();
    let right = right.trim().into();
    Node { value, left, right }
  }
}

#[derive(Debug)]
struct Map {
  guides: NavGuide,
  nodes: HashMap<String, Node>,
  starts: Vec<Node>,
}

impl From<&str> for Map {
  fn from(value: &str) -> Self {
    let lines: Vec<&str> = value.trim().lines().collect();
    let nav_line = lines[0];
    let guides = NavGuide::from(nav_line);

    let nodes_vec: Vec<Node> = (&lines[2..]).iter().map(|&line| Node::from(line)).collect();
    let mut nodes = HashMap::with_capacity(nodes_vec.len());
    let mut starts: Vec<Node> = vec![];
    for node in nodes_vec.into_iter() {
      if node.value.ends_with('A') {
        starts.push(node.clone());
      }
      nodes.insert(node.value.clone(), node);
    }

    Self {
      guides,
      nodes,
      starts,
    }
  }
}
/*



pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

/// Lowest Common Multiple (LCM) of the number and `other`.
#[inline]
fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
    if self.is_zero() && other.is_zero() {
        return (Self::zero(), Self::zero());
    }
    let gcd = self.gcd(other);
    // should not have to recalculate abs
    let lcm = (*self * (*other / gcd)).abs();
    (gcd, lcm)
}
*/
pub fn greater_common_divisor(a: usize, b: usize) -> usize {
  let mut a = a;
  let mut b = b;
  while b != 0 {
    let tmp = a;
    a = b;
    b = tmp % b;
  }
  a
}

pub fn lowest_common_multiple(a: usize, b: usize) -> usize {
  if a == 0 || b == 0 {
    return 0;
  }
  let gcd = greater_common_divisor(a, b);
  let lcm = a * (b / gcd);
  lcm
}

impl Map {
  pub fn navigate_for_node(&self, node: &Node) -> usize {
    let mut index = 0;
    let map = &self.nodes;
    let mut actual: &Node = map.get(&node.value).unwrap();
    let guides = &self.guides.0;

    loop {
      let guide_index = index % guides.len();
      let actual_move = &guides[guide_index];
      match actual_move {
        Nav::Left => actual = map.get(&actual.left).unwrap(),
        Nav::Right => actual = map.get(&actual.right).unwrap(),
      }

      index += 1;

      if actual.value.ends_with('Z') {
        break;
      }
    }

    index
  }

  pub fn navigate(&self) -> usize {
    let map = &self.nodes;
    let actuals: Vec<&Node> = self
      .starts
      .iter()
      .map(|node| map.get(&node.value).unwrap())
      .collect();
    actuals
      .iter()
      .map(|node| self.navigate_for_node(node))
      .fold(1, |acc, nav| lowest_common_multiple(acc, nav))
  }
}

fn process(input: &str) -> usize {
  let map = Map::from(input);
  map.navigate()
}

#[cfg(test)]
mod test_day08_part02_example {
  use super::*;

  #[test]
  pub fn example_01() {
    let input = "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";
    assert_eq!(process(input), 6);
  }
}
