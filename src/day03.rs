use regex::Regex;
use std::collections::HashMap;
use std::io;

pub fn main() {
    let re = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();
    let mut map: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    // Init 0 to whole map
    let mut i = 0;
    while i < 1001 {
        let mut j = 0;
        while j < 1001 {
            map.insert((i, j), Vec::new());
            j += 1;
        }
        i += 1;
    }
    println!("Init done");

    let mut inputs: Vec<(i32, i32, i32, i32, i32)> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());
        if input == "" {
            break;
        }
        // println!("input {:?}", input);
        // Process input, line by line... I have feeling that ID might be used on part 2
        let caps = re.captures(input.as_str()).unwrap();
        // caps:  Captures({0: Some("#24 @ 324,168: 12x21"), 1: Some("24"), 2: Some("324"), 3: Some("168"), 4: Some("12"), 5: Some("21")})
        let id: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let x: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let y: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let w: i32 = caps.get(4).unwrap().as_str().parse().unwrap();
        let h: i32 = caps.get(5).unwrap().as_str().parse().unwrap();

        // println!("{} {} {} {} {}", id, x, y, w, h);
        inputs.push((id, x, y, w, h));

        let mut i = x;

        while i < x + w {
            let mut j = y;
            while j < y + h {
                // println!("> {} {} {}, {:?}", id, i, j, map.get(&(i, j)));

                let mut val = map.get(&(i, j)).unwrap().clone();
                val.push(id);
                map.insert((i, j), val);

                j += 1;
            }
            i += 1;
        }
    }

    let mut count = 0;
    for val in map.values() {
        if val.len() >= 2 {
            count += 1;
        }
    }
    println!("Part 1: {}", count);

    // Go through the map again, see which one have val == 1
    for input in inputs {
        let (id, x, y, w, h) = input;
        let mut i = x;

        let mut possible = true;

        while i < x + w {
            let mut j = y;
            while j < y + h {
                let val = map.get(&(i, j)).unwrap();

                if val.len() > 1 {
                    possible = false;
                    break;
                }
                j += 1;
            }
            if !possible { 
                break;
            }
            i += 1;
        }

        if possible {
            println!("Part 2: {}", id);
            break;
        }
    }
}
