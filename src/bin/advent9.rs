// advent9.rs
// simple RLE decompression

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");

    let compressed = input.trim();
    let length = calc_decompressed_length(compressed);
    println!("part 1 decompressed length: {}", length);
    let length2 = calc_decompressed_length2(compressed);
    println!("part 2 decompressed length: {}", length2);
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

// ///////
// Part 2
fn calc_decompressed_length2(s: &str) -> usize {
    let mut length = 0;
    let mut iter = s.chars().enumerate();

    while let Some((pos, c)) = iter.next() {
        if c == '(' {
            let mut offset = pos + 1;
            let count = parse_usize2(&s[offset..], &mut offset);
            let repeat = parse_usize2(&s[offset..], &mut offset);
            length += repeat * calc_decompressed_length2(&s[offset..offset + count]);
            // skip the part we consumed and the part we repeat
            iter.nth(offset - pos - 2 + count);
        } else {
            length += 1;
        }
    }
    length
}

// consumes an integer and the next character after it (should be either 'x' or ')')
fn parse_usize2(s: &str, chars_consumed: &mut usize) -> usize {
    let mut num = 0;
    let mut iter = s.chars();
    while let Some(c) = iter.next() {
        *chars_consumed += 1;
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

// part 2
#[test]
fn test_calc_decompressed_length2() {
    assert_eq!(6, calc_decompressed_length2("ADVENT"));
    assert_eq!(7, calc_decompressed_length2("A(1x5)BC"));
    assert_eq!(9, calc_decompressed_length2("(3x3)XYZ"));
    assert_eq!(20, calc_decompressed_length2("X(8x2)(3x3)ABCY"));
    assert_eq!(241920,
               calc_decompressed_length2("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
    assert_eq!(445,
               calc_decompressed_length2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"));
}
