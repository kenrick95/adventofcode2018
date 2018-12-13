use std::fmt;
use std::io;

#[derive(Copy, Clone, PartialEq)]
enum Direction {
  None,
  North,
  East,
  South,
  West,
  Collision,
}
#[derive(Copy, Clone)]
struct Cell {
  track_char: char,

  has_moved: bool,

  // 0: no cart; 1: N; 2: E; 3: S; 4: W
  cart_direction: Direction,

  // cart_state: number of times it has faced an intersection mod 3
  // 0 --> next will turn left
  // 1 --> ...       straight
  // 2 --> ...       turn right
  cart_state: u8,
}

impl fmt::Debug for Cell {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut ch = self.track_char;

    if self.cart_direction == Direction::East {
      ch = '>';
    } else if self.cart_direction == Direction::West {
      ch = '<';
    } else if self.cart_direction == Direction::North {
      ch = '^';
    } else if self.cart_direction == Direction::South {
      ch = 'v';
    } else if self.cart_direction == Direction::Collision {
      ch = 'X';
    }

    write!(f, "{}", ch)
  }
}

fn construct_map(raw_map: &Vec<Vec<char>>) -> Vec<Vec<Cell>> {
  let mut map = Vec::new();
  let has_moved = false;

  for row in raw_map {
    let mut tmp = Vec::new();
    for c in row {
      let ch = *c;
      if ch == '-' || ch == '/' || ch == '\\' || ch == '|' || ch == '+' || ch == ' ' {
        tmp.push(Cell {
          track_char: ch,
          cart_direction: Direction::None,
          cart_state: 0,
          has_moved,
        });
      } else if ch == '>' {
        tmp.push(Cell {
          track_char: '-',
          cart_direction: Direction::East,
          cart_state: 0,
          has_moved,
        });
      } else if ch == '<' {
        tmp.push(Cell {
          track_char: '-',
          cart_direction: Direction::West,
          cart_state: 0,
          has_moved,
        });
      } else if ch == '^' {
        tmp.push(Cell {
          track_char: '|',
          cart_direction: Direction::North,
          cart_state: 0,
          has_moved,
        });
      } else if ch == 'v' {
        tmp.push(Cell {
          track_char: '|',
          cart_direction: Direction::South,
          cart_state: 0,
          has_moved,
        });
      }
    }
    map.push(tmp);
  }
  return map;
}

fn count_cart(map: &Vec<Vec<Cell>>) -> i32 {
  let mut count = 0;
  for row in map {
    for cell in row {
      if cell.cart_direction != Direction::None {
        count += 1;
      }
    }
  }
  return count;
}

// So dirty and it causes so many bugs... :facepalm:
fn tick(map: &Vec<Vec<Cell>>, remove_collision: bool) -> (Vec<Vec<Cell>>, bool) {
  let mut new_map = map.clone();
  let mut collision = false;

  // clear all carts has_moved in new_map
  {
    for i in 0..map.len() {
      for j in 0..map[i].len() {
        let cell = new_map[i][j];
        new_map[i][j] = Cell {
          has_moved: false,
          ..cell
        };
      }
    }
  }

  {
    for i in 0..new_map.len() {
      for j in 0..new_map[i].len() {
        let cell = new_map[i][j];

        // if cart, then move it; else skip
        if cell.cart_direction == Direction::None {
          continue;
        }

        if cell.has_moved {
          continue;
        }

        // It is guaranteed that next i and next j is on track because we should already turn the cart to correct direction before placing in next_cell
        let (mut next_i, mut next_j) = (i, j);
        if cell.cart_direction == Direction::North {
          next_i -= 1;
        } else if cell.cart_direction == Direction::East {
          next_j += 1;
        } else if cell.cart_direction == Direction::South {
          next_i += 1;
        } else if cell.cart_direction == Direction::West {
          next_j -= 1;
        }

        // Check for collision
        let next_cell = new_map[next_i][next_j];
        if next_cell.cart_direction != Direction::None {
          collision = true;
        }

        if collision {
          if remove_collision {
            collision = false;
            // Nullify both cells
            new_map[i][j] = Cell {
              cart_direction: Direction::None,
              cart_state: 0,
              ..cell
            };
            new_map[next_i][next_j] = Cell {
              cart_direction: Direction::None,
              cart_state: 0,
              ..next_cell
            };
            continue;
          } else {
            new_map[next_i][next_j] = Cell {
              cart_direction: Direction::Collision,
              has_moved: true,
              ..next_cell
            };
            break;
          }
        }

        // Place a cart here
        let mut next_direction = cell.cart_direction;
        let mut next_state = cell.cart_state;
        if next_cell.track_char == '/' {
          // Must turn
          if cell.cart_direction == Direction::East {
            next_direction = Direction::North;
          } else if cell.cart_direction == Direction::West {
            next_direction = Direction::South;
          } else if cell.cart_direction == Direction::North {
            next_direction = Direction::East;
          } else if cell.cart_direction == Direction::South {
            next_direction = Direction::West;
          }
        } else if next_cell.track_char == '\\' {
          // Must turn
          if cell.cart_direction == Direction::East {
            next_direction = Direction::South;
          } else if cell.cart_direction == Direction::West {
            next_direction = Direction::North;
          } else if cell.cart_direction == Direction::North {
            next_direction = Direction::West;
          } else if cell.cart_direction == Direction::South {
            next_direction = Direction::East;
          }
        } else if next_cell.track_char == '+' {
          // Intersection
          if cell.cart_state == 0 {
            // Turn left
            if cell.cart_direction == Direction::East {
              next_direction = Direction::North;
            } else if cell.cart_direction == Direction::West {
              next_direction = Direction::South;
            } else if cell.cart_direction == Direction::North {
              next_direction = Direction::West;
            } else if cell.cart_direction == Direction::South {
              next_direction = Direction::East;
            }
          } else if cell.cart_state == 1 {
            // Go straight, direction unchanged
          } else if cell.cart_state == 2 {
            // Turn right
            if cell.cart_direction == Direction::East {
              next_direction = Direction::South;
            } else if cell.cart_direction == Direction::West {
              next_direction = Direction::North;
            } else if cell.cart_direction == Direction::North {
              next_direction = Direction::East;
            } else if cell.cart_direction == Direction::South {
              next_direction = Direction::West;
            }
          }

          next_state = (cell.cart_state + 1) % 3;
        }

        new_map[i][j] = Cell {
          cart_direction: Direction::None,
          cart_state: 0,
          ..cell
        };
        new_map[next_i][next_j] = Cell {
          cart_direction: next_direction,
          cart_state: next_state,
          has_moved: true,
          ..next_cell
        };
      }

      if collision {
        break;
      }
    }
  }

  if remove_collision && count_cart(&new_map) == 1 {
    return (new_map, true);
  }

  return (new_map, collision);
}

pub fn main() {
  let mut raw_map: Vec<Vec<char>> = Vec::new();
  loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "" {
      break;
    }

    let line: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
    raw_map.push(line);
  }

  let original_map: Vec<Vec<Cell>> = construct_map(&raw_map);

  let skip_part_1 = true;
  let skip_part_2 = false;
  if !skip_part_1 {
    let mut map = original_map.clone();
    let mut t = 0;
    // println!("{}: {:?}", t, map);
    loop {
      t += 1;
      let (new_map, has_crash) = tick(&map, false);
      // println!("{}: {:?}", t, new_map);
      map = new_map;
      if has_crash {
        break;
      }
    }

    let mut i = 0;
    let mut j = 0;
    let mut found = false;
    for row in map {
      j = 0;
      for cell in row {
        if cell.cart_direction == Direction::Collision {
          found = true;
          break;
        }
        j += 1;
      }
      if found {
        break;
      }
      i += 1;
    }
    // Part 1: 64,86 is wrong ._.
    // 64,57 is correct
    println!("Part 1: {},{}", j, i);
  }

  if !skip_part_2 {
    let mut map = original_map.clone();
    let mut t = 0;
    // println!("{}: {:?}", t, map);
    loop {
      t += 1;
      let (new_map, should_break) = tick(&map, true);
      // println!("{}: {:?}", t, new_map);
      println!("{}, count {}", t, count_cart(&new_map));
      map = new_map;
      if should_break {
        break;
      }
    }

    let mut i = 0;
    let mut j = 0;
    let mut found = false;
    for row in map {
      j = 0;
      for cell in row {
        if cell.cart_direction != Direction::None {
          found = true;
          break;
        }
        j += 1;
      }
      if found {
        break;
      }
      i += 1;
    }
    // Part 2: 99,3 is wrong ._.
    println!("Part 2: {},{}", j, i);
  }
}
