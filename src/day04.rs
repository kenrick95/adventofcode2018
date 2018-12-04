use regex::Regex;
use std::collections::HashMap;
use std::io;

pub fn main() {
    let re = Regex::new(r"^\[(\d+)-(\d+)-(\d+)\s(\d+):(\d+)\]\s([a-zA-Z0-9 #]+)$").unwrap();
    let re_guard = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();

    /**
     * map_of(guard_id: i32, array_of(usize: asleep_count))
     */
    let mut sleep_map: HashMap<i32, Vec<usize>> = HashMap::new();

    let mut inputs: Vec<(usize, usize, usize, usize, usize, String)> = Vec::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());
        if input == "" {
            break;
        }
        // [1518-03-31 00:26] falls asleep
        let caps = re.captures(input.as_str()).unwrap();
        // caps: Captures({0: Some("[1518-03-31 00:26] falls asleep"), 1: Some("1518"), 2: Some("03"), 3: Some("31"), 4: Some("00"), 5: Some("26"), 6: Some("falls asleep")})

        let year: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let month: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let date: usize = caps.get(3).unwrap().as_str().parse().unwrap();
        let hour: usize = caps.get(4).unwrap().as_str().parse().unwrap();
        let minute: usize = caps.get(5).unwrap().as_str().parse().unwrap();
        let action = String::from(caps.get(6).unwrap().as_str());

        inputs.push((year, month, date, hour, minute, action));
    }
    inputs.sort();
    // inputs is now sorted by time
    // println!("{:?}", inputs);

    let mut empty_vec: Vec<usize> = Vec::new();
    let mut i: usize = 0;
    while i < 60 {
        empty_vec.push(0);
        i += 1;
    }

    let mut guard_id = -1;
    let mut start_sleep_time = 0;
    for input in inputs {
        let (year, month, date, hour, minute, action) = input;
        // Parse action
        if re_guard.is_match(action.as_str()) {
            let caps_guard = re_guard.captures(action.as_str()).unwrap();
            guard_id = caps_guard.get(1).unwrap().as_str().parse().unwrap();

            // init guard's sleep map if not available
            if !sleep_map.contains_key(&guard_id) {
                sleep_map.insert(guard_id, empty_vec.clone());
            }
        } else if action == "falls asleep" {
            start_sleep_time = minute;
        } else if action == "wakes up" {
            let mut guard_sleep_map = sleep_map.get(&guard_id).unwrap().clone();
            let mut i: usize = start_sleep_time;
            while i < minute {
                guard_sleep_map[i] += 1;
                i += 1;
            }
            sleep_map.insert(guard_id, guard_sleep_map);
        }
    }
    // println!("{:?}", sleep_map);

    let mut max_sleep_guard_id = 0;
    let mut max_sleep_guard_sum = 0;
    for (guard_id, map) in sleep_map.iter() {
        let sum = map.iter().sum();
        // println!("guard_id {}, {:?}, sum {:?}", guard_id, map, sum);
        if sum > max_sleep_guard_sum {
            max_sleep_guard_sum = sum;
            max_sleep_guard_id = *guard_id;
        }
    }
    // println!("{:?} {:?}", max_sleep_guard_id, max_sleep_guard_sum);

    let mut max_key = 0;
    {
        let map = sleep_map.get(&max_sleep_guard_id).unwrap();
        let mut i = 0;
        let mut max_val = 0;
        while i < map.len() {
            if map[i] > max_val {
                max_val = map[i];
                max_key = i;
            }
            i += 1;
        }
    }
    println!("Part 1: {}", max_key * (max_sleep_guard_id as usize));


    let mut p2_max_sleep_guard_id = 0;
    let mut p2_max_sleep_guard_value = 0;
    let mut p2_max_sleep_guard_minute = 0;
    for (guard_id, map) in sleep_map.iter() {

        let mut i = 0;
        let mut max_val = 0;
        let mut max_key = 0;
        while i < map.len() {
            if map[i] > max_val {
                max_val = map[i];
                max_key = i;
            }
            i += 1;
        }
        if max_val > p2_max_sleep_guard_value {
            p2_max_sleep_guard_value = max_val;
            p2_max_sleep_guard_minute = max_key;
            p2_max_sleep_guard_id = *guard_id;
        }
    }
    println!("Part 1: {}", p2_max_sleep_guard_minute * (p2_max_sleep_guard_id as usize));
}
