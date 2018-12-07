use std::io;

const DISTANCE_THRESHOLD: i32 = 10000;
const MAP_SIZE: usize = 400;
fn satisfy_distance((y, x): (usize, usize), list: &Vec<(usize, usize)>) -> bool {
    let total_manhattan_disatance = list.iter().fold(0, |acc, item| {
        let (iy, ix) = *item;
        return acc + (iy as i32 - y as i32).abs() + (ix as i32 - x as i32).abs();
    });
    // println!("{} {}, {}", y, x, total_manhattan_disatance);

    return total_manhattan_disatance < DISTANCE_THRESHOLD;
}

pub fn main() {
    let mut map: Vec<Vec<bool>> = Vec::new();
    // Fill map MAP_SIZExMAP_SIZE with "false"
    {
        let mut i = 0;
        while i < MAP_SIZE {
            let mut j = 0;
            let mut tmp: Vec<bool> = Vec::new();
            while j < MAP_SIZE {
                tmp.push(false);
                j += 1;
            }
            map.push(tmp);
            i += 1;
        }
    }

    let mut list: Vec<(usize, usize)> = Vec::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());
        if input == "" {
            break;
        }

        let splitted: Vec<&str> = input.split_terminator(", ").collect();
        let x = String::from(splitted[0]).parse().unwrap();
        let y = String::from(splitted[1]).parse().unwrap();

        list.push((y, x));
    }

    let mut counter = 0;
    {
        let mut i: usize = 0;
        while i < MAP_SIZE {
            let mut j: usize = 0;
            let mut tmp: Vec<bool> = Vec::new();
            while j < MAP_SIZE {
                let satisfied = satisfy_distance((i, j), &list);
                tmp.push(satisfied);
                if satisfied {
                    counter += 1;
                }
                j += 1;
            }
            map.push(tmp);
            i += 1;
        }
    }
    println!("Part 2: {:?}", counter);
}
