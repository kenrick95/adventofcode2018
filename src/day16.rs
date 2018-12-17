use regex::Regex;
// use std::cmp::PartialEq;
use std::collections::HashMap;
use std::io;

// impl<'r, 's> PartialEq for fn(&'r Instructions, &'s Vec<usize>) -> Vec<usize> {
//   fn eq<F>(&self, other: &F) -> bool
//   where
//     F: Fn(Instructions, &Vec<usize>) -> Vec<usize>,
//   {
//     self as usize == other as usize
//   }
// }

struct Instructions(
  /* opcode */ usize,
  /* A */ usize,
  /* B */ usize,
  /* C: output */ usize,
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

fn get_possible_functions(case: &Case) -> Vec<fn(&Instructions, &Vec<usize>) -> Vec<usize>> {
  let functions: Vec<fn(&Instructions, &Vec<usize>) -> Vec<usize>> = vec![
    do_addr, do_addi, do_mulr, do_muli, do_banr, do_bani, do_borr, do_bori, do_setr, do_seti,
    do_gtir, do_gtri, do_gtrr, do_eqir, do_eqri, do_eqrr,
  ];
  let mut possible_functions = vec![];

  for function in functions {
    if check_case(case, function) {
      possible_functions.push(function);
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
fn do_addr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] + registers[instructions.2];
  return result;
}

// Add immediate: reg[C] <- reg[A] + B
fn do_addi(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] + instructions.2;
  return result;
}

// Multiply register: reg[C] <- reg[A] * reg[B]
fn do_mulr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] * registers[instructions.2];
  return result;
}

// Multiply immediate: reg[C] <- reg[A] * B
fn do_muli(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] * instructions.2;
  return result;
}

// Bitwise AND register: reg[C] <- reg[A] & reg[B]
fn do_banr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] & registers[instructions.2];
  return result;
}

// Bitwise AND immediate: reg[C] <- reg[A]  & B
fn do_bani(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] & instructions.2;
  return result;
}

// Bitwise OR register: reg[C] <- reg[A] | reg[B]
fn do_borr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] | registers[instructions.2];
  return result;
}

// Bitwise OR immediate: reg[C] <- reg[A] | B
fn do_bori(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1] | instructions.2;
  return result;
}

// Set register: reg[C] <- reg[A]
fn do_setr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = registers[instructions.1];
  return result;
}

// Set immediate: reg[C] <- A
fn do_seti(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  result[instructions.3] = instructions.1;
  return result;
}

// greater-than immediate/register: reg[C] <- A > reg[B] ? 1 : 0
fn do_gtir(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  if instructions.1 > registers[instructions.2] {
    result[instructions.3] = 1;
  } else {
    result[instructions.3] = 0;
  }
  return result;
}

// greater-than register/immediate: reg[C] <- reg[A] > B ? 1 : 0
fn do_gtri(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  if registers[instructions.1] > instructions.2 {
    result[instructions.3] = 1;
  } else {
    result[instructions.3] = 0;
  }
  return result;
}

// greater-than register/register: reg[C] <- reg[A] > reg[B] ? 1 : 0
fn do_gtrr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  if registers[instructions.1] > registers[instructions.2] {
    result[instructions.3] = 1;
  } else {
    result[instructions.3] = 0;
  }
  return result;
}

// equal immediate/register: reg[C] <- A > reg[B] ? 1 : 0
fn do_eqir(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  if instructions.1 == registers[instructions.2] {
    result[instructions.3] = 1;
  } else {
    result[instructions.3] = 0;
  }
  return result;
}

// equal register/immediate: reg[C] <- reg[A] > B ? 1 : 0
fn do_eqri(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
  let mut result = registers.clone();
  if registers[instructions.1] == instructions.2 {
    result[instructions.3] = 1;
  } else {
    result[instructions.3] = 0;
  }
  return result;
}

// equal register/register: reg[C] <- reg[A] > reg[B] ? 1 : 0
fn do_eqrr(instructions: &Instructions, registers: &Vec<usize>) -> Vec<usize> {
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
  let mut opcode_fn_mapping: HashMap<usize, Vec<fn(&Instructions, &Vec<usize>) -> Vec<usize>>> =
    HashMap::new();

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
      // TODO: Do intersection of current_possible_functions and possible_functions
      // let mut intersections = vec![];
      // {
      //   for func in current_possible_functions {
      //     if possible_functions.contains(&*func) { // <-- This has error since PartialEq can't work without impl; but with impl it said said it has conflicting implementation
      //       intersections.push(*func);
      //     }
      //   }
      // }
      // opcode_fn_mapping.insert(opcode, intersections);
    }

    println!("Part 1: {:?}", p1_count);
  }

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
      let instructions = (
        instructions_vec[0],
        instructions_vec[1],
        instructions_vec[2],
        instructions_vec[3],
      );
      // Get opcode from instructions
      // Get function from opcode
      // Run function
    }
  }
}
