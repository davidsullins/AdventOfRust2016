// advent12.rs
// assembly language

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let instructions: Vec<_> =
        stdin.lock().lines().map(|l| l.expect("Failed to read line")).collect();

    // part 1
    let program_state = execute_until_halt(&instructions);
    println!("part 1 a={}", program_state.registers[0]);

    // part 2
    let program_state = execute_until_halt2(&instructions);
    println!("part 2 a={}", program_state.registers[0]);
}

#[derive(Debug)]
struct ProgramState {
    registers: [i32; 4],
    program_counter: usize,
}

impl ProgramState {
    fn new() -> ProgramState {
        ProgramState {
            registers: [0; 4],
            program_counter: 0,
        }
    }

    fn parse_instruction(&mut self, instr: &str) {
        lazy_static! {
            static ref RE_CPY_REG: Regex = Regex::new(r"^cpy ([a-d]) ([a-d])").unwrap();
            static ref RE_CPY_IMM: Regex = Regex::new(r"^cpy ([-\d]+) ([a-d])").unwrap();
            static ref RE_INC: Regex = Regex::new(r"^inc ([a-d])").unwrap();
            static ref RE_DEC: Regex = Regex::new(r"^dec ([a-d])").unwrap();
            static ref RE_JNZ_REG: Regex = Regex::new(r"^jnz ([a-d]) ([-\d]+)").unwrap();
            static ref RE_JNZ_IMM: Regex = Regex::new(r"^jnz ([-\d]+) ([-\d]+)").unwrap();
        }

        if let Some(caps) = RE_CPY_REG.captures(instr) {
            let src_reg_idx = idx_from_reg(&caps[1]);
            let dest_reg_idx = idx_from_reg(&caps[2]);
            self.registers[dest_reg_idx] = self.registers[src_reg_idx];
            self.program_counter += 1;
        } else if let Some(caps) = RE_CPY_IMM.captures(instr) {
            let immediate: i32 = caps[1].parse().unwrap();
            let reg_idx = idx_from_reg(&caps[2]);
            self.registers[reg_idx] = immediate;
            self.program_counter += 1;
        } else if let Some(caps) = RE_INC.captures(instr) {
            let reg_idx = idx_from_reg(&caps[1]);
            self.registers[reg_idx] += 1;
            self.program_counter += 1;
        } else if let Some(caps) = RE_DEC.captures(instr) {
            let reg_idx = idx_from_reg(&caps[1]);
            self.registers[reg_idx] -= 1;
            self.program_counter += 1;
        } else if let Some(caps) = RE_JNZ_REG.captures(instr) {
            let reg_idx = idx_from_reg(&caps[1]);
            let offset: isize = caps[2].parse().unwrap();
            if self.registers[reg_idx] != 0 {
                self.program_counter = (self.program_counter as isize + offset) as usize;
            } else {
                self.program_counter += 1;
            };
        } else if let Some(caps) = RE_JNZ_IMM.captures(instr) {
            let immediate: i32 = caps[1].parse().unwrap();
            let offset: isize = caps[2].parse().unwrap();
            if immediate != 0 {
                self.program_counter = (self.program_counter as isize + offset) as usize;
            } else {
                self.program_counter += 1;
            };
        } else {
            panic!("unknown instruction {}", instr);
        }
    }
}

fn idx_from_reg(reg: &str) -> usize {
    reg.chars().next().unwrap() as usize - 'a' as usize
}

fn execute_until_halt(instructions: &[String]) -> ProgramState {
    let mut program_state = ProgramState::new();

    while program_state.program_counter < instructions.len() {
        let next_instr = &instructions[program_state.program_counter];
        program_state.parse_instruction(next_instr);
    }

    program_state
}

// ///////
// Part 2
fn execute_until_halt2(instructions: &[String]) -> ProgramState {
    let mut program_state = ProgramState::new();
    program_state.registers[2] = 1;

    while program_state.program_counter < instructions.len() {
        let next_instr = &instructions[program_state.program_counter];
        program_state.parse_instruction(next_instr);
    }

    program_state
}

// //////
// Tests
#[test]
fn test_parse_instruction() {
    let mut program_state = ProgramState::new();

    program_state.parse_instruction("cpy 41 a");
    assert_eq!(41, program_state.registers[0]);
    program_state.parse_instruction("cpy -37 d");
    assert_eq!(-37, program_state.registers[3]);
    program_state.parse_instruction("inc a");
    assert_eq!(42, program_state.registers[0]);
    program_state.parse_instruction("cpy a b");
    assert_eq!(42, program_state.registers[1]);
    program_state.parse_instruction("dec b");
    assert_eq!(41, program_state.registers[1]);
    assert_eq!(5, program_state.program_counter);
    program_state.parse_instruction("jnz c 2");
    assert_eq!(6, program_state.program_counter);
    program_state.parse_instruction("jnz b 2");
    assert_eq!(8, program_state.program_counter);
    program_state.parse_instruction("jnz a -1");
    assert_eq!(7, program_state.program_counter);
    program_state.parse_instruction("jnz 0 -1");
    assert_eq!(8, program_state.program_counter);
    program_state.parse_instruction("jnz 1 -1");
    assert_eq!(7, program_state.program_counter);
}

#[test]
fn test_execute_until_halt() {
    let mut instructions = Vec::new();

    instructions.push("cpy 41 a".to_string());
    instructions.push("inc a".to_string());
    instructions.push("inc a".to_string());
    instructions.push("dec a".to_string());
    instructions.push("jnz a 2".to_string());
    instructions.push("dec a".to_string());

    let program_state = execute_until_halt(&instructions);
    assert_eq!(42, program_state.registers[0]);
    assert_eq!(6, program_state.program_counter);
}
