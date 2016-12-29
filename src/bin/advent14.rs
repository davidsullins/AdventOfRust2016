// advent14.rs
// keys for one-time pad

extern crate md5;

use std::io;
use std::fmt::Write;
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");

    let salt = input.trim();
    println!("part 1 64th index: {}", get_nth_idx(salt, 64, false));
    println!("part 2 64th index, key stretching: {}",
             get_nth_idx(salt, 64, true));
}

// ///////
// Part 1

// returns the nibble of the first triple found
fn find_triple(md5sum: md5::Digest) -> Option<u8> {
    md5sum.windows(2)
        .find(|w| {
            let nibble = w[0] & 0xf;
            (nibble == w[1] >> 4) && ((nibble == w[0] >> 4) || (nibble == w[1] & 0xf))
        })
        .map(|w| w[0] & 0xf)
}

// true if the md5 digest has 5 of the specified nibbles in a row
fn has_quint(nibble: u8, md5sum: md5::Digest) -> bool {
    let both = (nibble << 4) | nibble;

    md5sum.windows(3).any(|w| {
        w[1] == both &&
        (w[0] == both && w[2] >> 4 == nibble || w[2] == both && w[0] & 0xf == nibble)
    })
}

// Optimized so we only check each index for a triple once, and check each index for a quintuple
// once for each possible nibble.
// Work ahead 1000 indices and queue up triples and quintuples as we scan.
// Remove old triple indices as we go. Assuming quints will be rare so don't bother removing them.
fn get_nth_idx(salt: &str, n: usize, stretch_keys: bool) -> u64 {
    let mut guess = salt.to_string();
    let salt_len = salt.len();
    let mut key_count = 0;
    let mut triples = VecDeque::new();
    let mut quints = [vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![],
                      vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

    for i in 0u64.. {
        guess.truncate(salt_len);
        write!(guess, "{}", i).unwrap();
        let md5sum = if stretch_keys {
            // part 2
            calc_stretched_key(&guess)
        } else {
            // part 1
            md5::compute(guess.as_bytes())
        };
        if let Some(nibble) = find_triple(md5sum) {
            triples.push_back((i, nibble));
            for nibble in 0..0x10 {
                if has_quint(nibble, md5sum) {
                    quints[nibble as usize].push(i);
                }
            }

            // remove queue items more than 1000 indices before now
            while let Some((t_idx, nibble)) = triples.pop_front() {
                if t_idx + 1000 >= i {
                    // went too far, put this one back and stop searching
                    triples.push_front((t_idx, nibble));
                    break;
                }
                // check for quints that match
                for &q_idx in quints[nibble as usize].iter() {
                    if t_idx < q_idx && t_idx + 1000 >= q_idx {
                        // found a key!
                        key_count += 1;
                        if key_count == n {
                            return t_idx;
                        }
                        break;
                    }
                }
            }
        }
    }

    unreachable!();
}

// ///////
// Part 2

// Profiling the original implementation with callgrind showed most of the time spent in formatted
// write, so replaced with a table lookup
fn calc_stretched_key(s: &str) -> md5::Digest {
    const ASCII_FROM_NIBBLE: [u8; 16] = [0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
                                         0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66];
    let mut md5_str = [0u8; 32];
    let mut md5sum = md5::compute(s.as_bytes());

    for _ in 0..2016 {
        for (byte, chunk) in md5sum.iter().zip(md5_str.chunks_mut(2)) {
            chunk[0] = ASCII_FROM_NIBBLE[(byte >> 4) as usize];
            chunk[1] = ASCII_FROM_NIBBLE[(byte & 0xf) as usize];
        }
        md5sum = md5::compute(&md5_str);
    }

    md5sum
}

// //////
// Tests

#[test]
fn test_find_triple() {
    assert_eq!(None, find_triple(md5::compute("abc17".as_bytes())));
    assert_eq!(Some(8), find_triple(md5::compute("abc18".as_bytes())));
    assert_eq!(Some(0xe), find_triple(md5::compute("abc39".as_bytes())));
    assert_eq!(Some(9), find_triple(md5::compute("abc92".as_bytes())));
    assert_eq!(Some(0xc), find_triple(md5::compute("abc22728".as_bytes())));
}

#[test]
fn test_has_quint() {
    assert_eq!(false, has_quint(8, md5::compute("abc18".as_bytes())));
    assert_eq!(true, has_quint(9, md5::compute("abc200".as_bytes())));
    assert_eq!(true, has_quint(0xe, md5::compute("abc816".as_bytes())));
}

#[test]
fn test_get_nth_idx() {
    assert_eq!(39, get_nth_idx("abc", 1, false));
    assert_eq!(92, get_nth_idx("abc", 2, false));
    assert_eq!(22728, get_nth_idx("abc", 64, false));
}

// part 2
#[test]
fn test_calc_stretched_key() {
    let key = calc_stretched_key("abc0");
    println!("key: {:?}", key);
    assert_eq!([0xa1, 7, 0xff], key[0..3]);
}

#[test]
#[ignore]
fn test_get_nth_idx2() {
    assert_eq!(10, get_nth_idx("abc", 1, true));
    assert_eq!(22551, get_nth_idx("abc", 64, true));
}
