use regex::Regex;
use std::collections::HashMap;
use std::io;

#[path = "./day16.rs"]
mod day16;

fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return String::from(input.trim());
}

// NOTE: On both parts, I have deassembled the input program on paper and figured out the answer using calculator and Wolfram Alpha
// Line 2-11 is a function takes in input at reg[1] and outputs at reg[0]; reg[0] <- sum_of_factors(reg[1])
// Exit is at Line 16; reg[0] initial value acts as a jump to Line 17-26 (Part 1 input) or Line 17-35 (Part 2 input)
// Part 1: sum_of_factors(943) = 1008
// Part 2: sum_of_factors(110534976) = 11534976
pub fn main() {
    let re_ip = Regex::new(r"^\#ip (\d+)$").unwrap();
    let re_instruction = Regex::new(r"^([a-z]+) (\d+) (\d+) (\d+)$").unwrap();

    let mut registers = vec![0, 0, 0, 0, 0, 0];
    let mut pc_reg_index = 0;
    let mut pc = 0;
    let mut program: Vec<day16::Instructions> = vec![];

    let mut fn_name_mapping: HashMap<
        String,
        (usize, fn(&day16::Instructions, &Vec<usize>) -> Vec<usize>),
    > = HashMap::new();
    {
        fn_name_mapping.insert("addr".to_string(), (0, day16::do_addr));
        fn_name_mapping.insert("addi".to_string(), (1, day16::do_addi));
        fn_name_mapping.insert("mulr".to_string(), (2, day16::do_mulr));
        fn_name_mapping.insert("muli".to_string(), (3, day16::do_muli));
        fn_name_mapping.insert("banr".to_string(), (4, day16::do_banr));
        fn_name_mapping.insert("bani".to_string(), (5, day16::do_bani));
        fn_name_mapping.insert("borr".to_string(), (6, day16::do_borr));
        fn_name_mapping.insert("bori".to_string(), (7, day16::do_bori));
        fn_name_mapping.insert("setr".to_string(), (8, day16::do_setr));
        fn_name_mapping.insert("seti".to_string(), (9, day16::do_seti));
        fn_name_mapping.insert("gtir".to_string(), (10, day16::do_gtir));
        fn_name_mapping.insert("gtri".to_string(), (11, day16::do_gtri));
        fn_name_mapping.insert("gtrr".to_string(), (12, day16::do_gtrr));
        fn_name_mapping.insert("eqir".to_string(), (13, day16::do_eqir));
        fn_name_mapping.insert("eqri".to_string(), (14, day16::do_eqri));
        fn_name_mapping.insert("eqrr".to_string(), (15, day16::do_eqrr));
    }
    let opcode_name_mapping = vec![
        "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir",
        "gtri", "gtrr", "eqir", "eqri", "eqrr",
    ];

    loop {
        let input = read_stdin();
        if input == "" {
            break;
        }

        if re_ip.is_match(input.as_str()) {
            let caps_ip = re_ip.captures(input.as_str()).unwrap();
            let ip_reg: usize = caps_ip.get(1).unwrap().as_str().parse().unwrap();
            pc_reg_index = ip_reg;
        } else {
            let caps_instruction = re_instruction.captures(input.as_str()).unwrap();
            let instruction_str = String::from(caps_instruction.get(1).unwrap().as_str());
            let instruction_a: usize = caps_instruction.get(2).unwrap().as_str().parse().unwrap();
            let instruction_b: usize = caps_instruction.get(3).unwrap().as_str().parse().unwrap();
            let instruction_c: usize = caps_instruction.get(4).unwrap().as_str().parse().unwrap();
            let (opcode, _) = fn_name_mapping.get(&instruction_str).unwrap();

            program.push(day16::Instructions(
                *opcode,
                instruction_a,
                instruction_b,
                instruction_c,
            ));
        }
    }

    let mut t = 0;

    // Part 1 program
    loop {
        let instruction = &program[pc];
        let opcode = instruction.0;
        let opcode_name = opcode_name_mapping[opcode];
        let (_, opcode_fn) = fn_name_mapping.get(opcode_name).unwrap();

        let new_registers = opcode_fn(&instruction, &registers);
        registers = new_registers;

        pc = registers[pc_reg_index];
        pc += 1;

        if pc >= program.len() {
            break;
        }
        registers[pc_reg_index] = pc;

        println!("[{}] {}, {:?}, {:?}", t, pc, instruction, registers);
        t += 1;

        // ._. TODO: Solution not working, need to disassemble the input, might as well create a new file to solve it...
        if t > 1000 {
            break;
        }
    }

    println!("Part 1: {}", registers[0]);
}
