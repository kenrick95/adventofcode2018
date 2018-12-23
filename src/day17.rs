use regex::Regex;
use std::collections::VecDeque;
use std::fs;

const MAP_SIZE: usize = 2000;

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    None,
    Cell,
    Water,
    WaterSource,
}


fn read_inputs() -> Vec<String> {
  let all_inputs = String::from(
    String::from(fs::read_to_string("./src/day17.in").expect("Unable to read file")).trim(),
  );
  let inputs_str: Vec<&str> = all_inputs.split('\n').collect();
  let inputs = inputs_str
    .clone()
    .iter()
    .map(|x| String::from(x.clone()))
    .collect();
  return inputs;
}

pub fn main() {
    let re = Regex::new(r"(.)=(\d+), (.)=(\d+)\.\.(\d+)").unwrap();
    // Water source: x=500, y=0
    let mut map: Vec<Vec<State>> = vec![];
    {
        let mut tmp = Vec::new();
        tmp.resize(MAP_SIZE, State::None);
        map.resize(MAP_SIZE, tmp);
    }
    let inputs = read_inputs();
    for input in inputs {
        let caps = re.captures(input.as_str()).unwrap();
        let char_1 = String::from(caps.get(1).unwrap().as_str());
        let char_1_coord: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let char_2 = String::from(caps.get(3).unwrap().as_str());
        let char_2_coord_from: usize = caps.get(4).unwrap().as_str().parse().unwrap();
        let char_2_coord_to: usize = caps.get(5).unwrap().as_str().parse().unwrap();

        if char_1 == "x" && char_2 == "y" {
            for i in char_2_coord_from..(char_2_coord_to + 1) {
                map[i][char_1_coord] = State::Cell;
            }
        } else if char_1 == "y" && char_2 == "x" {
            for i in char_2_coord_from..(char_2_coord_to + 1) {
                map[char_1_coord][i] = State::Cell;
            }
        }
    }
    map[0][500] = State::WaterSource;

    // Every tick, pour water
    let mut t = 0;
    for t in 0..1 {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let head = (0, 500);
        queue.push_back(head);
        while let Some(head) = queue.pop_front() {
            println!("> {:?} {:?}", head, map[head.0][head.1]);
            let next = (head.0 + 1, head.1);

            let head_left = (head.0, head.1 - 1);
            let next_left = (head.0 + 1, head.1 - 1);
            let head_right = (head.0, head.1 + 1);
            let next_right = (head.0 + 1, head.1 - 1);
            if map[next.0][next.1] == State::None {
                map[next.0][next.1] = State::Water;
                queue.push_back(next);
            } else if map[next.0][next.1] == State::Cell {
                // Fill left and right till a space is found
                if map[head_left.0][head_left.1] == State::None
                    && map[next_left.0][next_left.1] == State::Cell
                {
                    map[head_left.0][head_left.1] = State::Water;
                    queue.push_back(head_left);
                }

                if map[head_right.0][head_right.1] == State::None
                    && map[next_right.0][next_right.1] == State::Cell
                {
                    map[head_right.0][head_right.1] = State::Water;
                    queue.push_back(head_right);
                }
            }
            // The above will: pour water down, if hits a "pail", will spread water to left and right at height=1

            // Check all the Waters in previous line if there are more eligible candidates
            if head.0 > 0 {
                for j in 0..MAP_SIZE {
                    // TODO: Can only push
                    // 1. prev line is water
                    // 2. current line is filled with water/cell
                    // 3. "bounded" number of Nones in prev line before hitting a cell <-- okay I'm a bit stuck in formulating this ...
                    if map[head.0 - 1][j] == State::Water {
                        if j > 0 && map[head.0 - 1][j - 1] == State::None {
                            if map[head.0][j - 1] == State::Water {
                                map[head.0 - 1][j - 1] = State::Water;
                                queue.push_back((head.0 - 1, j - 1));
                            }
                        } else if j < MAP_SIZE - 1 && map[head.0 - 1][j + 1] == State::None {
                            if map[head.0][j + 1] == State::Water {
                                map[head.0 - 1][j + 1] = State::Water;
                                queue.push_back((head.0 - 1, j + 1));
                            }
                        }
                    }
                }
            }
        }
    }
}
