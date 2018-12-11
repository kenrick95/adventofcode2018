use std::io;

const MAP_SIZE: usize = 300;
const SQUARE_SIZE: usize = 3;

fn get_cell_value((x, y): (usize, usize), sn: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let mut result: i32 = rack_id * (y as i32) + sn;
    result = (result % 1000 * rack_id % 1000) % 1000;
    result = result / 100;
    return result - 5;
}

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let sn: i32 = String::from(input.trim()).parse().unwrap();

    let mut map: Vec<Vec<i32>> = vec![];
    {
        // Init blank map
        let mut tmp: Vec<i32> = vec![];
        tmp.resize(MAP_SIZE, 0);
        map.resize(MAP_SIZE, tmp);
    }

    {
        let mut i = 0;
        while i < MAP_SIZE {
            let mut j = 0;
            while j < MAP_SIZE {
                // i and j is 1-based for get_cell_value
                map[i][j] = get_cell_value((j + 1, i + 1), sn);
                j += 1;
            }
            i += 1;
        }
    }

    // Some DP may be required?
    // cum_map[i][j] = sum of map[0..i][0..j] (inclusive)
    let mut cum_map: Vec<Vec<i32>> = vec![];
    {
        let mut tmp: Vec<i32> = vec![];
        tmp.resize(MAP_SIZE, 0);
        cum_map.resize(MAP_SIZE, tmp);
    }
    {
        let mut i = 0;
        while i < MAP_SIZE {
            let mut j = 0;
            while j < MAP_SIZE {
                // The main idea is here; the rest are edge cases to handle i or j accessing non-existent index
                if i > 0 && j > 0 {
                    cum_map[i][j] =
                        cum_map[i - 1][j] + cum_map[i][j - 1] - cum_map[i - 1][j - 1] + map[i][j];
                } else if i == 0 && j == 0 {
                    cum_map[i][j] = map[i][j];
                } else if i == 0 && j > 0 {
                    cum_map[i][j] = cum_map[i][j - 1] + map[i][j]
                } else {
                    // if i > 0 && j == 0 {
                    cum_map[i][j] = cum_map[i - 1][j] + map[i][j]
                }
                j += 1;
            }
            i += 1;
        }
    }

    // square_sum_map[i][j] = sum of map[i..i+2][j..j+2]
    // I actually don't need to save this square_sum_map
    let mut square_sum_map: Vec<Vec<i32>> = vec![];
    let mut largest_value: i32 = 0;
    let mut largest_key: (usize, usize) = (0, 0);
    {
        let mut tmp: Vec<i32> = vec![];
        tmp.resize(MAP_SIZE - SQUARE_SIZE, 0);
        square_sum_map.resize(MAP_SIZE - SQUARE_SIZE, tmp);
    }
    {
        let mut i = 0;
        while i < MAP_SIZE - SQUARE_SIZE {
            let mut j = 0;
            while j < MAP_SIZE - SQUARE_SIZE {
                if i > 0 && j > 0 {
                    square_sum_map[i][j] =
                        cum_map[i + 2][j + 2] - cum_map[i - 1][j + 2] - cum_map[i + 2][j - 1]
                            + cum_map[i - 1][j - 1];
                } else if i == 0 && j == 0 {
                    square_sum_map[i][j] = cum_map[i + 2][j + 2];
                } else if i == 0 && j > 0 {
                    square_sum_map[i][j] = cum_map[i + 2][j + 2] - cum_map[i + 2][j - 1];
                } else {
                    // if i > 0 && j == 0 {
                    square_sum_map[i][j] = cum_map[i + 2][j + 2] - cum_map[i - 1][j + 2];
                }

                if square_sum_map[i][j] > largest_value {
                    largest_value = square_sum_map[i][j];
                    largest_key = (i, j);
                }

                j += 1;
            }
            i += 1;
        }
    }

    {
        let (i, j) = largest_key;

        println!(
            "largest_value: {:?}, largest_key: {:?}",
            largest_value, largest_key
        );
        println!("Part 1: {},{}", j + 1, i + 1);
    }

    // Part 2: compute square_sum_map, but with SQUARE_SIZE=1..MAP_SIZE
    let mut p2_largest_value: i32 = 0;
    let mut p2_largest_key: (usize, usize, usize) = (0, 0, 1);
    {
        let mut size = 1;
        while size < MAP_SIZE + 1 {
            let mut i = 0;
            while i < MAP_SIZE - size {
                let mut j = 0;
                while j < MAP_SIZE - size {
                    // Size minus one
                    let size1 = size - 1;
                    let tmp;
                    if i > 0 && j > 0 {
                        tmp = cum_map[i + size1][j + size1]
                            - cum_map[i - 1][j + size1]
                            - cum_map[i + size1][j - 1]
                            + cum_map[i - 1][j - 1];
                    } else if i == 0 && j == 0 {
                        tmp = cum_map[i + size1][j + size1];
                    } else if i == 0 && j > 0 {
                        tmp = cum_map[i + size1][j + size1] - cum_map[i + size1][j - 1];
                    } else {
                        // if i > 0 && j == 0 {
                        tmp = cum_map[i + size1][j + size1] - cum_map[i - 1][j + size1];
                    }

                    if tmp > p2_largest_value {
                        p2_largest_value = tmp;
                        p2_largest_key = (i, j, size);
                    }

                    j += 1;
                }
                i += 1;
            }

            size += 1;
        }
    }

    {
        let (i, j, size) = p2_largest_key;

        println!(
            "p2_largest_value: {:?}, p2_largest_key: {:?}",
            p2_largest_value, p2_largest_key
        );
        println!("Part 2: {},{},{}", j + 1, i + 1, size);
    }
}
