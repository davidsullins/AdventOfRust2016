// advent21.rs
// password scrambling

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

    let scrambled = scramble_password("abcdefgh", &instructions);
    println!("Part 1 scrambled password: {}", scrambled);
}

fn scramble_password(password: &str, instructions: &[String]) -> String {
    let mut letters: std::collections::VecDeque<u8> = password.as_bytes().iter().cloned().collect();

    for instr in instructions {
        lazy_static! {
            static ref RE_SWAP_POS: Regex = Regex::new(r"^swap position ([\d]+) with position ([\d]+)").unwrap();
            static ref RE_SWAP_LETTER: Regex = Regex::new(r"^swap letter ([a-z]) with letter ([a-z])").unwrap();
            static ref RE_ROT_LSTEP: Regex = Regex::new(r"^rotate left ([\d]+) step").unwrap();
            static ref RE_ROT_RSTEP: Regex = Regex::new(r"^rotate right ([\d]+) step").unwrap();
            static ref RE_ROT_LETTER: Regex = Regex::new(r"^rotate based on position of letter ([a-z])").unwrap();
            static ref RE_REV: Regex = Regex::new(r"^reverse positions ([\d]+) through ([\d]+)").unwrap();
            static ref RE_MOVE: Regex = Regex::new(r"^move position ([\d]+) to position ([\d]+)").unwrap();
        }

        if let Some(caps) = RE_SWAP_POS.captures(instr) {
            let pos1 = caps[1].parse().unwrap();
            let pos2 = caps[2].parse().unwrap();
            letters.swap(pos1, pos2);
        } else if let Some(caps) = RE_SWAP_LETTER.captures(instr) {
            let letter1: u8 = instr.as_bytes()[caps.get(1).unwrap().start()];
            let letter2: u8 = instr.as_bytes()[caps.get(2).unwrap().start()];
            let pos1 = letters.iter().position(|&x| x == letter1).unwrap();
            let pos2 = letters.iter().position(|&x| x == letter2).unwrap();
            letters.swap(pos1, pos2);
        } else if let Some(caps) = RE_REV.captures(instr) {
            let pos1: usize = caps[1].parse().unwrap();
            let pos2: usize = caps[2].parse().unwrap();
            assert!(pos2 > pos1);
            let dist = (pos2 - pos1 + 1) / 2;
            for i in 0..dist {
                letters.swap(pos1 + i, pos2 - i);
            }
        } else if let Some(caps) = RE_ROT_LSTEP.captures(instr) {
            let count: usize = caps[1].parse().unwrap();
            for _ in 0..count {
                let c = letters.pop_front().unwrap();
                letters.push_back(c);
            }
        } else if let Some(caps) = RE_ROT_RSTEP.captures(instr) {
            let count: usize = caps[1].parse().unwrap();
            for _ in 0..count {
                let c = letters.pop_back().unwrap();
                letters.push_front(c);
            }
        } else if let Some(caps) = RE_MOVE.captures(instr) {
            let pos1 = caps[1].parse().unwrap();
            let pos2 = caps[2].parse().unwrap();
            // not a very efficient way to do this
            let c = letters.remove(pos1).unwrap();
            letters.insert(pos2, c);
        } else if let Some(caps) = RE_ROT_LETTER.captures(instr) {
            let letter: u8 = instr.as_bytes()[caps.get(1).unwrap().start()];
            let pos = letters.iter().position(|&x| x == letter).unwrap();
            let count = 1 + pos + if pos >= 4 { 1 } else { 0 };
            for _ in 0..count {
                let c = letters.pop_back().unwrap();
                letters.push_front(c);
            }
        } else {
            panic!("unknown instruction {}", instr);
        }
    }

    String::from_utf8(letters.iter().cloned().collect()).unwrap()
}

// //////
// Tests
#[test]
fn test_scramble_password() {
    let mut instructions = Vec::new();

    // example from the webpage
    instructions.push("swap position 4 with position 0".to_string());
    assert_eq!("ebcda", scramble_password("abcde", &instructions));
    instructions.push("swap letter d with letter b".to_string());
    assert_eq!("edcba", scramble_password("abcde", &instructions));
    instructions.push("reverse positions 0 through 4".to_string());
    assert_eq!("abcde", scramble_password("abcde", &instructions));
    instructions.push("rotate left 1 step".to_string());
    assert_eq!("bcdea", scramble_password("abcde", &instructions));
    instructions.push("move position 1 to position 4".to_string());
    assert_eq!("bdeac", scramble_password("abcde", &instructions));
    instructions.push("move position 3 to position 0".to_string());
    assert_eq!("abdec", scramble_password("abcde", &instructions));
    instructions.push("rotate based on position of letter b".to_string());
    assert_eq!("ecabd", scramble_password("abcde", &instructions));
    instructions.push("rotate based on position of letter d".to_string());
    assert_eq!("decab", scramble_password("abcde", &instructions));

    // additional tests
    instructions.push("rotate right 3 steps".to_string());
    assert_eq!("cabde", scramble_password("abcde", &instructions));
    instructions.push("reverse positions 0 through 1".to_string());
    assert_eq!("acbde", scramble_password("abcde", &instructions));
}
