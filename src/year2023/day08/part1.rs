use std::collections::HashMap;

pub fn exec(input: &str) -> String {
  process(input).to_string()
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

#[derive(Debug)]
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
}

impl From<&str> for Map {
  fn from(value: &str) -> Self {
    let lines: Vec<&str> = value.trim().lines().collect();
    let nav_line = lines[0];
    let guides = NavGuide::from(nav_line);

    let node_lines = &lines[2..];
    let nodes_vec: Vec<Node> = node_lines.iter().map(|&line| Node::from(line)).collect();
    let mut nodes = HashMap::with_capacity(nodes_vec.len());
    for node in nodes_vec.into_iter() {
      nodes.insert(node.value.clone(), node);
    }

    Self { guides, nodes }
  }
}

impl Map {
  pub fn navigate(&self) -> usize {
    let mut index = 0;
    let map = &self.nodes;
    let mut actual: &Node = map.get("AAA".into()).unwrap();
    let guides = &self.guides.0;

    loop {
      let guide_index = index % guides.len();
      let actual_move = &guides[guide_index];
      match actual_move {
        Nav::Left => actual = map.get(&actual.left).unwrap(),
        Nav::Right => actual = map.get(&actual.right).unwrap(),
      }

      index += 1;

      if actual.value == "ZZZ" {
        break;
      }
    }
    index
  }
}

fn process(input: &str) -> usize {
  let map = Map::from(input);
  map.navigate()
}

#[cfg(test)]
mod test_day08_part01_example {
  use super::*;

  #[test]
  pub fn example_01() {
    let input = "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";
    assert_eq!(process(input), 2);
  }

  #[test]
  pub fn example_02() {
    let input = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";
    assert_eq!(process(input), 6);
  }
}
