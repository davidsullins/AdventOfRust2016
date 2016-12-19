// advent10.rs
// parsing instructions for chip factory

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let mut factory = Factory::new();

    for line in stdin.lock().lines().map(|l| l.expect("Failed to read line")) {
        // part 1
        factory.parse_instruction(&line);
        // part 2
        factory.parse_instruction2(&line);
    }

    // part 1
    if let Some(bot_id) = factory.find_comparison(61, 17) {
        println!("part 1 bot ID {}", bot_id);
    } else {
        println!("part 1 bot not found!");
    }

    // part 2
    let product = factory.get_output_val(0) * factory.get_output_val(1) * factory.get_output_val(2);
    println!("part 2 output product {}", product);
}

#[derive(Debug, PartialEq, Clone)]
enum Chip {
    Unassigned,
    BotLow(usize),
    BotHigh(usize),
    Val(usize),
}

struct Factory {
    chip0: Vec<Chip>,
    chip1: Vec<Chip>,
    outputs: Vec<Chip>,
}

impl Factory {
    fn new() -> Factory {
        Factory {
            chip0: Vec::new(),
            chip1: Vec::new(),
            outputs: Vec::new(),
        }
    }

    // no need to parse outputs since they aren't used for the answer to part 1
    fn parse_instruction(&mut self, instr: &str) {
        lazy_static! {
            static ref RE_VAL: Regex = Regex::new(r"^value (\d+) goes to bot (\d+)").unwrap();
            static ref RE_BOT_LOW: Regex = Regex::new(r"^bot (\d+).+low to bot (\d+)").unwrap();
            static ref RE_BOT_HIGH: Regex = Regex::new(r"^bot (\d+).+high to bot (\d+)").unwrap();
        }
        if let Some(caps) = RE_VAL.captures(instr) {
            let val: usize = caps.at(1).unwrap().parse().unwrap();
            let bot_id: usize = caps.at(2).unwrap().parse().unwrap();
            self.give_chip_to_bot(bot_id, Chip::Val(val));
        } else {
            if let Some(caps) = RE_BOT_LOW.captures(instr) {
                let src_bot: usize = caps.at(1).unwrap().parse().unwrap();
                let dest_bot: usize = caps.at(2).unwrap().parse().unwrap();
                self.give_chip_to_bot(dest_bot, Chip::BotLow(src_bot));
            }
            if let Some(caps) = RE_BOT_HIGH.captures(instr) {
                let src_bot: usize = caps.at(1).unwrap().parse().unwrap();
                let dest_bot: usize = caps.at(2).unwrap().parse().unwrap();
                self.give_chip_to_bot(dest_bot, Chip::BotHigh(src_bot));
            }
        }
    }

    fn give_chip_to_bot(&mut self, bot_id: usize, chip: Chip) {
        if bot_id >= self.chip0.len() {
            self.chip0.resize(bot_id + 1, Chip::Unassigned);
            self.chip1.resize(bot_id + 1, Chip::Unassigned);
        }

        let chip0 = &mut self.chip0[bot_id];
        let chip1 = &mut self.chip1[bot_id];

        if *chip0 == Chip::Unassigned {
            *chip0 = chip;
        } else if *chip1 == Chip::Unassigned {
            *chip1 = chip;
        } else {
            panic!("Bot {} received {:?}, but was already holding {:?} and {:?}",
                   bot_id,
                   chip,
                   chip0,
                   chip1);
        }
    }

    fn get_chip_val(&mut self, bot_id: usize, chip0: bool) -> usize {
        let chip = if chip0 {
            self.chip0[bot_id].clone()
        } else {
            self.chip1[bot_id].clone()
        };

        let chip_val = match chip {
            Chip::Val(new_val) => new_val,
            Chip::BotLow(low_bot) => {
                std::cmp::min(self.get_chip_val(low_bot, false),
                              self.get_chip_val(low_bot, true))
            }
            Chip::BotHigh(high_bot) => {
                std::cmp::max(self.get_chip_val(high_bot, false),
                              self.get_chip_val(high_bot, true))
            }
            _ => panic!("Attempting to get uninitialized chip from bot {}", bot_id),
        };

        // not strictly necessary but should be more efficient if we cache the result
        if chip0 {
            self.chip0[bot_id] = Chip::Val(chip_val);
        } else {
            self.chip1[bot_id] = Chip::Val(chip_val);
        }

        chip_val
    }

    // find the bot that compares val0 and val1
    fn find_comparison(&mut self, val0: usize, val1: usize) -> Option<usize> {
        for bot_id in 0..self.chip0.len() {
            let chip_val0 = self.get_chip_val(bot_id, true);
            let chip_val1 = self.get_chip_val(bot_id, false);
            if chip_val0 == val0 && chip_val1 == val1 || (chip_val0 == val1 && chip_val1 == val0) {
                return Some(bot_id);
            }
        }

        None
    }

    // ///////
    // Part 2
    fn parse_instruction2(&mut self, instr: &str) {
        lazy_static! {
            static ref RE_OUT_LOW: Regex = Regex::new(r"^bot (\d+).+low to output (\d+)").unwrap();
            static ref RE_OUT_HIGH: Regex = Regex::new(r"^bot (\d+).+high to output (\d+)").unwrap();
        }
        if let Some(caps) = RE_OUT_LOW.captures(instr) {
            let bot: usize = caps.at(1).unwrap().parse().unwrap();
            let output: usize = caps.at(2).unwrap().parse().unwrap();
            self.assign_chip_to_output(output, Chip::BotLow(bot));
        }
        if let Some(caps) = RE_OUT_HIGH.captures(instr) {
            let bot: usize = caps.at(1).unwrap().parse().unwrap();
            let output: usize = caps.at(2).unwrap().parse().unwrap();
            self.assign_chip_to_output(output, Chip::BotHigh(bot));
        }
    }

    fn assign_chip_to_output(&mut self, output_id: usize, chip: Chip) {
        if output_id >= self.outputs.len() {
            self.outputs.resize(output_id + 1, Chip::Unassigned);
        }

        let output_chip = &mut self.outputs[output_id];

        if *output_chip == Chip::Unassigned {
            *output_chip = chip;
        } else {
            panic!("Output {} received {:?}, but was already holding {:?}",
                   output_id,
                   chip,
                   output_chip);
        }
    }

    fn get_output_val(&mut self, output_id: usize) -> usize {
        let chip = self.outputs[output_id].clone();

        match chip {
            Chip::BotLow(low_bot) => {
                std::cmp::min(self.get_chip_val(low_bot, false),
                              self.get_chip_val(low_bot, true))
            }
            Chip::BotHigh(high_bot) => {
                std::cmp::max(self.get_chip_val(high_bot, false),
                              self.get_chip_val(high_bot, true))
            }
            Chip::Val(new_val) => new_val,
            _ => panic!("Tried to read uninitialized output {}", output_id),
        }
    }
}

// //////
// Tests
#[test]
fn test_get_chip_val() {
    let mut b = Factory::new();
    b.give_chip_to_bot(3, Chip::Val(9));
    b.give_chip_to_bot(3, Chip::Val(10));
    b.give_chip_to_bot(1, Chip::BotHigh(3));
    b.give_chip_to_bot(5, Chip::BotLow(3));

    assert_eq!(9, b.get_chip_val(3, true));
    assert_eq!(10, b.get_chip_val(3, false));
    assert_eq!(10, b.get_chip_val(1, true));
    assert_eq!(9, b.get_chip_val(5, true));
}

#[test]
fn test_give_chip_to_bot() {
    let mut b = Factory::new();
    b.give_chip_to_bot(2, Chip::Val(5));
    b.give_chip_to_bot(1, Chip::BotLow(2));
    b.give_chip_to_bot(0, Chip::BotHigh(2));
    b.give_chip_to_bot(1, Chip::Val(3));
    b.give_chip_to_bot(0, Chip::BotHigh(1));
    b.give_chip_to_bot(2, Chip::Val(2));

    assert_eq!(Some(2), b.find_comparison(5, 2));
    assert_eq!(Some(2), b.find_comparison(2, 5));
}

#[test]
fn test_parse_instruction() {
    let mut b = Factory::new();
    b.parse_instruction("value 5 goes to bot 2");
    b.parse_instruction("bot 2 gives low to bot 1 and high to bot 0");
    b.parse_instruction("value 3 goes to bot 1");
    b.parse_instruction("bot 1 gives low to output 1 and high to bot 0");
    b.parse_instruction("bot 0 gives low to output 2 and high to output 0");
    b.parse_instruction("value 2 goes to bot 2");

    assert_eq!(Some(2), b.find_comparison(5, 2));
    assert_eq!(Some(2), b.find_comparison(2, 5));
}

// part 2
#[test]
fn test_parse_instruction2() {
    let mut b = Factory::new();
    b.parse_instruction("value 5 goes to bot 2");
    b.parse_instruction("bot 2 gives low to bot 1 and high to bot 0");
    b.parse_instruction("value 3 goes to bot 1");
    b.parse_instruction("bot 1 gives low to output 1 and high to bot 0");
    b.parse_instruction("bot 0 gives low to output 2 and high to output 0");
    b.parse_instruction("value 2 goes to bot 2");
    b.parse_instruction2("value 5 goes to bot 2");
    b.parse_instruction2("bot 2 gives low to bot 1 and high to bot 0");
    b.parse_instruction2("value 3 goes to bot 1");
    b.parse_instruction2("bot 1 gives low to output 1 and high to bot 0");
    b.parse_instruction2("bot 0 gives low to output 2 and high to output 0");
    b.parse_instruction2("value 2 goes to bot 2");

    assert_eq!(Some(2), b.find_comparison(5, 2));
    assert_eq!(Some(2), b.find_comparison(2, 5));
    assert_eq!(5, b.get_output_val(0));
    assert_eq!(2, b.get_output_val(1));
    assert_eq!(3, b.get_output_val(2));
}
