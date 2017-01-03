// advent17.rs
// Vault maze path searching

extern crate md5;

use std::io;
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let passcode = input.trim();

    if let Some(path) = find_path_to_goal(passcode, false) {
        println!("Part 1 shortest path: {}", path);
    } else {
        println!("Part 1 no path to vault");
    }

    if let Some(len) = find_longest_path_to_goal(passcode) {
        println!("Part 2 longest path length: {}", len);
    } else {
        println!("Part 2 no path to vault");
    }
}

// ///////
// Part 1

// Calculate shortest path from 0,0 to 3,3 if such a path exists
fn find_path_to_goal(passcode: &str, longest: bool) -> Option<String> {
    let state = MazeState::from_passcode(passcode);

    let mut states = VecDeque::new();
    states.push_back(state);

    // part 2
    let mut longest_path = None;

    while let Some(state) = states.pop_front() {
        for &dir in Dir::iter() {
            if state.is_open(dir) {
                let mut new_state = state.clone();
                new_state.move_dir(dir);
                if new_state.x == 3 && new_state.y == 3 {
                    // Found the vault!
                    let (_, path) = new_state.path.split_at(passcode.len());
                    if longest {
                        // part 2
                        longest_path = Some(path.to_string());
                        continue;
                    } else {
                        // part 1
                        return Some(path.to_string());
                    }
                }
                states.push_back(new_state);
            }
        }
    }

    if longest {
        // part 2
        longest_path
    } else {
        // part 1
        None
    }
}

#[derive(Clone)]
struct MazeState {
    x: u32,
    y: u32,
    path: String,
}

impl MazeState {
    fn from_passcode(passcode: &str) -> MazeState {
        MazeState {
            x: 0,
            y: 0,
            path: passcode.to_string(),
        }
    }

    fn is_open(&self, dir: Dir) -> bool {
        let is_wall = match dir {
            Dir::Up => self.y == 0,
            Dir::Down => self.y == 3,
            Dir::Left => self.x == 0,
            Dir::Right => self.x == 3,
        };
        if is_wall {
            false
        } else {
            let hash = *md5::compute(self.path.as_bytes());
            let nibble = match dir {
                Dir::Up => hash[0] >> 4,
                Dir::Down => hash[0] & 0xf,
                Dir::Left => hash[1] >> 4,
                Dir::Right => hash[1] & 0xf,
            };
            nibble >= 0xb
        }
    }

    fn move_dir(&mut self, dir: Dir) {
        match dir {
            Dir::Up => {
                self.y -= 1;
            }
            Dir::Down => {
                self.y += 1;
            }
            Dir::Left => {
                self.x -= 1;
            }
            Dir::Right => {
                self.x += 1;
            }
        }
        self.path.push(dir as u8 as char);
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Up = 'U' as isize,
    Down = 'D' as isize,
    Left = 'L' as isize,
    Right = 'R' as isize,
}

impl Dir {
    pub fn iter() -> std::slice::Iter<'static, Dir> {
        static DIRECTIONS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        DIRECTIONS.into_iter()
    }
}

// ///////
// Part 2
fn find_longest_path_to_goal(passcode: &str) -> Option<usize> {
    find_path_to_goal(passcode, true).map(|s| s.len())
}

// //////
// Tests

#[test]
fn test_is_wall() {
    let mut state = MazeState::from_passcode("hijkl");
    assert!(!state.is_open(Dir::Up));
    assert!(state.is_open(Dir::Down));
    assert!(!state.is_open(Dir::Left));
    assert!(!state.is_open(Dir::Right));

    state.move_dir(Dir::Down);
    assert!(state.is_open(Dir::Up));
    assert!(!state.is_open(Dir::Down));
    assert!(!state.is_open(Dir::Left));
    assert!(state.is_open(Dir::Right));

    let saved_state = state.clone();

    state.move_dir(Dir::Right);
    assert!(!state.is_open(Dir::Up));
    assert!(!state.is_open(Dir::Down));
    assert!(!state.is_open(Dir::Left));
    assert!(!state.is_open(Dir::Right));

    state = saved_state;
    state.move_dir(Dir::Up);
    assert!(!state.is_open(Dir::Up));
    assert!(!state.is_open(Dir::Down));
    assert!(!state.is_open(Dir::Left));
    assert!(state.is_open(Dir::Right));

    state.move_dir(Dir::Right);
    assert!(!state.is_open(Dir::Up));
    assert!(!state.is_open(Dir::Down));
    assert!(!state.is_open(Dir::Left));
    assert!(!state.is_open(Dir::Right));
}

#[test]
fn test_find_path_to_goal() {
    assert_eq!(None, find_path_to_goal("hijkl", false));
    assert_eq!("DDRRRD", find_path_to_goal("ihgpwlah", false).unwrap());
    assert_eq!("DDUDRLRRUDRD",
               find_path_to_goal("kglvqrro", false).unwrap());
    assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR",
               find_path_to_goal("ulqzkmiv", false).unwrap());
}

// part 2
#[test]
#[ignore]
fn test_find_longest_path_to_goal() {
    assert_eq!(None, find_longest_path_to_goal("hijkl"));
    assert_eq!(Some(370), find_longest_path_to_goal("ihgpwlah"));
    assert_eq!(Some(492), find_longest_path_to_goal("kglvqrro"));
    assert_eq!(Some(830), find_longest_path_to_goal("ulqzkmiv"));
}
