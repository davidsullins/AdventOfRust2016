// advent5.rs
// recovering door passwords use md5 hashes

extern crate md5;

use std::io;
use std::fmt::Write;

fn main() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");

    let door_id = input.trim();
    println!("part 1 password: {}", get_password(door_id));
    println!("part 2 password: {}", get_password2(door_id));

}

//////////
// Part 1

fn get_password(door_id: &str) -> String {
    let mut guess = door_id.to_string();
    let len = door_id.len();

    (0u64..)
        .filter_map(|x| {
            guess.truncate(len);
            write!(guess, "{}", x).unwrap();
            get_password_character(&guess)
        })
        .take(8)
        .collect()
}

// if md5 hash starts with 5 0's in hex, output the 6th hex digit
fn get_password_character(guess: &str) -> Option<char> {
    let md5sum = md5::compute(guess.as_bytes());

    if md5sum[0] == 0 && md5sum[1] == 0 && md5sum[2] <= 0xf {
        Some(char_from_nibble(md5sum[2]))
    } else {
        None
    }
}

// output the char representing a hex digit
fn char_from_nibble(nibble: u8) -> char {
    assert!(nibble <= 0xf);

    if nibble <= 9 {
        (('0' as u8) + nibble) as char
    } else {
        (('a' as u8) + nibble - 10) as char
    }
}

///////////
// Part 2

fn get_password2(door_id: &str) -> String {
    let mut guess = door_id.to_string();
    let len = door_id.len();
    let mut password = vec!['*'; 8];
    let mut total_chars = 0;

    let pw_iter = 
        (0u64..)
            .filter_map(|x| {
                guess.truncate(len);
                write!(guess, "{}", x).unwrap();
                get_password_character2(&guess)
            });

    for (c, pos) in pw_iter {
        if '*' == password[pos] {
            password[pos] = c;
            total_chars += 1;
            if total_chars == 8 {
                break;
            }
        }
    }

    password.into_iter().collect()
}

// like part 1 but the 6th hex digit is a position if < 8 and the 7th is the char
fn get_password_character2(guess: &str) -> Option<(char, usize)> {
    let md5sum = md5::compute(guess.as_bytes());

    if md5sum[0] == 0 && md5sum[1] == 0 && md5sum[2] <= 7 {
        Some((char_from_nibble((md5sum[3] & 0xf0) >> 4), md5sum[2] as usize))
    } else {
        None
    }
}


/////////
// Tests

#[test]
#[ignore]
fn test_get_password() {
    assert_eq!("18f47a30", get_password("abc"));
}

#[test]
fn test_get_password_character() {
    assert_eq!(None, get_password_character("abc3231928"));
    assert_eq!(Some('1'), get_password_character("abc3231929"));
    assert_eq!(Some('8'), get_password_character("abc5017308"));
    assert_eq!(Some('f'), get_password_character("abc5278568"));
}

#[test]
fn test_char_from_nibble() {
    assert_eq!('0', char_from_nibble(0));
    assert_eq!('9', char_from_nibble(9));
    assert_eq!('a', char_from_nibble(0xa));
    assert_eq!('f', char_from_nibble(0xf));
}

// Part 2
#[test]
#[ignore]
fn test_get_password2() {
    assert_eq!("05ace8e3", get_password2("abc"));
}

#[test]
fn test_get_password_character2() {
    assert_eq!(None, get_password_character2("abc3231928"));
    assert_eq!(Some(('5', 1)), get_password_character2("abc3231929"));
    assert_eq!(None, get_password_character2("abc5017308"));
    assert_eq!(None, get_password_character2("abc5278568"));
    assert_eq!(Some(('e', 4)), get_password_character2("abc5357525"));
}

