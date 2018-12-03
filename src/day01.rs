use std::io;
use std::collections::HashMap;
// https://repl.it/@kenrick95/Aoc2018Day1

fn main() {
    let mut val = 0;
    let mut count = HashMap::new();
    let mut inputs = Vec::new();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut part2_found = false;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "" {
            break;
        }
        let number: i32 = input.trim().parse().unwrap();
        inputs.push(number);
    }

    let mut iter = 0;
    loop {
        for i in &inputs {
            let number = *i;
            val += number;
            if (iter == 0) {
                part1 = val;
            }

            if count.contains_key(&val) {
                let prev: i32 = *(count.get(&val).unwrap());
                count.insert(val, prev + 1);
                if !part2_found {
                    part2_found = true;
                    part2 = val;
                }
            } else {
                count.insert(val, 1);
            }
        }

        if (part2_found) {
            break;
        }
        iter += 1;
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
