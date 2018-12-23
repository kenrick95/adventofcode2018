use std::fmt;
use std::io;

fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return String::from(input.trim());
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Open,
    Tree,
    Lumberyard,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Open => ".",
                Cell::Tree => "|",
                Cell::Lumberyard => "#",
            }
        )
    }
}

fn tick(map: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let adjacents: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut result = Vec::new();
    for i in 0..map.len() {
        let mut tmp = Vec::new();
        for j in 0..map[i].len() {
            let cell = map[i][j];
            let mut count_open = 0;
            let mut count_tree = 0;
            let mut count_lumberyard = 0;
            for (di, dj) in adjacents.iter() {
                let ri = i as i32 + di;
                let rj = j as i32 + dj;
                if 0 <= ri && ri < map.len() as i32 && 0 <= rj && rj < map[i].len() as i32 {
                    let rcell = map[ri as usize][rj as usize];
                    match rcell {
                        Cell::Open => count_open += 1,
                        Cell::Tree => count_tree += 1,
                        Cell::Lumberyard => count_lumberyard += 1,
                    }
                }
            }
            let next_cell = match cell {
                Cell::Open => {
                    if count_tree >= 3 {
                        Cell::Tree
                    } else {
                        Cell::Open
                    }
                }
                Cell::Tree => {
                    if count_lumberyard >= 3 {
                        Cell::Lumberyard
                    } else {
                        Cell::Tree
                    }
                }
                Cell::Lumberyard => {
                    if count_lumberyard >= 1 && count_tree >= 1 {
                        Cell::Lumberyard
                    } else {
                        Cell::Open
                    }
                }
            };
            tmp.push(next_cell);
        }
        result.push(tmp);
    }
    return result;
}

fn count_cells(map: &Vec<Vec<Cell>>) -> (usize, usize, usize) {
    let mut count_open = 0;
    let mut count_tree = 0;
    let mut count_lumberyard = 0;
    for row in map.iter() {
        for cell in row.iter() {
            match cell {
                Cell::Open => count_open += 1,
                Cell::Tree => count_tree += 1,
                Cell::Lumberyard => count_lumberyard += 1,
            };
        }
    }
    return (count_open, count_tree, count_lumberyard);
}

pub fn main() {
    let mut original_map: Vec<Vec<Cell>> = vec![];
    loop {
        let line = read_stdin();
        if line == "" {
            break;
        }
        let mut tmp = Vec::new();
        for c in line.chars() {
            if c == '|' {
                tmp.push(Cell::Tree);
            } else if c == '#' {
                tmp.push(Cell::Lumberyard);
            } else {
                tmp.push(Cell::Open);
            }
        }
        original_map.push(tmp);
    }
    let mut map = original_map.clone();

    for t in 0..10 {
        let new_map = tick(&map);
        map = new_map;
        // println!("{:?}", map);
        // let (_, count_tree, count_lumberyard) = count_cells(&map);
        // println!("{:?}: {:?}, {:?}", t, count_tree, count_lumberyard);
    }
    {
        let (_, count_tree, count_lumberyard) = count_cells(&map);
        println!("Part 1: {:?}", count_tree * count_lumberyard);
    }
    // Let it run for a while till some repetition occurs
    for t in 10..5000 {
        let new_map = tick(&map);
        map = new_map;
    }
    // Form the sequence that repeats, then break once it repeats
    let mut seqs: Vec<(usize, usize)> = vec![];
    for t in 5000..15000 {
        let new_map = tick(&map);
        map = new_map;
        let (_, count_tree, count_lumberyard) = count_cells(&map);
        // println!("{:?}: {:?}, {:?}", t, count_tree, count_lumberyard);

        if seqs.len() > 0 && seqs[0].0 == count_tree && seqs[0].1 == count_lumberyard {
            // Saw it before, break the loop
            break;
        }
        seqs.push((count_tree, count_lumberyard));
    }
    // println!("seqs: {:?}", seqs);

    {
        let num_ticks = 1000000000;
        // yeah, off by one error
        let index = (num_ticks - 5000 - 1) % seqs.len();
        let (count_tree, count_lumberyard) = seqs[index];
        // 190314: too low
        // 197276: right answer
        // 201000, 202722: too high
        println!("Part 2: {:?}", count_tree * count_lumberyard);
    }
}
