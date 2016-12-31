// advent1.rs
// Manhattan lengths and segment intersection

use std::io;
use std::collections::HashSet;

type Vec2 = [i32; 2];

// unit vectors for the 4 cardinal directions
const NORTH: Vec2 = [0, 1];
#[allow(dead_code)]
const SOUTH: Vec2 = [0, -1];
#[allow(dead_code)]
const EAST: Vec2 = [1, 0];
#[allow(dead_code)]
const WEST: Vec2 = [-1, 0];


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("part 1 distance: {}", calc_manhattan_length(&input));
    println!("part 2 distance: {}", find_first_revisited_distance(&input));
}

// split the input into individual turns, add up all the distance vectors, and return the distance
fn calc_manhattan_length(turns: &str) -> i32 {
    let mut dir = NORTH;
    let mut position = [0, 0];

    for turn in turns.trim().split(", ") {
        let dist = parse_turn(&mut dir, turn);
        position[0] += dir[0] * dist;
        position[1] += dir[1] * dist;
    }

    position[0].abs() + position[1].abs()
}

// returns new distance, updates direction
// We're making the assumption that turn starts with L or R followed by a number.
// This function probably will panic if given garbage
fn parse_turn(dir: &mut Vec2, turn: &str) -> i32 {
    let (dir_str, dist_str) = turn.split_at(1); // could panic

    *dir = match dir_str.chars().nth(0) {
        Some('R') => turn_right(*dir),
        Some('L') => turn_left(*dir),
        _ => panic!("unexpected character in input"),    // could panic
    };

    dist_str.parse::<i32>().unwrap()            // could panic
}

// rotate clockwise 90 degrees
fn turn_right(dir: Vec2) -> Vec2 {
    [dir[1], -dir[0]]
}

// rotate counter-clockwise 90 degrees
fn turn_left(dir: Vec2) -> Vec2 {
    [-dir[1], dir[0]]
}


// ////////
// Part 2
fn find_first_revisited_distance(turns: &str) -> i32 {
    let mut dir = NORTH;
    let mut position = [0, 0];
    let mut visited = HashSet::new();
    visited.insert(position);

    'outer: for turn in turns.trim().split(", ") {
        let distance = parse_turn(&mut dir, turn);
        for _ in 0..distance {
            position[0] += dir[0];
            position[1] += dir[1];
            if !visited.insert(position) {
                // already in set, we've been here before
                break 'outer;
            }
        }
    }

    position[0].abs() + position[1].abs()
}


// //////
// Tests
#[test]
fn test_turn_right() {
    assert_eq!(EAST, turn_right(NORTH));
    assert_eq!(SOUTH, turn_right(EAST));
    assert_eq!(WEST, turn_right(SOUTH));
    assert_eq!(NORTH, turn_right(WEST));
}

#[test]
fn test_turn_left() {
    assert_eq!(WEST, turn_left(NORTH));
    assert_eq!(NORTH, turn_left(EAST));
    assert_eq!(EAST, turn_left(SOUTH));
    assert_eq!(SOUTH, turn_left(WEST));
}

#[test]
fn test_parse_turn() {
    let mut dir = NORTH;
    assert_eq!(3, parse_turn(&mut dir, "R3"));
    assert_eq!(EAST, dir);
    assert_eq!(23, parse_turn(&mut dir, "L23"));
    assert_eq!(NORTH, dir);
}

#[test]
fn test_calc_manhattan_length() {
    assert_eq!(5, calc_manhattan_length("R2, L3"));
    assert_eq!(2, calc_manhattan_length("R2, R2, R2"));
    assert_eq!(12, calc_manhattan_length("R5, L5, R5, R3"));
}

// part 2
#[test]
fn test_find_first_revisited_distance() {
    assert_eq!(4, find_first_revisited_distance("R8, R4, R4, R8"));
}
