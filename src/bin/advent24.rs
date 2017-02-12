// advent24.rs
// cleaning ducts (like traveling salesman)

extern crate permutohedron;
extern crate common;

use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use common::{Location, find_steps};

fn main() {
    let stdin = io::stdin();
    let input: Vec<_> = stdin.lock()
        .lines()
        .map(|l| l.expect("failed to read line"))
        .collect();

    let distances = PointDistances::from_input(&input).unwrap();

    // part 1
    println!("Part 1 shortest path: {}", distances.find_shortest_path());

    // part 2
    println!("Part 2 shortest path: {}",
             distances.find_shortest_path_return());
}

struct Grid {
    walls: Vec<bool>,
    width: i32,
    height: i32,
    points: Vec<Location>,
}

impl Grid {
    fn from_input(maze_str: &[String]) -> Option<Grid> {
        let height = maze_str.len();
        let width = if height > 0 { maze_str[0].len() } else { 0 };

        // find walls
        let walls = maze_str.iter().flat_map(|l| l.chars().map(|c| c == '#')).collect();

        // find points of interest within the maze
        let mut points = vec![];
        let mut point_map = HashMap::new();
        for (y, line) in maze_str.iter().enumerate() {
            if line.len() != width {
                // don't know what to do with inconsistent width
                return None;
            }
            for (x, c) in line.chars().enumerate() {
                if c >= '0' && c <= '9' {
                    let point_idx = c as usize - '0' as usize;
                    point_map.insert(point_idx, (x as i32, y as i32));
                }
            }
        }
        for point_idx in 0..point_map.len() {
            points.push(point_map[&point_idx]);
        }

        Some(Grid {
            walls: walls,
            height: height as i32,
            width: width as i32,
            points: points,
        })
    }

    fn is_wall(&self, (x, y): Location) -> bool {
        self.walls[(y * self.width + x) as usize] || x < 0 || y < 0 || x >= self.width ||
        y >= self.height
    }
}

struct PointDistances {
    distances: Vec<usize>,
    point_count: usize,
}

impl PointDistances {
    fn from_input(maze_str: &[String]) -> Option<PointDistances> {
        Grid::from_input(maze_str).and_then(|grid| Self::from_grid(&grid))
    }

    fn from_grid(grid: &Grid) -> Option<PointDistances> {
        let point_count = grid.points.len();
        let mut distances = vec![0; point_count*point_count];
        for p1 in 0..point_count {
            for p2 in p1 + 1..point_count {
                if let Some(dist) = find_steps(grid.points[p1],
                                               grid.points[p2],
                                               |point| grid.is_wall(point)) {
                    distances[p1 * point_count + p2] = dist;
                    distances[p2 * point_count + p1] = dist;
                } else {
                    // no path between points, can't handle this
                    return None;
                }
            }
        }

        Some(PointDistances {
            distances: distances,
            point_count: point_count,
        })
    }

    fn get(&self, p1: usize, p2: usize) -> usize {
        self.distances[p1 + p2 * self.point_count]
    }

    // part 1
    fn find_shortest_path(&self) -> usize {
        let mut indices: Vec<_> = (1..self.point_count).collect();
        permutohedron::Heap::new(&mut indices[..])
            .map(|x| self.get(0, x[0]) + x.windows(2).fold(0, |acc, x| acc + self.get(x[0], x[1])))
            .min()
            .unwrap()
    }

    // part 2
    fn find_shortest_path_return(&self) -> usize {
        let mut indices: Vec<_> = (1..self.point_count).collect();
        permutohedron::Heap::new(&mut indices[..])
            .map(|x| {
                self.get(0, x[self.point_count - 2]) + self.get(0, x[0]) +
                x.windows(2).fold(0, |acc, x| acc + self.get(x[0], x[1]))
            })
            .min()
            .unwrap()
    }
}

// //////
// Tests
#[test]
fn test_grid() {
    let input = vec!["###########".to_string(),
                     "#0.1.....2#".to_string(),
                     "#.#######.#".to_string(),
                     "#4.......3#".to_string(),
                     "###########".to_string()];

    let grid = Grid::from_input(&input).unwrap();
    assert_eq!(5, grid.height);
    assert_eq!(11, grid.width);
    assert_eq!((1, 1), grid.points[0]);
    assert_eq!((9, 3), grid.points[3]);
}

#[test]
fn test_distances() {
    let input = vec!["###########".to_string(),
                     "#0.1.....2#".to_string(),
                     "#.#######.#".to_string(),
                     "#4.......3#".to_string(),
                     "###########".to_string()];

    let distances = PointDistances::from_input(&input).unwrap();
    assert_eq!(14, distances.find_shortest_path());
}
