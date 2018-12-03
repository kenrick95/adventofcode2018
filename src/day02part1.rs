use std::io;
use std::collections::HashMap;
// https://repl.it/@kenrick95/Aoc2018Day2Part1

pub fn main() {
    let mut inputs = Vec::new();
    let mut count2 = 0;
    let mut count3 = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "" {
            break;
        }
        inputs.push(input);
    }
    for input in &inputs {
        let mut char_counter = HashMap::new();
        for c in input.chars() {
            println!("{}", c);
            if char_counter.contains_key(&c) {
                let prev = *(char_counter.get(&c).unwrap());
                char_counter.insert(c, prev + 1);
            } else {
                char_counter.insert(c, 1);
            }
        }
        let valid2 = char_counter.values().find(|&&v| v == 2).is_some();
        let valid3 = char_counter.values().find(|&&v| v == 3).is_some();
        println!("{} {}", valid2, valid3);
        if valid2 {
            count2 += 1;
        }
        if valid3 {
            count3 += 1;
        }
    }
    println!("Part 1: {}", count2 * count3);
}
