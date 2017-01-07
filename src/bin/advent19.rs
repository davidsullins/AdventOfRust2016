// advent19.rs
// Elf present-stealing

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let elf_count = input.trim().parse().expect("Input wasn't a number");
    let last_elf = find_last_elf(elf_count);
    println!("part 1 Elf {} got all the presents", last_elf);
    let last_elf2 = find_last_elf2(elf_count);
    println!("part 2 Elf {} got all the presents", last_elf2);
}

// ///////
// Part 1

fn find_last_elf(elf_count: usize) -> u32 {
    let mut elves: Vec<_> = (1..elf_count as u32 + 1).collect();
    loop {
        elves.retain(|&elf_number| elf_number > 0);
        let elf_count = elves.len();
        if elf_count == 1 {
            return elves[0];
        }
        for thief in 0..elf_count {
            let victim = (thief + 1) % elf_count;
            if elves[thief] > 0 {
                elves[victim] = 0;
            }
        }
    }
}

// ///////
// Part 2

fn find_last_elf2(elf_count: usize) -> u32 {
    let mut elves: Vec<_> = (1..elf_count as u32 + 1).collect();
    let mut thief = 0;
    loop {
        let removed_below_thief = elves.iter().take(thief).filter(|&&x| x == 0).count();
        elves.retain(|&x| x > 0);
        let remaining = elves.len();
        if remaining == 1 {
            return elves[0];
        }
        thief = (thief - removed_below_thief) % remaining;
        let mut victim = (thief + remaining / 2) % remaining;
        let mut victim_incr = if remaining % 2 == 0 { 1 } else { 2 };
        for _ in 0..remaining / 2 {
            assert!(0 != elves[thief]);
            elves[victim] = 0;
            thief = (thief + 1) % remaining;
            victim = (victim + victim_incr) % remaining;
            victim_incr = 3 - victim_incr;
        }
    }
}

// This version was really slow
fn _find_last_elf2_slow(elf_count: usize) -> u32 {
    let mut elves: Vec<_> = (1..elf_count as u32 + 1).collect();
    let mut thief = 0;
    for remaining in (2..elf_count + 1).rev() {
        let victim = (thief + remaining / 2) % remaining;
        elves.remove(victim);
        thief = if victim > thief {
            (thief + 1) % (remaining - 1)
        } else {
            thief % (remaining - 1)
        };
    }
    assert_eq!(1, elves.len());
    elves[0]
}


// //////
// Tests

#[test]
fn test_find_last_elf() {
    assert_eq!(3, find_last_elf(5));
}

#[test]
fn test_find_last_elf2() {
    assert_eq!(2, find_last_elf2(5));
}

#[test]
fn test_find_last_elf2_opt() {
    for i in 2..100 {
        assert_eq!(_find_last_elf2_slow(i), find_last_elf2(i));
    }
}
