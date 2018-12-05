// use std::io;
use std::collections::HashSet;
use std::fs;

fn transform(list: &Vec<char>) -> Vec<char> {
    let mut transformed_list = Vec::new();
    let mut i = 0;
    let mut skipping = false;
    while i < list.len() - 1 {
        let current_char = list[i];
        let next_char = list[i + 1];
        if (current_char.is_lowercase()
            && next_char.is_uppercase()
            && current_char.to_ascii_uppercase() == next_char)
            || (current_char.is_uppercase()
                && next_char.is_lowercase()
                && next_char.to_ascii_uppercase() == current_char)
        {
            // skip this char
            i += 2;
            skipping = true;
            // println!("{:?} skip", transformed_list);
            continue;
        }
        transformed_list.push(current_char);
        i += 1;
        skipping = false;
        // println!("{:?}", transformed_list);
    }
    if !skipping && list.len() >= 1 {
        transformed_list.push(list[list.len() - 1]);
    }
    // println!("{:?}", transformed_list);
    return transformed_list;
}

fn react(input_list: &Vec<char>) -> Vec<char> {
    let mut list = input_list.clone();
    let mut new_list: Vec<char>;
    loop {
        new_list = transform(&list);
        if new_list.len() == list.len() {
            break;
        }
        list = new_list;
    }
    return list;
}

pub fn main() {
    let mut input =
        String::from(fs::read_to_string("./src/day05.in").expect("Unable to read file"));
    // io::stdin().read_line(&mut input).unwrap();
    input = String::from(input.trim());
    let mut set = HashSet::new();

    let mut list: Vec<char> = Vec::new();
    for c in input.chars() {
        list.push(c);
        set.insert(c.to_ascii_uppercase());
    }

    println!("Part 1: {:?}", react(&list).len());

    // Part 2: Removing one type of character, find the shortest final reacted list
    // *so rustic* but maybe not that efficient since it involves many clones
    println!("{}", set.len());
    let p2_ans = set
        .iter()
        .map(|c| {
            let new_list = list
                .clone()
                .into_iter()
                .filter(|lc| lc.to_ascii_uppercase() != *c)
                .collect();
            react(&new_list).len()
        }).min();
    println!("Part 2: {:?}", p2_ans.unwrap());
}
