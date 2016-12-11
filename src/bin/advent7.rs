// advent7.rs
// IPv7, ABBA detection

use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();

    let addrs: Vec<_> = stdin.lock()
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .collect();

    let total_supports_tls = addrs.iter()
        .filter(|addr| supports_tls(&addr))
        .count();

    let total_supports_ssl = addrs.iter()
        .filter(|addr| supports_ssl(&addr))
        .count();

    println!("Part 1 total TLS addresses: {}", total_supports_tls);
    println!("Part 2 total SSL addresses: {}", total_supports_ssl);
}

// detect if address has ABBA in supernet sequences and no ABBA in hypernet sequences
fn supports_tls(addr: &str) -> bool {
    // assuming well-formed addresses that have correctly balanced [ and ]:
    // This means odd substrings are hypernet sequences and even substrings are not
    let (supernets, hypernets): (Vec<_>, Vec<_>) =
        addr.split(|c| c == '[' || c == ']').enumerate().partition(|&(i, _)| i % 2 == 0);

    hypernets.iter().all(|&(_, x)| !has_abba(x)) && supernets.iter().any(|&(_, x)| has_abba(x))
}

// detect if string has characters in the form ABBA
fn has_abba(s: &str) -> bool {
    s.as_bytes().windows(4).any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

// ///////
// Part 2

fn supports_ssl(addr: &str) -> bool {
    // Split IPV7 address into supernets and hypernets
    let (supernets, hypernets): (Vec<_>, Vec<_>) =
        addr.split(|c| c == '[' || c == ']').enumerate().partition(|&(i, _)| i % 2 == 0);

    // Find all ABAs in supernets and store them as a set of BABs
    let mut babs = HashSet::new();
    for &(_, supernet) in supernets.iter() {
        get_bab_from_aba(supernet, &mut babs);
    }

    // Search all hypernets for any BAB
    for &(_, hypernet) in hypernets.iter() {
        for bab in babs.iter() {
            if hypernet.contains(bab) {
                return true;
            }
        }
    }

    false
}

// find any ABAs in the string and add the corresponding BABs to the set
fn get_bab_from_aba(s: &str, babs: &mut HashSet<String>) {
    for bab in s.as_bytes()
        .windows(3)
        .filter_map(|w| if w[0] == w[2] && w[0] != w[1] {
            String::from_utf8(vec![w[1], w[0], w[1]]).ok()
        } else {
            None
        }) {
        babs.insert(bab);
    }
}

// ///////
// Tests

#[test]
fn test_supports_tls() {
    assert!(!supports_tls("[mnop]qrst"));
    assert!(supports_tls("abba[mnop]qrst"));
    assert!(!supports_tls("abcd[bddb]xyyx"));
    assert!(!supports_tls("aaaa[qwer]tyui"));
    assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
}

#[test]
fn test_has_abba() {
    assert!(!has_abba(""));
    assert!(!has_abba("abb"));
    assert!(!has_abba("abcd"));
    assert!(has_abba("abba"));
    assert!(has_abba("wfabbatu"));
    assert!(!has_abba("aaaa"));
    assert!(!has_abba("abbb"));
    assert!(!has_abba("abaa"));
}

// part 2
#[test]
fn test_supports_ssl() {
    assert!(supports_ssl("aba[bab]xyz"));
    assert!(!supports_ssl("xyz[xyz]xyz"));
    assert!(supports_ssl("aaa[kek]eke"));
    assert!(supports_ssl("zazbz[bzb]cdb"));
}

#[test]
fn test_get_bab_from_aba() {
    let mut test_set = HashSet::new();
    let mut babs = HashSet::new();
    get_bab_from_aba("", &mut babs);
    assert_eq!(test_set, babs);
    get_bab_from_aba("zazbz", &mut babs);
    test_set.insert("aza".to_string());
    test_set.insert("bzb".to_string());
    assert_eq!(test_set, babs);
}
