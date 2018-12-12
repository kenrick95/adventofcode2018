use regex::Regex;
use std::collections::HashMap;
use std::io;

const OFFSET: usize = 2000;

fn mutate(state: Vec<char>, rules: &HashMap<String, char>) -> Vec<char> {
    let mut new_state = vec!['.'; state.len()];
    let mut i = 2;
    for window in state.windows(5) {
        let chars: String = (*window).into_iter().collect();
        let result = *(rules.get(&chars).unwrap_or(&'.'));
        // println!("chars: {:?}, {:?} ==> {:?}", i, chars, result);
        new_state[i] = result;
        i += 1;
    }
    return new_state;
}

fn get_sum(state: &Vec<char>) -> i32 {
    let mut sum = 0;
    let mut i = -(OFFSET as i32);
    for ch in state {
        if *ch == '#' {
            sum += i;
        }
        i += 1;
    }
    return sum;
}

fn get_sum_at_gen(gen: u128, diff: u128, point_gen: u128, point_value: u128) -> u128 {
    return (gen - point_gen) * diff + point_value;
}

pub fn main() {
    let re_init = Regex::new(r"^initial state: ([.#]+)$").unwrap();
    let re_rules = Regex::new(r"^([.#]+) => ([.#])$").unwrap();
    let mut rules: HashMap<String, char> = HashMap::new();

    let mut state: Vec<char>;
    // parse input to initial state
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());
        let caps_init = re_init.captures(input.as_str()).unwrap();
        let init_state_input = String::from(caps_init.get(1).unwrap().as_str());

        state = vec!['.'; OFFSET + init_state_input.len() + OFFSET];

        let mut index = OFFSET;
        for ch in init_state_input.chars() {
            if ch == '#' {
                state[index] = '#';
            }
            index += 1;
        }
    }

    // Parse input to rules
    let mut br_count = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());
        if input == "" {
            br_count += 1;
            if br_count > 2 {
                break;
            }
            continue;
        }

        let caps = re_rules.captures(input.as_str()).unwrap();
        let rule_from = String::from(caps.get(1).unwrap().as_str());
        let rule_result = String::from(caps.get(2).unwrap().as_str());

        rules.insert(rule_from, rule_result.chars().next().unwrap());
    }

    let state_string: String = state.clone().into_iter().collect();
    println!("0: {:?}, {:?}", state_string.len(), state_string);
    for gen in 0..20 {
        let new_state = mutate(state, &rules);
        state = new_state;

        let state_string: String = state.clone().into_iter().collect();
        // println!("{:?}: {:?}, {:?}", gen + 1, get_sum(&state), state_string);
    }

    let mut prev_sum = get_sum(&state);
    println!("Part 1: {:?}", prev_sum);

    let mut diff = 0;

    for gen in 21..1000 {
        let new_state = mutate(state, &rules);
        state = new_state;

        // let state_string: String = state.clone().into_iter().collect();

        let sum = get_sum(&state);
        diff = sum - prev_sum;
        println!("{:?}: {:?}, {:?}", gen + 1, sum, diff);
        prev_sum = sum;
    }

    // Diff stabilizes at "186"
    // So, since get_sum_at_gen(100) = 19437, then get_sum_at_gen(50bn) = ?

    // Hmmm... Not sure why but the correct answer is get_sum_at_gen(50bn + 1) ....
    println!(
        "Part 2: {:?}",
        get_sum_at_gen(50000000000 + 1, diff as u128, 1000, prev_sum as u128)
    );
}
