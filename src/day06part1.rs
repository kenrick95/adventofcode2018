use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

fn find_closest_index((y, x): (usize, usize), list: &Vec<(usize, usize)>) -> i32 {
    let mut closest_index = -1;
    let mut closest_value = 10000;
    let mut i = 0;
    while i < list.len() {
        let (iy, ix) = list[i];
        let tmp = (ix as i32 - x as i32).abs() + (iy as i32 - y as i32).abs();
        if tmp < closest_value {
            closest_index = i as i32;
            closest_value = tmp;
        } else if tmp == closest_value {
            closest_index = -1;
        }
        i += 1;
    }
    return closest_index;
}

pub fn main() {
    let MAP_SIZE = 400;
    let mut map: Vec<Vec<i32>> = Vec::new();
    // Fill map MAP_SIZExMAP_SIZE with "-1"
    {
        let mut i = 0;
        while i < MAP_SIZE {
            let mut j = 0;
            let mut tmp: Vec<i32> = Vec::new();
            while j < MAP_SIZE {
                tmp.push(-1);
                j += 1;
            }
            map.push(tmp);
            i += 1;
        }
    }

    let mut list: Vec<(usize, usize)> = Vec::new();
    let mut index = 0;
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

        map[y][x] = index;
        index += 1;
    }

    // For each point in map, compute what is closest
    {
        let mut i = 0;
        while i < MAP_SIZE {
            let mut j = 0;
            while j < MAP_SIZE {
                map[i][j] = find_closest_index((i, j), &list);

                j += 1;
            }
            i += 1;
        }
    }
    // println!("{:?}", map);

    // Find index on the edge to invalidate
    {
        let mut invalidate_set = HashSet::new();
        let mut i = 0;
        while i < MAP_SIZE {
            invalidate_set.insert(map[i][0]);
            invalidate_set.insert(map[0][i]);
            invalidate_set.insert(map[i][MAP_SIZE - 1]);
            invalidate_set.insert(map[MAP_SIZE - 1][i]);
            i += 1;
        }
        // println!("{:?}", invalidate_set);

        i = 0;
        while i < MAP_SIZE {
            let mut j = 0;
            while j < MAP_SIZE {
                if invalidate_set.contains(&map[i][j]) {
                    map[i][j] = -1;
                }
                j += 1;
            }
            i += 1;
        }
    }
    // println!("{:?}", map);

    // Count value of remaining index
    let mut counter = HashMap::new();
    {
        let mut i = 0;
        while i < MAP_SIZE {
            let mut j = 0;
            while j < MAP_SIZE {
                if map[i][j] != -1 {
                    let count = counter
                        .entry(map[i][j])
                        .or_insert(0);
                    *count += 1;
                }
                j += 1;
            }
            i += 1;
        }
    }
    println!("{:?}", counter);
    println!("Part 1: {:?}", counter.values().max().unwrap());
}
