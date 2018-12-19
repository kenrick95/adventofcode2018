use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

#[derive(Debug)]
pub struct Instructions(
  pub /* opcode */ usize,
  pub /* A */ usize,
  pub /* B */ usize,
  pub /* C: output */ usize,
);

struct Case {
  registers_before: Vec<usize>,
  registers_after: Vec<usize>,
  instructions: Instructions,
}

fn read_stdin() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  return String::from(input.trim());
}

fn parse_numbers(caps: regex::Captures) -> Vec<usize> {
  let mut result: Vec<usize> = Vec::new();
  result.push(caps.get(1).unwrap().as_str().parse().unwrap());
  result.push(caps.get(2).unwrap().as_str().parse().unwrap());
  result.push(caps.get(3).unwrap().as_str().parse().unwrap());
  result.push(caps.get(4).unwrap().as_str().parse().unwrap());
  return result;
}

fn get_possible_functions(
  case: &Case,
) -> Vec<(String, fn(&Instructions, &Vec<usize>) -> Vec<usize>)> {
  let functions: Vec<(String, fn(&Instructions, &Vec<usize>) -> Vec<usize>)> = vec![
    (String::from("addr"), do_addr),
    (String::from("addi"), do_addi),
    (String::from("mulr"), do_mulr),
    (String::from("muli"), do_muli),
    (String::from("banr"), do_banr),
    (String::from("bani"), do_bani),
    (String::from("borr"), do_borr),
    (String::from("bori"), do_bori),
    (String::from("setr"), do_setr),
    (String::from("seti"), do_seti),
    (String::from("gtir"), do_gtir),
    (String::from("gtri"), do_gtri),
    (String::from("gtrr"), do_gtrr),
    (String::from("eqir"), do_eqir),
    (String::from("eqri"), do_eqri),
    (String::from("eqrr"), do_eqrr),
  ];
  let mut possible_functions = vec![];

  for item in functions {
    let (name, function) = item;
    if check_case(case, function) {
      possible_functions.push((name, function));
    }
  }

  return possible_functions;
}

fn check_case<F>(case: &Case, operation: F) -> bool
where
  F: Fn(&Instructions, &Vec<usize>) -> Vec<usize>,
{
  return operation(&case.instructions, &case.registers_before) == case.registers_after;
}

// Add register: reg[C] <- reg[A] + reg[B]
pub fn do_addr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] + registers[instructions.2];
  return result;
}

// Add immediate: reg[C] <- reg[A] + B
pub fn do_addi(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] + instructions.2;
  return result;
}

// Multiply register: reg[C] <- reg[A] * reg[B]
pub fn do_mulr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] * registers[instructions.2];
  return result;
}

// Multiply immediate: reg[C] <- reg[A] * B
pub fn do_muli(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] * instructions.2;
  return result;
}

// Bitwise AND register: reg[C] <- reg[A] & reg[B]
pub fn do_banr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] & registers[instructions.2];
  return result;
}

// Bitwise AND immediate: reg[C] <- reg[A]  & B
pub fn do_bani(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] & instructions.2;
  return result;
}

// Bitwise OR register: reg[C] <- reg[A] | reg[B]
pub fn do_borr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] | registers[instructions.2];
  return result;
}

// Bitwise OR immediate: reg[C] <- reg[A] | B
pub fn do_bori(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] | instructions.2;
  return result;
}

// Set register: reg[C] <- reg[A]
pub fn do_setr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1];
  return result;
}

// Set immediate: reg[C] <- A
pub fn do_seti(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = instructions.1;
  return result;
}

// greater-than immediate/register: reg[C] <- A > reg[B] ? 1 : 0
pub fn do_gtir(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  if instructions.1 > registers[instructions.2] {
    result[instructions.3] = 1;
  } else {
    result[instructions.3] = 0;
  }
  return result;
}

// greater-than register/immediate: reg[C] <- reg[A] > B ? 1 : 0
pub fn do_gtri(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  if registers[instructions.1] > instructions.2 {
    result[instructions.3] = 1;
  } else {
    result[instructions.3] = 0;
  }
  return result;
}

// greater-than register/register: reg[C] <- reg[A] > reg[B] ? 1 : 0
pub fn do_gtrr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  if registers[instructions.1] > registers[instructions.2] {
    result[instructions.3] = 1;
  } else {
    result[instructions.3] = 0;
  }
  return result;
}

// equal immediate/register: reg[C] <- A > reg[B] ? 1 : 0
pub fn do_eqir(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  if instructions.1 == registers[instructions.2] {
    result[instructions.3] = 1;
  } else {
    result[instructions.3] = 0;
  }
  return result;
}

// equal register/immediate: reg[C] <- reg[A] == B ? 1 : 0
pub fn do_eqri(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  if registers[instructions.1] == instructions.2 {
    result[instructions.3] = 1;
  } else {
    result[instructions.3] = 0;
  }
  return result;
}

// equal register/register: reg[C] <- reg[A] == reg[B] ? 1 : 0
pub fn do_eqrr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  if registers[instructions.1] == registers[instructions.2] {
    result[instructions.3] = 1;
  } else {
    result[instructions.3] = 0;
  }
  return result;
}

pub fn main() {
  let re_reg_before = Regex::new(r"^Before:\s+\[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();
  let re_reg_after = Regex::new(r"^After:\s+\[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();
  let re_reg_instructions = Regex::new(r"^(\d+) (\d+) (\d+) (\d+)$").unwrap();
  let mut opcode_fn_mapping: HashMap<
    usize,
    Vec<(String, fn(&Instructions, &Vec<usize>) -> Vec<usize>)>,
  > = HashMap::new();

  let mut cases = Vec::new();

  let mut break_count = 0;
  loop {
    let input = read_stdin();
    if input == "" {
      break_count += 1;
      if break_count > 2 {
        break;
      }
      continue;
    }
    break_count = 0;
    let caps_reg_before = re_reg_before.captures(input.as_str()).unwrap();
    let registers_before = parse_numbers(caps_reg_before);

    let input_instructions = read_stdin();
    let caps_instructions = re_reg_instructions
      .captures(input_instructions.as_str())
      .unwrap();
    let instructions = parse_numbers(caps_instructions);

    let input_after = read_stdin();
    let caps_reg_after = re_reg_after.captures(input_after.as_str()).unwrap();
    let registers_after = parse_numbers(caps_reg_after);

    cases.push(Case {
      registers_before,
      registers_after,
      instructions: Instructions(
        instructions[0],
        instructions[1],
        instructions[2],
        instructions[3],
      ),
    });
  }

  {
    let mut p1_count = 0;
    for case in cases {
      let possible_functions = get_possible_functions(&case);
      println!("possible_functions {:?}", possible_functions.len());
      if possible_functions.len() >= 3 {
        p1_count += 1;
      }
      let opcode = case.instructions.0;

      let current_possible_functions = opcode_fn_mapping
        .get(&opcode)
        .unwrap_or(&possible_functions);
      let mut intersections = vec![];
      {
        for item in current_possible_functions {
          let (c_name, _c_function) = item;
          if possible_functions.iter().any(|p_item| {
            let (p_name, _p_function) = p_item;
            return p_name == c_name;
          }) {
            intersections.push(item.clone());
          }
        }
      }
      opcode_fn_mapping.insert(opcode, intersections);
    }

    println!("Part 1: {:?}", p1_count);
  }

  // Initially ...
  // opcode 11 --> ["eqrr"]
  // opcode 10 --> ["addr", "addi", "seti"]
  // opcode 1 --> ["banr", "bani", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"]
  // opcode 3 --> ["eqri", "eqrr"]
  // opcode 8 --> ["addr", "seti"]
  // opcode 4 --> ["banr", "bani", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"]
  // opcode 5 --> ["muli", "banr", "bani", "bori", "setr", "eqir", "eqri"]
  // opcode 13 --> ["eqir", "eqri", "eqrr"]
  // opcode 7 --> ["gtri", "eqir", "eqrr"]
  // opcode 2 --> ["addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr"]
  // opcode 12 --> ["gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"]
  // opcode 9 --> ["muli", "banr", "bani", "gtir", "gtrr", "eqir", "eqri", "eqrr"]
  // opcode 15 --> ["gtri", "gtrr", "eqir", "eqri"]
  // opcode 0 --> ["addr", "borr", "bori", "setr", "gtrr"]
  // opcode 6 --> ["bani", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"]
  // opcode 14 --> ["setr", "gtir"]
  {
    // Resolve all the possibilities ... Remove functions that we're certain from other places, then check certainy again
    let mut certain_opcodes: HashSet<usize> = HashSet::new();
    for (k, v) in opcode_fn_mapping.iter() {
      let t: Vec<String> = v.iter().map(|i| (i.clone()).0).collect();
      println!("opcode {:?} --> {:?}", k, t);
      if t.len() == 1 {
        // This number is associated with 1 function, we're certain
        certain_opcodes.insert(*k);
      }
    }
    let mut i = 0;
    loop {
      let mut certain_fn_names: HashSet<String> = HashSet::new();
      certain_opcodes.iter().for_each(|op| {
        opcode_fn_mapping.get(&op).unwrap().iter().for_each(|item| {
          let (name, _fn) = item;
          certain_fn_names.insert(name.to_string());
        });
      });

      for (_k, v) in opcode_fn_mapping.iter_mut() {
        let mut left: Vec<(String, fn(&Instructions, &Vec<usize>) -> Vec<usize>)> = vec![];
        if v.len() == 1 {
          continue;
        }
        for x in v.iter() {
          let (name, func) = x.clone();
          if !certain_fn_names.contains(&name) {
            left.push((name, func));
          }
        }
        *v = left;
      }

      let mut all_ok = true;
      for (k, v) in opcode_fn_mapping.iter() {
        let t: Vec<String> = v.iter().map(|j| (j.clone()).0).collect();
        println!("[{}] opcode {:?} --> {:?}", i, k, t);

        if t.len() == 1 {
          certain_opcodes.insert(*k);
        } else {
          all_ok = false;
        }
      }
      if all_ok {
        break;
      }
      i += 1;
    }
  }
  // This results in something like:
  // [10] opcode 11 --> ["eqrr"]
  // [10] opcode 10 --> ["addi"]
  // [10] opcode 1 --> ["seti"]
  // [10] opcode 3 --> ["eqri"]
  // [10] opcode 8 --> ["addr"]
  // [10] opcode 4 --> ["banr"]
  // [10] opcode 5 --> ["bori"]
  // [10] opcode 13 --> ["eqir"]
  // [10] opcode 7 --> ["gtri"]
  // [10] opcode 2 --> ["mulr"]
  // [10] opcode 12 --> ["gtir"]
  // [10] opcode 9 --> ["muli"]
  // [10] opcode 15 --> ["gtrr"]
  // [10] opcode 0 --> ["borr"]
  // [10] opcode 6 --> ["bani"]
  // [10] opcode 14 --> ["setr"]

  // Part 2; WIP
  {
    let mut p2_registers = vec![0, 0, 0, 0];
    loop {
      let input = read_stdin();
      if input == "" {
        break_count += 1;
        if break_count > 2 {
          break;
        }
        continue;
      }
      break_count = 0;

      let caps_instructions = re_reg_instructions.captures(input.as_str()).unwrap();
      let instructions_vec = parse_numbers(caps_instructions);
      let instructions = Instructions(
        instructions_vec[0],
        instructions_vec[1],
        instructions_vec[2],
        instructions_vec[3],
      );
      // Get opcode from instructions
      let opcode = instructions.0;
      // Get function from opcode
      let possible_fns = opcode_fn_mapping.get(&opcode).unwrap();
      let (_instruction_fn_name, instruction_fn) = (possible_fns[0]).clone();
      // Run function
      let new_registers = instruction_fn(&instructions, &p2_registers);
      p2_registers = new_registers;
    }
    println!("Part 2: {:?}", p2_registers);
  }
}
