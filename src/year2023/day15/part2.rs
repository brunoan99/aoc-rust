pub fn exec(input: &str) -> String {
  process(input).to_string()
}

#[derive(Debug)]
struct Puzzle {
  hashs: Vec<String>,
}

impl From<&str> for Puzzle {
  fn from(value: &str) -> Self {
    let hashs = value
      .trim()
      .split(',')
      .map(|line| line.trim().to_owned())
      .collect();
    Self { hashs }
  }
}

#[derive(Debug)]
struct BoxOP {
  label: Vec<char>,
  op: char,
  focal_length: usize,
}

impl From<&str> for BoxOP {
  fn from(value: &str) -> Self {
    if value.contains("-") {
      Self {
        label: value.split_once("-").unwrap().0.chars().collect(),
        op: '-',
        focal_length: 0,
      }
    } else {
      let (label_str, focal_length_str) = value.split_once("=").unwrap();
      Self {
        label: label_str.chars().collect(),
        op: '=',
        focal_length: focal_length_str.parse().unwrap(),
      }
    }
  }
}

impl BoxOP {
  fn get_pos(&self) -> usize {
    self
      .label
      .iter()
      .fold(0, |acc, v| ((acc + v.clone() as usize) * 17) % 256)
  }
}

#[derive(Debug, Clone)]
struct Len {
  label: Vec<char>,
  focal_length: usize,
}

#[derive(Debug, Default, Clone)]
struct Box {
  lens: Vec<Len>,
}

#[derive(Debug)]
struct Slots {
  boxes: Vec<Box>,
}

impl From<Puzzle> for Slots {
  fn from(value: Puzzle) -> Self {
    let box_ops = value
      .hashs
      .into_iter()
      .map(|hash| BoxOP::from(hash.as_ref()))
      .collect::<Vec<BoxOP>>();

    let mut boxes: Vec<Box> = vec![Box { lens: vec![] }; 256];
    for box_op in box_ops.into_iter() {
      match box_op.op {
        '-' => {
          let pos = box_op.get_pos();
          let pos_in_lens_op = boxes[pos]
            .lens
            .iter()
            .position(|len| len.label == box_op.label);
          if let Some(pos_in_lens) = pos_in_lens_op {
            boxes[pos].lens.remove(pos_in_lens);
          }
        }
        '=' => {
          let pos = box_op.get_pos();
          let pos_in_lens_op = boxes[pos]
            .lens
            .iter()
            .position(|len| len.label == box_op.label);
          if let Some(pos_in_lens) = pos_in_lens_op {
            boxes[pos].lens[pos_in_lens].focal_length = box_op.focal_length;
          } else {
            boxes[pos].lens.push(Len {
              label: box_op.label,
              focal_length: box_op.focal_length,
            });
          }
        }
        _ => panic!("Unsuported operation"),
      }
    }

    Self { boxes }
  }
}

impl Slots {
  fn eval(self) -> usize {
    self
      .boxes
      .into_iter()
      .enumerate()
      .map(|(pos, b)| {
        b.lens
          .into_iter()
          .enumerate()
          .map(|(pos, len)| (pos + 1) * len.focal_length)
          .sum::<usize>()
          * (pos + 1)
      })
      .sum()
  }
}

fn process(input: &str) -> usize {
  let puzzle = Puzzle::from(input);
  let slots = Slots::from(puzzle);
  slots.eval()
}

#[cfg(test)]
mod test_day15_part02_example {
  use super::*;

  #[test]
  pub fn example_2() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(process(input), 145);
  }
}
