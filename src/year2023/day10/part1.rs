pub fn exec(input: &str) -> String {
  process(input).to_string()
}

#[derive(Clone, Copy, Debug)]
struct Point {
  value: char,
  row: usize,
  col: usize,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
  Start,
  FromTop,
  FromRight,
  FromBottom,
  FromLeft,
}

#[derive(Clone, Copy, Debug)]
struct Movement {
  point: Point,
  direction: Direction,
}

#[derive(Debug)]
struct Path {
  movements: Vec<Movement>,
}

impl From<Vec<Vec<char>>> for Path {
  fn from(value: Vec<Vec<char>>) -> Self {
    let mut movements = vec![];
    let mut actual = get_start(&value).unwrap();
    movements.push(actual);

    loop {
      actual = get_next(&value, &actual);

      if actual.point.value == 'S' {
        break;
      }

      movements.push(actual);
    }

    Path { movements }
  }
}

fn get_next(matrix: &Vec<Vec<char>>, m: &Movement) -> Movement {
  let point = &m.point;
  let char = &m.point.value;
  let direction = &m.direction;
  match (char, direction) {
    ('S', _) => get_start_next(matrix, &m.point),
    ('|', Direction::FromTop) => Movement {
      point: get_on_bottom(matrix, point).unwrap(),
      direction: Direction::FromTop,
    },
    ('|', Direction::FromBottom) => Movement {
      point: get_on_top(matrix, point).unwrap(),
      direction: Direction::FromBottom,
    },
    ('-', Direction::FromLeft) => Movement {
      point: get_on_right(matrix, point).unwrap(),
      direction: Direction::FromLeft,
    },
    ('-', Direction::FromRight) => Movement {
      point: get_on_left(matrix, point).unwrap(),
      direction: Direction::FromRight,
    },
    ('L', Direction::FromTop) => Movement {
      point: get_on_right(matrix, point).unwrap(),
      direction: Direction::FromLeft,
    },
    ('L', Direction::FromRight) => Movement {
      point: get_on_top(matrix, point).unwrap(),
      direction: Direction::FromBottom,
    },
    ('J', Direction::FromTop) => Movement {
      point: get_on_left(matrix, point).unwrap(),
      direction: Direction::FromRight,
    },
    ('J', Direction::FromLeft) => Movement {
      point: get_on_top(matrix, point).unwrap(),
      direction: Direction::FromBottom,
    },
    ('7', Direction::FromLeft) => Movement {
      point: get_on_bottom(matrix, point).unwrap(),
      direction: Direction::FromTop,
    },
    ('7', Direction::FromBottom) => Movement {
      point: get_on_left(matrix, point).unwrap(),
      direction: Direction::FromRight,
    },
    ('F', Direction::FromRight) => Movement {
      point: get_on_bottom(matrix, point).unwrap(),
      direction: Direction::FromTop,
    },
    ('F', Direction::FromBottom) => Movement {
      point: get_on_right(matrix, point).unwrap(),
      direction: Direction::FromLeft,
    },
    _ => {
      panic!("Unknow char")
    }
  }
}

fn get_on_top(matrix: &Vec<Vec<char>>, point: &Point) -> Option<Point> {
  let row = point.row;
  let col = point.col;
  if row > 0 {
    Some(Point {
      value: matrix[row - 1][col],
      row: row - 1,
      col,
    })
  } else {
    None
  }
}

fn get_on_right(matrix: &Vec<Vec<char>>, point: &Point) -> Option<Point> {
  let row = point.row;
  let col = point.col;
  if col + 1 != matrix[row].len() {
    Some(Point {
      value: matrix[row][col + 1],
      row,
      col: col + 1,
    })
  } else {
    None
  }
}

fn get_on_bottom(matrix: &Vec<Vec<char>>, point: &Point) -> Option<Point> {
  let row = point.row;
  let col = point.col;
  if row + 1 != matrix.len() {
    Some(Point {
      value: matrix[row + 1][col],
      row: row + 1,
      col,
    })
  } else {
    None
  }
}

fn get_on_left(matrix: &Vec<Vec<char>>, point: &Point) -> Option<Point> {
  let row = point.row;
  let col = point.col;
  if col >= 1 {
    Some(Point {
      value: matrix[row][col - 1],
      row,
      col: col - 1,
    })
  } else {
    None
  }
}

fn get_adjacents(matrix: &Vec<Vec<char>>, point: &Point) -> Vec<Option<Point>> {
  let row = point.row;
  let col = point.col;
  let mut adjacents: Vec<Option<Point>> = vec![];

  // top point
  if row > 0 {
    adjacents.push(Some(Point {
      value: matrix[row - 1][col],
      row: row - 1,
      col,
    }));
  } else {
    adjacents.push(None);
  }
  // right point
  if col + 1 != matrix[row].len() {
    adjacents.push(Some(Point {
      value: matrix[row][col + 1],
      row,
      col: col + 1,
    }))
  } else {
    adjacents.push(None);
  }
  // bottom point
  if row + 1 != matrix.len() {
    adjacents.push(Some(Point {
      value: matrix[row + 1][col],
      row: row + 1,
      col,
    }));
  } else {
    adjacents.push(None);
  }
  // left point
  if col > 0 {
    adjacents.push(Some(Point {
      value: matrix[row][col - 1],
      row,
      col: col - 1,
    }));
  } else {
    adjacents.push(None);
  }

  adjacents
}

fn get_start_next(matrix: &Vec<Vec<char>>, point: &Point) -> Movement {
  let adjacents = get_adjacents(matrix, point);

  if let '|' | '7' | 'F' = adjacents[0].map_or_else(|| '.', |p| p.value) {
    // top point
    Movement {
      point: adjacents[0].unwrap().to_owned(),
      direction: Direction::FromBottom,
    }
  } else if let '-' | 'J' | '7' = adjacents[1].map_or_else(|| '.', |p| p.value) {
    // right point
    Movement {
      point: adjacents[1].unwrap().to_owned(),
      direction: Direction::FromLeft,
    }
  } else if let '|' | 'L' | 'J' = adjacents[2].map_or_else(|| '.', |p| p.value) {
    // bottom point
    Movement {
      point: adjacents[2].unwrap().to_owned(),
      direction: Direction::FromTop,
    }
  } else if let '-' | 'L' | 'F' = adjacents[3].map_or_else(|| '.', |p| p.value) {
    // left point
    Movement {
      point: adjacents[3].unwrap().to_owned(),
      direction: Direction::FromRight,
    }
  } else {
    panic!("No pipe connect with Start")
  }
}

fn get_start(matrix: &Vec<Vec<char>>) -> Option<Movement> {
  for row in 0..(matrix.len()) {
    for col in 0..(matrix[row].len()) {
      if matrix[row][col] == 'S' {
        return Some(Movement {
          point: Point {
            value: 'S',
            row,
            col,
          },
          direction: Direction::Start,
        });
      }
    }
  }
  None
}

fn process(input: &str) -> usize {
  let matrix = input
    .lines()
    .map(|line| line.trim().chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();
  let path = Path::from(matrix);
  let max_dist = path.movements.len() / 2;
  max_dist
}

#[cfg(test)]
mod test_day10_part01_example {
  use super::*;

  #[test]
  pub fn example_01() {
    let input = ".....
    .S-7.
    .|.|.
    .L-J.
    .....";
    assert_eq!(process(input), 4);
  }

  #[test]
  pub fn example_02() {
    let input = "..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...";
    assert_eq!(process(input), 8);
  }
}
