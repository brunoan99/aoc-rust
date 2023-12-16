use std::collections::HashMap;

pub fn exec(input: &str) -> String {
  process(input).to_string()
}

#[derive(Debug)]
struct Puzzle {
  map: Vec<Vec<char>>,
}

impl From<&str> for Puzzle {
  fn from(value: &str) -> Self {
    let map = value
      .trim()
      .lines()
      .map(|line| line.trim().chars().collect::<Vec<char>>())
      .collect::<Vec<Vec<char>>>();
    Puzzle { map }
  }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
  x: usize,
  y: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
  ToTop,    // from bottom to top
  ToBottom, // from top to bottom
  ToRight,  // from left to right
  ToLeft,   // from right to left
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Move {
  point: Point,
  direction: Direction,
}

fn mov_drom_direction(dir: &Direction) -> (isize, isize) {
  match dir {
    Direction::ToTop => (-1, 0),
    Direction::ToBottom => (1, 0),
    Direction::ToRight => (0, 1),
    Direction::ToLeft => (0, -1),
  }
}

fn char_from_direction(dir: &Direction) -> char {
  match dir {
    Direction::ToTop => '^',
    Direction::ToBottom => 'v',
    Direction::ToRight => '>',
    Direction::ToLeft => '<',
  }
}

fn process_beam(map: &mut Vec<Vec<char>>, mov: &Move, memo: &mut HashMap<Move, ()>) {
  if let Some(_) = memo.get(&mov) {
    return;
  }
  let mut x = mov.point.x;
  let mut y = mov.point.y;
  let mut dir = mov.direction;
  let (mut x_mov, mut y_mov) = mov_drom_direction(&dir);

  while x < map.len() && y < map[x].len() {
    let mov = Move {
      point: Point { x, y },
      direction: dir,
    };
    memo.insert(mov, ());
    match (map[x][y], &dir) {
      ('^', _) | ('v', _) | ('<', _) | ('>', _) => {}
      ('.', _) => {
        // empty space
        map[x][y] = char_from_direction(&dir);
      }
      ('|', Direction::ToTop) | ('|', Direction::ToBottom) => {
        // the beam cross by the | without interact with it
        // so keep going
      }
      ('|', Direction::ToLeft) | ('|', Direction::ToRight) => {
        // the beam hit the | and its deflect to two different beams
        // so needs to split the beam
        if x > 0 {
          let to_top = Move {
            point: Point { x: x - 1, y },
            direction: Direction::ToTop,
          };
          process_beam(map, &to_top, memo);
        }
        if x < map.len() - 1 {
          let to_bottom = Move {
            point: Point { x: x + 1, y },
            direction: Direction::ToBottom,
          };
          process_beam(map, &to_bottom, memo);
        }
        break;
      }
      ('-', Direction::ToLeft) | ('-', Direction::ToRight) => {
        // the beam cross by the | without interact with it
      }
      ('-', Direction::ToTop) | ('-', Direction::ToBottom) => {
        // the beam hit the - and its defelct to two different beams
        // so needs to split the beam
        if y > 0 {
          let to_left = Move {
            point: Point { x, y: y - 1 },
            direction: Direction::ToLeft,
          };
          process_beam(map, &to_left, memo);
        }
        if y < map[x].len() - 1 {
          let to_right = Move {
            point: Point { x, y: y + 1 },
            direction: Direction::ToRight,
          };
          process_beam(map, &to_right, memo);
        }
        break;
      }
      ('/', Direction::ToTop) => {
        // the bem hit the / going to top direction
        // so will be deflected to right
        dir = Direction::ToRight;
        (x_mov, y_mov) = mov_drom_direction(&dir);
      }
      ('/', Direction::ToBottom) => {
        // the bem hit the / going to bottom direction
        // so will be deflected to left
        dir = Direction::ToLeft;
        (x_mov, y_mov) = mov_drom_direction(&dir);
      }
      ('/', Direction::ToLeft) => {
        // the bem hit the / going to left direction
        // so will be deflected to bottom
        dir = Direction::ToBottom;
        (x_mov, y_mov) = mov_drom_direction(&dir);
      }
      ('/', Direction::ToRight) => {
        // the bem hit the / going to right direction
        // so will be deflected to top
        dir = Direction::ToTop;
        (x_mov, y_mov) = mov_drom_direction(&dir);
      }
      ('\\', Direction::ToTop) => {
        // the bem hit the \ going to top direction
        // so will be deflected to left
        dir = Direction::ToLeft;
        (x_mov, y_mov) = mov_drom_direction(&dir);
      }
      ('\\', Direction::ToBottom) => {
        // the bem hit the \ going to bottom direction
        // so will be deflected to right
        dir = Direction::ToRight;
        (x_mov, y_mov) = mov_drom_direction(&dir);
      }
      ('\\', Direction::ToLeft) => {
        // the bem hit the \ going to left direction
        // so will be deflected to top
        dir = Direction::ToTop;
        (x_mov, y_mov) = mov_drom_direction(&dir);
      }
      ('\\', Direction::ToRight) => {
        // the bem hit the \ going to right direction
        // so will be deflected to bottom
        dir = Direction::ToBottom;
        (x_mov, y_mov) = mov_drom_direction(&dir);
      }
      _ => panic!("Unknow character {}", map[x][y]),
    }
    if x == 0 && x_mov == -1 {
      break;
    }
    if x == map.len() - 1 && x_mov == 1 {
      break;
    }
    if y == 0 && y_mov == -1 {
      break;
    }
    if y == map[x].len() - 1 && y_mov == 1 {
      break;
    }
    x = x.saturating_add_signed(x_mov);
    y = y.saturating_add_signed(y_mov);
  }
}

fn generate_diagram(memo: &mut HashMap<Move, ()>, lx: usize, ly: usize) -> Vec<Vec<char>> {
  let mut map = vec![vec!['.'; ly]; lx];

  for (x, y) in memo
    .keys()
    .into_iter()
    .map(|mov| mov.point)
    .map(|point| (point.x, point.y))
  {
    map[x][y] = '#'
  }

  map
}

fn count_energized(map: &Vec<Vec<char>>) -> usize {
  map
    .into_iter()
    .map(|line| line.into_iter().filter(|&char| char == &'#').count())
    .sum()
}

fn solve_from(map: &mut Vec<Vec<char>>, start: &Move) -> usize {
  let mut memo = HashMap::new();
  process_beam(map, start, &mut memo);
  let diagram = generate_diagram(&mut memo, map.len(), map[0].len());
  count_energized(&diagram)
}

fn get_edges(lx: usize, ly: usize) -> Vec<Move> {
  let mut edges = vec![];

  let pss = Point { x: 0, y: 0 };
  let pse = Point { x: 0, y: ly - 1 };
  let pes = Point { x: lx - 1, y: 0 };
  let pee = Point {
    x: lx - 1,
    y: ly - 1,
  };
  edges.push(Move {
    point: pss.clone(),
    direction: Direction::ToBottom,
  });
  edges.push(Move {
    point: pss.clone(),
    direction: Direction::ToRight,
  });
  edges.push(Move {
    point: pse.clone(),
    direction: Direction::ToBottom,
  });
  edges.push(Move {
    point: pse.clone(),
    direction: Direction::ToLeft,
  });
  edges.push(Move {
    point: pes.clone(),
    direction: Direction::ToTop,
  });
  edges.push(Move {
    point: pes.clone(),
    direction: Direction::ToRight,
  });
  edges.push(Move {
    point: pee.clone(),
    direction: Direction::ToTop,
  });
  edges.push(Move {
    point: pee.clone(),
    direction: Direction::ToLeft,
  });

  edges
}

fn get_corners(lx: usize, ly: usize) -> Vec<Move> {
  let mut corners = vec![];

  for x in 1..lx - 2 {
    corners.push(Move {
      point: Point { x, y: 0 },
      direction: Direction::ToRight,
    });
    corners.push(Move {
      point: Point { x, y: ly - 1 },
      direction: Direction::ToLeft,
    });
  }
  for y in 1..ly - 2 {
    corners.push(Move {
      point: Point { x: 0, y },
      direction: Direction::ToBottom,
    });
    corners.push(Move {
      point: Point { x: lx - 1, y },
      direction: Direction::ToTop,
    });
  }

  corners
}

fn solve(map: &mut Vec<Vec<char>>) -> usize {
  let lx = map.len();
  let ly = map[0].len();
  let starts: Vec<Move> = std::iter::empty()
    .chain(get_edges(lx, ly))
    .chain(get_corners(lx, ly))
    .collect();

  starts
    .into_iter()
    .map(|mov| solve_from(map, &mov))
    .max()
    .unwrap()
}

fn process(input: &str) -> usize {
  let mut puzzle = Puzzle::from(input);
  solve(&mut puzzle.map)
}

#[cfg(test)]
mod test_day16_part02_example {
  use super::*;

  #[test]
  pub fn example_2() {
    let input = r".|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....";
    assert_eq!(process(input), 51);
  }
}

// to low 7714
