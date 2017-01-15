// advent21.rs
// password scrambling

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufRead;
use std::collections::VecDeque;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let instructions: Vec<_> =
        stdin.lock().lines().map(|l| l.expect("Failed to read line")).collect();

    let scrambled = scramble_password("abcdefgh", &instructions);
    println!("Part 1 scrambled password: {}", scrambled);

    let unscrambled = unscramble_password("fbgdceah", &instructions);
    println!("Part 2 unscrambled password: {}", unscrambled);
}

// part 1
fn scramble_password(password: &str, instructions: &[String]) -> String {
    parse_instructions(password, instructions, true)
}

// part 2
fn unscramble_password(password: &str, instructions: &[String]) -> String {
    parse_instructions(password, instructions, false)
}

// part 1 and 2 combined
fn parse_instructions(password: &str, instructions: &[String], scramble: bool) -> String {
    let mut letters: VecDeque<u8> = password.as_bytes().iter().cloned().collect();

    // create reverse rotation by letter mapping for unscrambling (only for part 2)
    // Maps x to Some(y) iff there's an unique right rotation count y that could have landed at x
    let mut rot_by_letter = vec![];
    if !scramble {
        let len = password.len();
        let mut pairs = vec![];
        for from in 0..len {
            let rcount = 1 + from + if from >= 4 { 1 } else { 0 };
            let to = (from + rcount) % len;
            let lcount = (len + from - to) % len;
            pairs.push((to, lcount));
        }
        rot_by_letter.resize(len, None);
        let mut present = std::collections::HashSet::new();
        for (to, count) in pairs {
            if present.contains(&to) {
                rot_by_letter[to] = None;
            } else {
                rot_by_letter[to] = Some(count);
                present.insert(to);
            }
        }
    }

    lazy_static! {
        static ref RE_SWAP_POS: Regex = Regex::new(r"^swap position ([\d]+) with position ([\d]+)").unwrap();
        static ref RE_SWAP_LETTER: Regex = Regex::new(r"^swap letter ([a-z]) with letter ([a-z])").unwrap();
        static ref RE_ROT_LSTEP: Regex = Regex::new(r"^rotate left ([\d]+) step").unwrap();
        static ref RE_ROT_RSTEP: Regex = Regex::new(r"^rotate right ([\d]+) step").unwrap();
        static ref RE_ROT_LETTER: Regex = Regex::new(r"^rotate based on position of letter ([a-z])").unwrap();
        static ref RE_REV: Regex = Regex::new(r"^reverse positions ([\d]+) through ([\d]+)").unwrap();
        static ref RE_MOVE: Regex = Regex::new(r"^move position ([\d]+) to position ([\d]+)").unwrap();
    }

    // It seems ugly to box the iterator like this but I can't think of another way to select
    // between iterator types below.
    let iter: Box<std::iter::Iterator<Item = &String>> = if scramble {
        Box::new(instructions.iter())
    } else {
        Box::new(instructions.iter().rev())
    };

    for instr in iter {
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
            let dir = if scramble {
                Rotate::Left
            } else {
                Rotate::Right
            };
            rotate_letters(&mut letters, count, dir);
        } else if let Some(caps) = RE_ROT_RSTEP.captures(instr) {
            let count: usize = caps[1].parse().unwrap();
            let dir = if scramble {
                Rotate::Right
            } else {
                Rotate::Left
            };
            rotate_letters(&mut letters, count, dir);
        } else if let Some(caps) = RE_MOVE.captures(instr) {
            let pos1 = caps[1].parse().unwrap();
            let pos2 = caps[2].parse().unwrap();
            // not a very efficient way to do this
            if scramble {
                let c = letters.remove(pos1).unwrap();
                letters.insert(pos2, c);
            } else {
                let c = letters.remove(pos2).unwrap();
                letters.insert(pos1, c);
            }
        } else if let Some(caps) = RE_ROT_LETTER.captures(instr) {
            let letter: u8 = instr.as_bytes()[caps.get(1).unwrap().start()];
            let pos = letters.iter().position(|&x| x == letter).unwrap();
            let count = if scramble {
                1 + pos + if pos >= 4 { 1 } else { 0 }
            } else {
                rot_by_letter[pos].expect("no unique rotation for letter")
            };
            rotate_letters(&mut letters, count, Rotate::Right);
        } else {
            panic!("unknown instruction {}", instr);
        }
    }

    String::from_utf8(letters.into_iter().collect()).unwrap()
}

enum Rotate {
    Right,
    Left,
}

fn rotate_letters(letters: &mut VecDeque<u8>, count: usize, dir: Rotate) {
    let count = count % letters.len();
    match dir {
        Rotate::Right => {
            for _ in 0..count {
                let c = letters.pop_back().unwrap();
                letters.push_front(c);
            }
        }
        Rotate::Left => {
            for _ in 0..count {
                let c = letters.pop_front().unwrap();
                letters.push_back(c);
            }
        }
    }
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

#[test]
fn test_unscramble_password() {
    let mut instructions = Vec::new();

    // example from the webpage
    instructions.push("swap position 4 with position 0".to_string());
    instructions.push("swap letter d with letter b".to_string());
    instructions.push("reverse positions 0 through 4".to_string());
    instructions.push("rotate left 1 step".to_string());
    instructions.push("move position 1 to position 4".to_string());
    instructions.push("move position 3 to position 0".to_string());
    instructions.push("rotate based on position of letter b".to_string());
    assert_eq!("abcde", unscramble_password("ecabd", &instructions));

    // Note: No unambiguous way to reverse rotate on position of d!
    // instructions.push("rotate based on position of letter d".to_string());
}
