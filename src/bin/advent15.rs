// advent15.rs
// aligning discs

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let mut discs = vec![];

    for line in stdin.lock().lines().map(|l| l.expect("Failed to read line")) {
        // assume discs come in order, because they do in my input
        discs.push(Disc::from_description(&line));
    }

    // part 1
    let drop_time = calc_drop_time(&discs);
    println!("Part 1: drop time = {}", drop_time);

    // part 2
    discs.push(Disc {
        position: 0,
        count: 11,
    });
    let drop_time2 = calc_drop_time(&discs);
    println!("Part 2: drop time = {}", drop_time2);
}

#[derive(Clone)]
struct Disc {
    position: usize,
    count: usize,
}

impl Disc {
    fn from_description(desc: &str) -> Disc {
        lazy_static! {
            static ref RE_DESC: Regex =
                Regex::new(r"^Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+)").unwrap();
        }
        if let Some(caps) = RE_DESC.captures(desc) {
            let count: usize = caps.at(1).unwrap().parse().unwrap();
            let position: usize = caps.at(2).unwrap().parse().unwrap();
            Disc {
                position: position,
                count: count,
            }
        } else {
            panic!("Invalid description!")
        }
    }

    fn incr(&mut self) {
        self.incr_by(1);
    }

    fn incr_by(&mut self, incr: usize) {
        self.position = (self.position + incr) % self.count;
    }
}

// move discs to the positions they'll be in when the capsule arrives
fn adjust_discs(discs: &mut [Disc]) {
    for (i, disc) in discs.iter_mut().enumerate() {
        disc.incr_by(i + 1);
    }
}

fn are_discs_aligned(discs: &[Disc]) -> bool {
    discs.iter().all(|disc| disc.position == 0)
}

fn calc_drop_time(discs: &[Disc]) -> usize {
    let mut discs = Vec::from(discs);
    adjust_discs(&mut discs);
    let mut drop_time = 0;
    while !are_discs_aligned(&discs) {
        for disc in &mut discs {
            disc.incr();
        }
        drop_time += 1;
    }
    drop_time
}


// //////
// Tests
#[test]
fn test_disc_from_description() {
    let disc = Disc::from_description("Disc #1 has 5 positions; at time=0, it is at position 4.");
    assert_eq!(4, disc.position);
    assert_eq!(5, disc.count);
}

#[test]
fn test_disc_incr() {
    let mut disc = Disc {
        position: 4,
        count: 5,
    };
    disc.incr();
    assert_eq!(0, disc.position);
    disc.incr_by(37);
    assert_eq!(2, disc.position);
}

#[test]
fn test_adjust_discs() {
    let disc = Disc {
        position: 0,
        count: 3,
    };
    let mut discs = vec![];
    for _ in 0..4 {
        discs.push(disc.clone());
    }
    adjust_discs(&mut discs);
    assert_eq!(1, discs[0].position);
    assert_eq!(2, discs[1].position);
    assert_eq!(0, discs[2].position);
    assert_eq!(1, discs[3].position);
}

#[test]
fn test_calc_drop_time() {
    let mut discs = vec![];
    discs.push(Disc {
        position: 4,
        count: 5,
    });
    discs.push(Disc {
        position: 1,
        count: 2,
    });
    assert_eq!(5, calc_drop_time(&mut discs));
}
