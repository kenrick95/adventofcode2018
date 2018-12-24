use regex::Regex;
use std::collections::VecDeque;
use std::fmt;
use std::fs;

const MAP_SIZE: usize = 2000;

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    None,
    Cell,
    Water,
    WaterSource,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::None => ".",
                State::Cell => "#",
                State::Water => "~",
                State::WaterSource => "W",
            }
        )
    }
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

fn print_map(map: &Vec<Vec<State>>, min_i: usize, min_j: usize, max_i: usize, max_j: usize) {
    for i in min_i..(max_i + 1) {
        for j in min_j..(max_j + 1) {
            print!("{}", map[i][j]);
        }
        println!();
    }
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
    let mut min_i = MAP_SIZE;
    let mut min_j = MAP_SIZE;
    let mut max_i = 0;
    let mut max_j = 0;
    let inputs = read_inputs();
    for input in inputs {
        let caps = re.captures(input.as_str()).unwrap();
        let char_1 = String::from(caps.get(1).unwrap().as_str());
        let char_1_coord: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let char_2 = String::from(caps.get(3).unwrap().as_str());
        let char_2_coord_from: usize = caps.get(4).unwrap().as_str().parse().unwrap();
        let char_2_coord_to: usize = caps.get(5).unwrap().as_str().parse().unwrap();

        if char_1 == "x" && char_2 == "y" {
            min_j = usize::min(min_j, char_1_coord);
            max_j = usize::max(max_j, char_1_coord);
            for i in char_2_coord_from..(char_2_coord_to + 1) {
                map[i][char_1_coord] = State::Cell;
                min_i = usize::min(min_i, i);
                max_i = usize::max(max_i, i);
            }
        } else if char_1 == "y" && char_2 == "x" {
            min_i = usize::min(min_i, char_1_coord);
            max_i = usize::max(max_i, char_1_coord);
            for j in char_2_coord_from..(char_2_coord_to + 1) {
                map[char_1_coord][j] = State::Cell;
                min_j = usize::min(min_j, j);
                max_j = usize::max(max_j, j);
            }
        }
    }
    // min_j = 0;
    // max_j = MAP_SIZE - 1;
    min_i = 0;
    min_j = usize::max(0, min_j - 10);
    max_j = usize::min(MAP_SIZE, max_j + 10);

    // map[6][501] = State::None;
    map[0][500] = State::WaterSource;

    // Every tick, pour water
    // let mut t = 0;
    {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let head = (0, 500);
        queue.push_back(head);
        while let Some(head) = queue.pop_front() {
            println!("> {:?} {:?}", head, map[head.0][head.1]);
            if head.0 > max_i {
                break;
            }

            let next = (head.0 + 1, head.1);

            if map[next.0][next.1] == State::Water {
                continue;
            }


            if map[next.0][next.1] == State::None {
                map[next.0][next.1] = State::Water;
                queue.push_back(next);
            } else if map[next.0][next.1] == State::Cell {
                let mut need_to_back_up = true;
                let mut di = 0;
                while need_to_back_up {
                    // Fill left and right till a space is found
                    let mut dleft = 0;
                    loop {
                        dleft += 1;
                        let head_left = (head.0 - di, head.1 - dleft);
                        let next_left = (head.0 - di + 1, head.1 - dleft);
                        if vec![State::None, State::Water].contains(&map[head_left.0][head_left.1])
                            && map[next_left.0][next_left.1] == State::Cell
                        {
                            map[head_left.0][head_left.1] = State::Water;
                        } else if di > 0
                            && vec![State::None, State::Water]
                                .contains(&map[head_left.0][head_left.1])
                            && map[next_left.0][next_left.1] == State::Water
                        {
                            map[head_left.0][head_left.1] = State::Water;
                        } else if vec![State::None, State::Water]
                            .contains(&map[head_left.0][head_left.1])
                            && map[next_left.0][next_left.1] == State::None
                        {
                            map[head_left.0][head_left.1] = State::Water;
                            map[next_left.0][next_left.1] = State::Water;
                            queue.push_back(next_left);
                            need_to_back_up = false;
                            break;
                        } else {
                            break;
                        }
                    }

                    let mut dright = 0;
                    loop {
                        dright += 1;
                        let head_right = (head.0 - di, head.1 + dright);
                        let next_right = (head.0 - di + 1, head.1 + dright);
                        if vec![State::None, State::Water]
                            .contains(&map[head_right.0][head_right.1])
                            && map[next_right.0][next_right.1] == State::Cell
                        {
                            map[head_right.0][head_right.1] = State::Water;
                        } else if di > 0
                            && vec![State::None, State::Water]
                                .contains(&map[head_right.0][head_right.1])
                            && map[next_right.0][next_right.1] == State::Water
                        {
                            map[head_right.0][head_right.1] = State::Water;
                        } else if vec![State::None, State::Water]
                            .contains(&map[head_right.0][head_right.1])
                            && map[next_right.0][next_right.1] == State::None
                        {
                            map[head_right.0][head_right.1] = State::Water;
                            map[next_right.0][next_right.1] = State::Water;
                            queue.push_back(next_right);
                            need_to_back_up = false;
                            break;
                        } else {
                            break;
                        }
                    }

                    di += 1;
                }
            }
            print_map(&map, min_i, min_j, max_i, max_j);
        }
    }

    {
        let mut answer = 0;
        for i in min_i..(max_i + 1) {
            for j in min_j..(max_j + 1) {
                if map[i][j] == State::Water {
                    answer += 1;
                }
            }
        }
        print_map(&map, min_i, min_j, max_i, max_j);
        // TODO: 1265 is too low
        println!("Part 1: {}", answer);
    }
}
