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
    let instructions: Vec<_> = stdin.lock()
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .map(|l| parse_instr(&l))
        .collect();

    // part 1
    let program_state = execute_until_halt(&instructions);
    println!("part 1 a={}", program_state.registers[0]);

    // part 2
    let program_state = execute_until_halt2(&instructions);
    println!("part 2 a={}", program_state.registers[0]);
}

type RegIdx = usize;

enum Instr {
    Cpy(Arg, RegIdx),
    Inc(RegIdx),
    Dec(RegIdx),
    Jnz(Arg, Arg),
}

enum Arg {
    Reg(RegIdx),
    Imm(i32),
}

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

    fn execute_instr(&mut self, instr: &Instr) {
        match *instr {
            Instr::Jnz(ref val_arg, ref offset_arg) => {
                let val = self.val_from_arg(val_arg);
                let offset = self.val_from_arg(offset_arg);
                if val != 0 {
                    self.program_counter = (self.program_counter as i32 + offset) as usize;
                } else {
                    self.program_counter += 1;
                };
            }
            Instr::Dec(reg_idx) => {
                self.registers[reg_idx] -= 1;
                self.program_counter += 1;
            }
            Instr::Inc(reg_idx) => {
                self.registers[reg_idx] += 1;
                self.program_counter += 1;
            }
            Instr::Cpy(ref arg, reg_idx) => {
                let src_val = self.val_from_arg(arg);
                self.registers[reg_idx] = src_val;
                self.program_counter += 1;
            }
        }
    }

    fn val_from_arg(&self, arg: &Arg) -> i32 {
        match *arg {
            Arg::Reg(reg_idx) => self.registers[reg_idx],
            Arg::Imm(imm) => imm,
        }
    }
}

fn parse_instr(instr: &str) -> Instr {
    lazy_static! {
        static ref RE_CPY: Regex =
            Regex::new(r"^cpy (?:([a-d])|([-\d]+)) ([a-d])").unwrap();
        static ref RE_INC: Regex = Regex::new(r"^inc ([a-d])").unwrap();
        static ref RE_DEC: Regex = Regex::new(r"^dec ([a-d])").unwrap();
        static ref RE_JNZ: Regex =
            Regex::new(r"^jnz (?:([a-d])|([-\d]+)) (?:([a-d])|([-\d]+))").unwrap();
    }

    if let Some(caps) = RE_JNZ.captures(instr) {
        let val = parse_arg(caps.get(1), caps.get(2));
        let offset = parse_arg(caps.get(3), caps.get(4));
        Instr::Jnz(val, offset)
    } else if let Some(caps) = RE_DEC.captures(instr) {
        Instr::Dec(idx_from_reg(&caps[1]))
    } else if let Some(caps) = RE_INC.captures(instr) {
        Instr::Inc(idx_from_reg(&caps[1]))
    } else if let Some(caps) = RE_CPY.captures(instr) {
        let src_val = parse_arg(caps.get(1), caps.get(2));
        let dest_reg_idx = idx_from_reg(&caps[3]);
        Instr::Cpy(src_val, dest_reg_idx)
    } else {
        panic!("unknown instruction {}", instr);
    }
}

fn parse_arg(reg: Option<regex::Match>, imm: Option<regex::Match>) -> Arg {
    if let Some(imm_match) = imm {
        Arg::Imm(imm_match.as_str().parse().unwrap())
    } else {
        let src_reg_idx = idx_from_reg(reg.unwrap().as_str());
        Arg::Reg(src_reg_idx)
    }
}

fn idx_from_reg(reg: &str) -> usize {
    reg.chars().next().unwrap() as usize - 'a' as usize
}

fn execute_until_halt(instructions: &[Instr]) -> ProgramState {
    let mut program_state = ProgramState::new();

    while program_state.program_counter < instructions.len() {
        let next_instr = &instructions[program_state.program_counter];
        program_state.execute_instr(next_instr);
    }

    program_state
}

// ///////
// Part 2
fn execute_until_halt2(instructions: &[Instr]) -> ProgramState {
    let mut program_state = ProgramState::new();
    program_state.registers[2] = 1;

    while program_state.program_counter < instructions.len() {
        let next_instr = &instructions[program_state.program_counter];
        program_state.execute_instr(next_instr);
    }

    program_state
}

// //////
// Tests
#[test]
fn test_parse_instruction() {
    let mut program_state = ProgramState::new();

    program_state.execute_instr(&parse_instr("cpy 41 a"));
    assert_eq!(41, program_state.registers[0]);
    program_state.execute_instr(&parse_instr("cpy -37 d"));
    assert_eq!(-37, program_state.registers[3]);
    program_state.execute_instr(&parse_instr("inc a"));
    assert_eq!(42, program_state.registers[0]);
    program_state.execute_instr(&parse_instr("cpy a b"));
    assert_eq!(42, program_state.registers[1]);
    program_state.execute_instr(&parse_instr("dec b"));
    assert_eq!(41, program_state.registers[1]);
    assert_eq!(5, program_state.program_counter);
    program_state.execute_instr(&parse_instr("jnz c 2"));
    assert_eq!(6, program_state.program_counter);
    program_state.execute_instr(&parse_instr("jnz b 2"));
    assert_eq!(8, program_state.program_counter);
    program_state.execute_instr(&parse_instr("jnz a -1"));
    assert_eq!(7, program_state.program_counter);
    program_state.execute_instr(&parse_instr("jnz 0 -1"));
    assert_eq!(8, program_state.program_counter);
    program_state.execute_instr(&parse_instr("jnz 1 -1"));
    assert_eq!(7, program_state.program_counter);
}

#[test]
fn test_execute_until_halt() {
    let mut instructions = Vec::new();

    instructions.push(parse_instr("cpy 41 a"));
    instructions.push(parse_instr("inc a"));
    instructions.push(parse_instr("inc a"));
    instructions.push(parse_instr("dec a"));
    instructions.push(parse_instr("jnz a 2"));
    instructions.push(parse_instr("dec a"));

    let program_state = execute_until_halt(&instructions);
    assert_eq!(42, program_state.registers[0]);
    assert_eq!(6, program_state.program_counter);
}
