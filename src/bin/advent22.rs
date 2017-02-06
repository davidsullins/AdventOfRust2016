// advent22.rs
// disk space (sliding blocks puzzle)

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::max;

type Location = (i32, i32);

fn main() {
    let stdin = io::stdin();

    let nodes: Vec<_> = stdin.lock()
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .filter_map(|l| node_from_str(&l))
        .collect();

    let viable = find_viable_pairs(&nodes);
    println!("part 1: {} viable pairs", viable.len());

    let steps = calc_fewest_steps_to_goal(&nodes);
    println!("part 2: {} steps to goal", steps.unwrap());
}

#[derive(PartialEq)]
struct Node {
    loc: Location,
    used: usize,
    avail: usize,
}

fn is_viable_pair((n1, n2): (&Node, &Node)) -> bool {
    n1.used <= n2.avail && n1 != n2 && n1.used != 0
}

fn find_viable_pairs(nodes: &[Node]) -> Vec<(&Node, &Node)> {
    let mut viable = vec![];
    for n1 in nodes {
        viable.extend(nodes.iter().map(|n2| (n1, n2)).filter(|&x| is_viable_pair(x)))
    }
    viable
}

fn node_from_str(s: &str) -> Option<Node> {
    lazy_static! {
        static ref RE_NODE: Regex =
            Regex::new(r"^/dev/grid/node-x([\d]+)-y([\d]+)\s+[\d]+T\s+([\d]+)T\s+([\d]+)").unwrap();
    }

    RE_NODE.captures(s).map(|caps| {
        let x = caps[1].parse().unwrap();
        let y = caps[2].parse().unwrap();
        let used = caps[3].parse().unwrap();
        let avail = caps[4].parse().unwrap();
        Node {
            loc: (x, y),
            used: used,
            avail: avail,
        }
    })
}

// ///////
// Part 2

// this function can only find an answer if these conditions are met:
// 1. Exactly 1 empty node
// 2. All other nodes either can't be moved at all, or can only be moved to the empty node
// 3. All nodes in the first 2 rows are moveable
//
// Returns None if we can't find an answer
fn calc_fewest_steps_to_goal(nodes: &[Node]) -> Option<usize> {
    let mut empties = Vec::new();
    let mut moveable = HashSet::new();
    let mut multiple_pairs = HashSet::new();

    for n1 in nodes {
        if n1.used == 0 {
            empties.push(n1.loc);
        }
        for (n1, n2) in nodes.iter().map(|n2| (n1, n2)).filter(|&x| is_viable_pair(x)) {
            if !moveable.insert(n1.loc) {
                multiple_pairs.insert(n1.loc);
            }
            if !moveable.insert(n2.loc) {
                multiple_pairs.insert(n2.loc);
            }
        }
    }

    // return None if our assumptions are violated
    if empties.len() != 1 || multiple_pairs.len() != 1 ||
       empties.get(0) != multiple_pairs.iter().nth(0) {
        // the algorithm is not good enough to handle this :(
        return None;
    }
    let empty_node = empties[0];

    // find grid dimensions
    let (x_max, y_max) = nodes.iter().fold((0, 0), |(x, y), n| (max(x, n.loc.0), max(y, n.loc.1)));
    let width = x_max + 1;
    let height = y_max + 1;
    if width == 1 {
        // goal is already at 0, 0
        return Some(0);
    }

    // build static grid (true if it's a wall, false if it isn't)
    let mut grid = vec![true; (width * height) as usize];
    for (x, y) in moveable {
        grid[(x + y * width) as usize] = false;
    }

    // algorithm might fail if there are unmoveable nodes in the first 2 rows
    if grid.iter().take(2 * width as usize).any(|&x| x) {
        return None;
    }

    // move empty space just to the left of the goal node
    let dist = find_steps(empty_node,
                          (width - 2, 0),
                          |(x, y)| x >= width || y >= height || grid[(x + y * width) as usize]);

    // simple calculation to determine how many more steps
    let steps = (5 * width - 9) as usize;

    dist.map(|x| x + steps)
}

fn find_steps<F>(start: Location, goal: Location, is_wall: F) -> Option<usize>
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
#[test]
fn test_is_viable_pair() {
    let node_0_10 = Node {
        loc: (0, 0),
        used: 0,
        avail: 10,
    };
    let node_10_10 = Node {
        loc: (0, 1),
        used: 10,
        avail: 10,
    };
    let node_11_10 = Node {
        loc: (1, 0),
        used: 11,
        avail: 10,
    };

    // false because first node empty
    assert!(!is_viable_pair((&node_0_10, &node_10_10)));
    assert!(!is_viable_pair((&node_0_10, &node_11_10)));
    // false because nodes identical
    assert!(!is_viable_pair((&node_10_10, &node_10_10)));
    // false because second node doesn't have enough space
    assert!(!is_viable_pair((&node_11_10, &node_10_10)));
    assert!(!is_viable_pair((&node_11_10, &node_0_10)));

    assert!(is_viable_pair((&node_10_10, &node_11_10)));
    assert!(is_viable_pair((&node_10_10, &node_0_10)));
}

#[test]
fn test_find_viable_pairs() {
    const NODE_0_10: Node = Node {
        loc: (0, 0),
        used: 0,
        avail: 10,
    };
    const NODE_10_10: Node = Node {
        loc: (0, 1),
        used: 10,
        avail: 10,
    };
    const NODE_11_10: Node = Node {
        loc: (1, 0),
        used: 11,
        avail: 10,
    };
    let nodes = vec![NODE_0_10, NODE_10_10, NODE_11_10];

    let viable = find_viable_pairs(&nodes);
    assert_eq!(2, viable.len());
}

#[test]
fn test_calc_fewest_steps_to_goal() {
    let nodes = vec![Node {
                         loc: (0, 0),
                         used: 8,
                         avail: 2,
                     },
                     Node {
                         loc: (0, 1),
                         used: 6,
                         avail: 5,
                     },
                     Node {
                         loc: (0, 2),
                         used: 28,
                         avail: 4,
                     },
                     Node {
                         loc: (1, 0),
                         used: 7,
                         avail: 2,
                     },
                     Node {
                         loc: (1, 1),
                         used: 0,
                         avail: 8,
                     },
                     Node {
                         loc: (1, 2),
                         used: 7,
                         avail: 4,
                     },
                     Node {
                         loc: (2, 0),
                         used: 6,
                         avail: 4,
                     },
                     Node {
                         loc: (2, 1),
                         used: 8,
                         avail: 1,
                     },
                     Node {
                         loc: (2, 2),
                         used: 6,
                         avail: 3,
                     }];

    assert_eq!(Some(7), calc_fewest_steps_to_goal(&nodes));
}
