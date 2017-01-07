// advent20.rs
// find ip not in range

use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut blacklist = vec![];

    for line in stdin.lock().lines().map(|l| l.expect("Failed to read line")) {
        let ip_range: Vec<u32> =
            line.trim().split('-').map(|x| x.parse().expect("non-numeric IP")).collect();
        if 2 != ip_range.len() {
            println!("unexpected input {}", line);
            continue;
        }
        blacklist.push((ip_range[0], ip_range[1]));
    }
    blacklist.sort_by_key(|x| x.0);

    if let Some(ip) = find_first_unblocked_ip(&blacklist) {
        println!("Part 1 first available IP: {}", ip);
    } else {
        println!("Part 1 no IPs available");
    }

    println!("Part 2 total unblocked IPs: {}",
             find_total_unblocked_ips(&blacklist));
}

// ///////
// Part 1

#[cfg(test)]
const MAX_IP: u32 = 10;

#[cfg(not(test))]
const MAX_IP: u32 = std::u32::MAX;

fn find_first_unblocked_ip(blacklist: &[(u32, u32)]) -> Option<u32> {
    if let Some(&(lowest, _)) = blacklist.first() {
        if lowest > 0 {
            return Some(0);
        }
    }

    let mut highest = 0;
    for &(low, high) in blacklist {
        if low > highest + 1 {
            return Some(highest + 1);
        }
        highest = std::cmp::max(highest, high);
        if highest == MAX_IP {
            break;
        }
    }

    if highest < MAX_IP {
        Some(highest + 1)
    } else {
        None
    }
}

// ///////
// Part 2

fn find_total_unblocked_ips(blacklist: &[(u32, u32)]) -> usize {
    let mut total_available = 0;

    if let Some(&(lowest, _)) = blacklist.first() {
        if lowest > 0 {
            total_available = 1;
        }
    }

    let mut highest = 0;
    for &(low, high) in blacklist {
        if highest < MAX_IP {
            total_available += (highest + 1..low).count();
        }
        highest = std::cmp::max(highest, high);
        if highest == MAX_IP {
            break;
        }
    }

    if highest < MAX_IP {
        total_available + (highest + 1..MAX_IP).count()
    } else {
        total_available
    }
}




// //////
// Tests
#[test]
fn test_find_first_unblocked_ip() {
    let blacklist = vec![(0, 2), (4, 7), (5, 8)];
    assert_eq!(Some(3), find_first_unblocked_ip(&blacklist));
}

#[test]
fn test_find_total_unblocked_ips() {
    let blacklist = vec![(0, 2), (4, 7), (5, 8)];
    assert_eq!(2, find_total_unblocked_ips(&blacklist));
}
