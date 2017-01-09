// advent8.rs
// parsing instructions for small LCD

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;

#[cfg(not(test))]
const SCREEN_WIDTH: usize = 50;
#[cfg(not(test))]
const SCREEN_HEIGHT: usize = 6;

#[cfg(test)]
const SCREEN_WIDTH: usize = 7;
#[cfg(test)]
const SCREEN_HEIGHT: usize = 3;

type LightGrid = [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT];

fn main() {
    let mut grid = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.expect("Failed to read line")) {
        apply_cmd(&line, &mut grid);
    }

    println!("Part 1 light count: {}", count_lights(&grid));
    println!("Part 2 screen display:");
    print_grid(&grid);
}

fn apply_cmd(cmd: &str, grid: &mut LightGrid) {
    lazy_static! {
        static ref RE_RECT: Regex = Regex::new(r"rect (\d+)x(\d+)").unwrap();
        static ref RE_ROW: Regex = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
        static ref RE_COL: Regex = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
    }

    if let Some(caps) = RE_RECT.captures(cmd) {
        let cols: usize = caps[1].parse().unwrap();
        let rows: usize = caps[2].parse().unwrap();

        for row in grid.iter_mut().take(rows) {
            for cell in row.iter_mut().take(cols) {
                *cell = true;
            }
        }
    } else if let Some(caps) = RE_ROW.captures(cmd) {
        let row: usize = caps[1].parse().unwrap();
        let rot: usize = caps[2].parse().unwrap();

        let old_row = grid[row];

        for (i, &pixel) in old_row.iter().enumerate() {
            grid[row][(i + rot) % SCREEN_WIDTH] = pixel;
        }
    } else if let Some(caps) = RE_COL.captures(cmd) {
        let col: usize = caps[1].parse().unwrap();
        let rot: usize = caps[2].parse().unwrap();

        let mut old_col = [false; SCREEN_HEIGHT];
        for i in 0..SCREEN_HEIGHT {
            old_col[i] = grid[i][col];
        }

        for (i, &pixel) in old_col.iter().enumerate() {
            grid[(i + rot) % SCREEN_HEIGHT][col] = pixel;
        }
    }
}

fn count_lights(grid: &LightGrid) -> u32 {
    let mut total = 0;

    for row in grid.iter() {
        for light in row.iter() {
            if *light {
                total += 1;
            }
        }
    }

    total
}

// ///////
// Part 2
fn print_grid(grid: &LightGrid) {
    let mut row_string = String::new();
    for row in grid.iter() {
        row_string.clear();
        for &cell in row.iter() {
            row_string.push(if cell { '#' } else { '.' });
        }
        println!("{}", row_string);
    }
}

// //////
// Tests

#[test]
fn test_apply_cmd() {
    let mut grid = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
    const SOLUTION: LightGrid = [[false, true, false, false, true, false, true],
                                 [true, false, true, false, false, false, false],
                                 [false, true, false, false, false, false, false]];

    apply_cmd("rect 3x2", &mut grid);
    apply_cmd("rotate column x=1 by 1", &mut grid);
    apply_cmd("rotate row y=0 by 4", &mut grid);
    apply_cmd("rotate column x=1 by 1", &mut grid);

    assert_eq!(SOLUTION, grid);
    assert_eq!(6, count_lights(&grid));
}
