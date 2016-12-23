// advent13.rs
// Office maze path searching

use std::io::{self, Write};
use std::collections::{VecDeque, HashSet};

type Location = (i32, i32);

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let favorite = input.trim().parse().ok().expect("Input wasn't a number");

    if let Some(steps) = find_steps_to_goal((1, 1), (31, 39), favorite) {
        println!("Part 1: min steps to 31, 39: {}", steps);
    } else {
        println!("Part 1: 31, 39 not reachable!");
    }

    let reachable = count_reachable_locations((1, 1), 50, favorite);
    println!("Part 2: max reachable locations in 50 steps: {}", reachable);

}

// ///////
// Part 1
fn is_wall((x, y): Location, favorite: i32) -> bool {
    (x * x + 3 * x + 2 * x * y + y + y * y + favorite).count_ones() % 2 != 0
}

// Calculate minimum number of steps in path from start to goal, if such a path exists
fn find_steps_to_goal(start: Location, goal: Location, favorite: i32) -> Option<usize> {
    if start == goal {
        return Some(0);
    }

    let mut locations = VecDeque::new();
    let mut visited = HashSet::new();
    locations.push_back((start, 0));
    visited.insert(start);

    while let Some((location, steps)) = locations.pop_front() {
        // find all neighbors
        for &(i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let neighbor = (location.0 + i, location.1 + j);
            if neighbor == goal {
                return Some(steps + 1);
            }
            if neighbor.0 >= 0 && neighbor.1 >= 0 && !is_wall(neighbor, favorite) &&
               !visited.contains(&neighbor) {
                locations.push_back((neighbor, steps + 1));
                visited.insert(neighbor);
            }
        }
    }

    None
}

// just for fun
fn _print_office(width: i32, length: i32, favorite: i32) {
    for y in 0..length {
        for x in 0..width {
            let c = if is_wall((x, y), favorite) {
                b"#"
            } else {
                b"."
            };
            std::io::stdout().write(c).unwrap();
        }
        std::io::stdout().write(b"\n").unwrap();
    }
}

// ///////
// Part 2
// Count how many locations are reachable in a maximum number of steps
fn count_reachable_locations(start: Location, max_steps: usize, favorite: i32) -> usize {
    let mut locations = VecDeque::new();
    let mut visited = HashSet::new();
    locations.push_back((start, 0));
    visited.insert(start);

    while let Some((location, steps)) = locations.pop_front() {
        if steps >= max_steps {
            continue;
        }

        // find all neighbors
        for &(i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let neighbor = (location.0 + i, location.1 + j);
            if neighbor.0 >= 0 && neighbor.1 >= 0 && !is_wall(neighbor, favorite) &&
               !visited.contains(&neighbor) {
                locations.push_back((neighbor, steps + 1));
                visited.insert(neighbor);
            }
        }
    }

    visited.len()
}

// //////
// Tests

#[test]
fn test_is_wall() {
    assert!(!is_wall((0, 0), 10));
    assert!(is_wall((1, 0), 10));
    assert!(!is_wall((2, 0), 10));
    assert!(is_wall((3, 0), 10));
    assert!(is_wall((4, 0), 10));
    assert!(is_wall((5, 0), 10));
    assert!(!is_wall((0, 1), 10));
    assert!(!is_wall((1, 1), 10));
    assert!(is_wall((2, 1), 10));
    assert!(!is_wall((3, 1), 10));
}

#[test]
fn test_find_steps_to_goal() {
    assert_eq!(Some(0), find_steps_to_goal((13, 37), (13, 37), 10));
    assert_eq!(Some(11), find_steps_to_goal((1, 1), (7, 4), 10));
    assert_eq!(None, find_steps_to_goal((1, 1), (31, 39), 10));
}
