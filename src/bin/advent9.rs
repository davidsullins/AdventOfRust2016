// advent9.rs
// simple RLE decompression

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");

    let compressed = input.trim();
    let length = calc_decompressed_length(compressed);
    println!("part 1 decompressed length: {}", length);
}

// ///////
// Part 1
fn calc_decompressed_length(s: &str) -> usize {
    let mut iter = s.chars();
    let mut length = 0;

    while let Some(c) = iter.next() {
        if c == '(' {
            let count = parse_usize(&mut iter);
            let repeat = parse_usize(&mut iter);
            length += count * repeat;
            // don't forget to skip the part we repeat
            iter.nth(count - 1);
        } else {
            length += 1;
        }
    }
    length
}

// consumes an integer and the next character after it (should be either 'x' or ')')
fn parse_usize(iter: &mut std::str::Chars) -> usize {
    let mut num = 0;
    while let Some(c) = iter.next() {
        if c >= '0' && c <= '9' {
            let digit = c as usize - '0' as usize;
            num = num * 10 + digit;
        } else {
            break;
        }
    }
    num
}

// //////
// Tests

#[test]
fn test_calc_decompressed_length() {
    assert_eq!(6, calc_decompressed_length("ADVENT"));
    assert_eq!(7, calc_decompressed_length("A(1x5)BC"));
    assert_eq!(9, calc_decompressed_length("(3x3)XYZ"));
    assert_eq!(11, calc_decompressed_length("A(2x2)BCD(2x2)EFG"));
    assert_eq!(6, calc_decompressed_length("(6x1)(1x3)A"));
    assert_eq!(18, calc_decompressed_length("X(8x2)(3x3)ABCY"));
}
