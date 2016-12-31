// advent4.rs
// parsing and shift ciphers

extern crate regex;

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let mut sector_id_sum = 0;
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.expect("Failed to read line")) {
        if is_real_room(&line) {
            // part 1
            let sector_id = parse_sector_id(&line);
            sector_id_sum += sector_id;

            // part 2
            let decrypted_room = decrypt_room(&line);
            // I had to decrypt everything and read through the output to figure out what
            // to search for here. Poorly specified problem.
            if decrypted_room.contains("northpole object storage") {
                println!("Part 2 sector ID {}", sector_id);
            }
        }
    }

    println!("Part 1 sector ID sum: {}", sector_id_sum);
}

fn is_real_room(room: &str) -> bool {
    let mut freq_map = HashMap::new();

    // count chars
    for c in room.chars() {
        if c == '-' {
            continue;
        }
        if !c.is_alphabetic() {
            break;
        }
        *freq_map.entry(c).or_insert(0) += 1;
    }

    let mut char_counts: Vec<(char, usize)> = freq_map.iter().map(|(k, v)| (*k, *v)).collect();
    char_counts.sort_by(|a, b| {
        if b.1 == a.1 {
            a.0.cmp(&b.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    let mut correct_checksum = char_counts.iter().map(|&(k, _)| k).take(5).collect::<String>();
    correct_checksum.push(']');

    room.contains(&correct_checksum)
}

fn parse_sector_id(room: &str) -> i32 {
    room.rsplit('-').nth(0).unwrap().split('[').nth(0).unwrap().parse().unwrap()
}

// ///////
// Part 2

fn decrypt_room(room: &str) -> String {
    let re = Regex::new(r"([a-z-]+)-(\d+)\[").unwrap(); // won't panic because it's a valid RE

    let cap = re.captures(room).unwrap();   // could panic if the RE doesn't match

    // unwraps are safe here because the RE matched
    let text = cap.at(1).unwrap();
    let num: u32 = cap.at(2).unwrap().parse().unwrap();
    let shift = (num % 26) as u8;

    text.chars().map(|c| decrypt_char(c, shift)).collect()
}

fn decrypt_char(c: char, shift: u8) -> char {
    if c == '-' {
        ' '
    } else {
        let offset = b'a';
        let new = (c as u8 - offset + shift) % 26;
        (new + offset) as char
    }
}

// //////
// Tests

// part 1
#[test]
fn test_is_real_room() {
    assert!(is_real_room("aaaaa-bbb-z-y-x-123[abxyz]"));
    assert!(is_real_room("a-b-c-d-e-f-g-h-987[abcde]"));
    assert!(is_real_room("not-a-real-room-404[oarel]"));
    assert!(!is_real_room("totally-real-room-200[decoy]"));
}

#[test]
fn test_parse_sector_id() {
    assert_eq!(123, parse_sector_id("aaaaa-bbb-z-y-x-123[abxyz]"));
    assert_eq!(987, parse_sector_id("a-b-c-d-e-f-g-h-987[abcde]"));
    assert_eq!(404, parse_sector_id("not-a-real-room-404[oarel]"));
    assert_eq!(200, parse_sector_id("totally-real-room-200[decoy]"));
}

// part 2
#[test]
fn test_decrypt_room() {
    assert_eq!("very encrypted name",
               decrypt_room("qzmt-zixmtkozy-ivhz-343[dummy]"));
}

#[test]
fn test_decrypt_char() {
    assert_eq!('v', decrypt_char('q', 5));
}
