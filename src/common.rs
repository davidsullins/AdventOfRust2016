// common.rs
// Work that's common to multiple Advent of Code problems

use std::collections::{VecDeque, HashSet};

pub type Location = (i32, i32);

// Calculate minimum number of steps in path from start to goal, if such a path exists
pub fn find_steps<F>(start: Location, goal: Location, is_wall: F) -> Option<usize>
    where F: Fn(Location) -> bool
{
    if start == goal {
        return Some(0);
    }

    let mut locations = VecDeque::new();
    let mut visited = HashSet::new();
    locations.push_back((start, 0));
    visited.insert(start);

    while let Some((location, steps)) = locations.pop_front() {
        // find all neighbors
        for &(i, j) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let neighbor = (location.0 + i, location.1 + j);
            if neighbor == goal {
                return Some(steps + 1);
            }
            if neighbor.0 >= 0 && neighbor.1 >= 0 && !is_wall(neighbor) &&
               !visited.contains(&neighbor) {
                locations.push_back((neighbor, steps + 1));
                visited.insert(neighbor);
            }
        }
    }

    None
}

// //////
// Tests
//
#[cfg(test)]
mod tests {
    use super::{Location, find_steps};

    fn is_wall((x, y): Location, favorite: i32) -> bool {
        (x * x + 3 * x + 2 * x * y + y + y * y + favorite).count_ones() % 2 != 0
    }

    #[test]
    fn test_find_steps() {
        assert_eq!(Some(0),
                   find_steps((13, 37), (13, 37), |loc| is_wall(loc, 10)));
        assert_eq!(Some(11), find_steps((1, 1), (7, 4), |loc| is_wall(loc, 10)));
        assert_eq!(None, find_steps((1, 1), (31, 39), |loc| is_wall(loc, 10)));
    }
}
