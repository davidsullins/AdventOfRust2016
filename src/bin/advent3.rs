// advent3.rs
// impossible triangles

use std::io;
use std::io::prelude::*;

fn main() {
    // read stdin into a Vec of "triples". Triple is a 3-element Vec<i32>
    let mut triples = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.expect("Failed to read line")) {
        let triple: Vec<i32> = line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        triples.push(triple);
    }

    // part 1
    let possible_triangles1 = triples.iter().filter(|t| is_triangle_possible(&t)).count();
    println!("Part 1 possible triangles: {}", possible_triangles1);

    // part 2
    // handle chunks of 3 triples at a time (so conceptually 3x3 matrices)
    // create new triples by column and test those, count how many of the 3 columns are triangles
    // then sum up results from all the chunks
    let possible_triangles2: usize = triples
        .chunks(3)
        .map(|m3x3| 
             (0..3)
             .filter(|col| 
                     is_triangle_possible(&m3x3.iter()
                                          .map(|row| row[*col])
                                          .collect::<Vec<i32>>()))
             .count())
        .sum();
    println!("Part 2 possible triangles: {}", possible_triangles2);
}

fn is_triangle_possible(tri: &[i32]) -> bool {
    tri[0] + tri[1] > tri[2] &&
    tri[0] + tri[2] > tri[1] &&
    tri[1] + tri[2] > tri[0]
}

/////////
// Tests
#[test]
fn test_is_triangle_possible() {
    assert!(!is_triangle_possible(&{[5, 10, 25]}));
    assert!(is_triangle_possible(&{[5, 4, 3]}));
}

