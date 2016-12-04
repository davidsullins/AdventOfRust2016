// advent2.rs
// bathroom keypad decoding

use std::io;
use std::io::prelude::*;


fn main() {
    let mut pos1 = (1,1);
    let mut code1 = String::new();
    let mut pos2 = (0,2);
    let mut code2 = String::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.expect("Failed to read line")) {
        // part 1
        pos1 = parse_move(pos1, &line);
        code1.push_str(&digit_from_pos(pos1).to_string());

        // part 2
        pos2 = parse_move2(pos2, &line);
        code2.push(char_from_pos2(pos2));
    }

    println!("Part 1: {}", code1);
    println!("Part 2: {}", code2);
}

fn digit_from_pos(pos: (i32, i32)) -> i32 {
    1 + pos.0 + 3 * pos.1
}

fn parse_move(pos: (i32, i32), s: &str) -> (i32, i32) {
    let (mut x, mut y) = pos;

    for c in s.trim().chars() {
        match c {
            'U' => if y > 0 {y -= 1;},
            'D' => if y < 2 {y += 1;},
            'L' => if x > 0 {x -= 1;},
            'R' => if x < 2 {x += 1;},
            _ => panic!("unexpected char in input")
        }
    }

    (x, y)
}

///////////
// Part 2

// Same as part 1, except the conditions are less obvious
fn parse_move2(pos: (i32, i32), s: &str) -> (i32, i32) {
    let (mut x, mut y) = pos;

    for c in s.trim().chars() {
        match c {
            'U' => if y - (x - 2).abs() > 0 {y -= 1;},
            'D' => if y + (x - 2).abs() < 4 {y += 1;},
            'L' => if x - (y - 2).abs() > 0 {x -= 1;},
            'R' => if x + (y - 2).abs() < 4 {x += 1;},
            _ => panic!("unexpected char in input")
        }
    }

    (x, y)
}

const KEYPAD2_TABLE: [[char; 5]; 5] = 
    [['*', '*', '1', '*', '*'],
     ['*', '2', '3', '4', '*'],
     ['5', '6', '7', '8', '9'],
     ['*', 'A', 'B', 'C', '*'],
     ['*', '*', 'D', '*', '*']];

fn char_from_pos2(pos: (i32, i32)) -> char {
    KEYPAD2_TABLE[pos.1 as usize][pos.0 as usize]
}

/////////
// Tests
#[test]
fn test_digit_from_pos() {
    assert_eq!(1, digit_from_pos((0, 0)));
    assert_eq!(2, digit_from_pos((1, 0)));
    assert_eq!(3, digit_from_pos((2, 0)));
    assert_eq!(4, digit_from_pos((0, 1)));
    assert_eq!(5, digit_from_pos((1, 1)));
    assert_eq!(6, digit_from_pos((2, 1)));
    assert_eq!(7, digit_from_pos((0, 2)));
    assert_eq!(8, digit_from_pos((1, 2)));
    assert_eq!(9, digit_from_pos((2, 2)));
}

#[test]
fn test_parse_move() {
    assert_eq!((0, 0), parse_move((1, 1), "ULL"));
    assert_eq!((2, 2), parse_move((0, 0), "RRDDD"));
    assert_eq!((1, 2), parse_move((2, 2), "LURDL"));
    assert_eq!((1, 1), parse_move((1, 2), "UUUUD"));
}

// part 2
#[test]
fn test_parse_move2() {
    assert_eq!((0, 2), parse_move2((0, 2), "ULL"));
    assert_eq!((2, 4), parse_move2((0, 2), "RRDDD"));
    assert_eq!((2, 3), parse_move2((2, 4), "LURDL"));
    assert_eq!((2, 1), parse_move2((2, 3), "UUUUD"));
}

#[test]
fn test_char_from_pos2() {
    assert_eq!('5', char_from_pos2((0, 2)));
}

