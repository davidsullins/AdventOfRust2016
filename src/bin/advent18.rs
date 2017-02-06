// advent18.rs
// Safe tiles vs traps

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut trap_room = TrapRoom::create_from_str(&input);
    trap_room.grow(40);
    println!("part 1 safe tile count = {}", trap_room.count_safe_tiles());
    trap_room.grow(400000);
    println!("part 2 safe tile count = {}", trap_room.count_safe_tiles());
}

// ///////
// Part 1

struct TrapRoom {
    traps: Vec<Vec<bool>>,
    row_len: usize,
}

impl TrapRoom {
    fn create_from_str(s: &str) -> TrapRoom {
        let mut row = vec![false];
        row.extend(s.chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '^' => Some(true),
                _ => None,
            }));
        row.push(false);
        TrapRoom {
            row_len: row.len(),
            traps: vec![row],
        }
    }

    fn add_row(&mut self) {
        let mut new_row = Vec::with_capacity(self.row_len);
        if let Some(old_row) = self.traps.last() {
            new_row.push(false);
            new_row.extend(old_row.windows(3).map(|w| w[0] != w[2]));
            new_row.push(false);
        }
        self.traps.push(new_row);
    }

    fn grow(&mut self, len: usize) {
        if len > self.traps.len() {
            let additional = len - self.traps.len();
            self.traps.reserve(additional);
            for _ in 0..additional {
                self.add_row();
            }

        }
    }

    fn count_safe_tiles(&self) -> usize {
        self.traps.iter().flat_map(|v| &v[1..self.row_len - 1]).filter(|&&b| !b).count()
    }
}

// //////
// Tests

#[test]
fn test_count_safe_tiles() {
    let mut room1 = TrapRoom::create_from_str("..^^.");
    room1.grow(3);
    assert_eq!(6, room1.count_safe_tiles());

    let mut room2 = TrapRoom::create_from_str(".^^.^.^^^^");
    room2.grow(10);
    assert_eq!(38, room2.count_safe_tiles());
}
