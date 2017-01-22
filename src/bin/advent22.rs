// advent22.rs
// disk space (sliding blocks puzzle)

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() {
    let stdin = io::stdin();

    let nodes: Vec<_> = stdin.lock()
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .filter_map(|l| node_from_str(&l))
        .collect();

    let viable = find_viable_pairs(&nodes);
    println!("part 1: {} viable pairs", viable.len());
}

#[derive(PartialEq)]
struct Node {
    x: usize,
    y: usize,
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
            x: x,
            y: y,
            used: used,
            avail: avail,
        }
    })
}

// //////
// Tests
#[test]
fn test_is_viable_pair() {
    let node_0_10 = Node {
        x: 0,
        y: 0,
        used: 0,
        avail: 10,
    };
    let node_10_10 = Node {
        x: 0,
        y: 1,
        used: 10,
        avail: 10,
    };
    let node_11_10 = Node {
        x: 1,
        y: 0,
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
        x: 0,
        y: 0,
        used: 0,
        avail: 10,
    };
    const NODE_10_10: Node = Node {
        x: 0,
        y: 1,
        used: 10,
        avail: 10,
    };
    const NODE_11_10: Node = Node {
        x: 1,
        y: 0,
        used: 11,
        avail: 10,
    };
    let nodes = vec![NODE_0_10, NODE_10_10, NODE_11_10];


    let viable = find_viable_pairs(&nodes);
    assert_eq!(2, viable.len());
}
