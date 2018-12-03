use std::io;

// https://repl.it/@kenrick95/Aoc2018Day2Part2

fn main() {
    let mut inputs = Vec::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());
        if input == "" {
            break;
        }
        inputs.push(input);
    }
    let mut i = 0;
    let mut j;
    while i < inputs.len() {
        j = i + 1;
        while j < inputs.len() {
            // Do string comparison
            let str1: Vec<char> = inputs[i].chars().collect();
            let str2: Vec<char> = inputs[j].chars().collect();

            let mut k = 0;
            let mut diff_count = 0;

            while k < str1.len() {
                if str1[k] != str2[k] {
                    diff_count += 1;
                }
                if diff_count > 1 {
                    break;
                }
                k += 1;
            }

            if diff_count == 1 {
                // Found the answer
                let mut ans: Vec<char> = Vec::new();
                let mut l = 0;
                while l < str1.len() {
                    if str1[l] == str2[l] {
                        ans.push(str1[l]);
                    }
                    l += 1;
                }
                let ans_str: String = ans.into_iter().collect();
                println!("{:?}", ans_str);
                break;
            }

            j += 1;
        }
        i += 1;
    }
}
