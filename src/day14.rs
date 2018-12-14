use std::io;

fn digitize(number: usize) -> Vec<u8> {
  let mut res = Vec::new();

  let mut rem = number;
  if rem == 0 {
    res.push(rem as u8);
  }
  while rem > 0 {
    res.push((rem % 10) as u8);
    rem = rem / 10;
  }

  res.reverse();
  return res;
}

// Day 14: Basically brute force, not sure if there's any better solution
pub fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  input = String::from(input.trim());
  let input_num: usize = input.parse().unwrap();

  let mut recipes: Vec<u8> = vec![3, 7];
  let mut current_elf_process_index = (0, 1);

  let skip_part_2 = false;
  let mut found_part_2 = false;

  let mut t = 0;
  while {
    if skip_part_2 {
      // On part 1, we're pretty sure to stop before here
      t < input_num + 10
    } else {
      // On part 2, we're not sure when to stop
      true
    }
  } {
    let (i, j) = current_elf_process_index;
    let e1 = recipes[i] as usize;
    let e2 = recipes[j] as usize;
    recipes.append(&mut digitize(e1 + e2));
    current_elf_process_index = ((i + e1 + 1) % recipes.len(), (j + e2 + 1) % recipes.len());

    {
      // Part 2: collect everything into a very long string
      // Then substr
      let len = recipes.len();
      if len >= 10 && !found_part_2 {
        let final_slice = &recipes[len - 10..];
        let full_chars = (*final_slice).iter().map(|d| (*d + 48) as char);
        let full_chars_str: String = full_chars.into_iter().collect();
        // println!("{:?}", full_chars_str);
        if full_chars_str.contains(input.as_str()) {
          found_part_2 = true;
          // only stop the loop if final 10 chars of `recipes` == `input`
          println!("Part 2: {:?}", len - 10 + full_chars_str.find(input.as_str()).unwrap());
        }
      }
    }

    // Part 1 can break the loop if `recipes.len` > input + 10
    if recipes.len() > input_num + 10 && (skip_part_2 || found_part_2) {
      break;
    }

    // println!(
    //   "[{}] (i, j): {:?}; {:?}",
    //   t, current_elf_process_index, recipes
    // );
    t += 1;
  }
  // println!("End (i, j): {:?}; {:?}", current_elf_process_index, recipes);
  {
    let ans = &recipes[input_num..input_num + 10];
    // Here to convert number (u8) into ASCII character. 48 (char) is ASCII's "0"
    let ans_chars = (*ans).iter().map(|d| (*d + 48) as char);
    let ans_str: String = ans_chars.into_iter().collect();
    println!("Part 1: {:?}", ans_str);
  }
}
