use regex::Regex;
use std::io;
fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return String::from(input.trim());
}
const MAP_SIZE: usize = 2000;

#[derive(Copy, Clone, PartialEq)]
enum State {
    None,
    Cell,
    Water,
    WaterSource,
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
    loop {
        let input = read_stdin();
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
    // Hmm I actually have no idea how to implement yet ...
}
