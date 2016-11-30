# AdventOfRust2016
Advent of Code 2016 solutions in Rust

See http://adventofcode.com to learn more about Advent of Code.

 Each day's problem comes in 2 parts, which both use the same input data. One problem set is released each day from December 1 to 25.

This project is a set of Advent of Code 2016 solutions implemented in Rust. These solutions are not meant to be the quickest hacks possible nor are they an attempt at the most elegant or optimal solution. They aim to be good examples of how to write Rust code for simple problems. Solutions include unit tests because you should use those. Solutions freely pull in external crates when helpful because most Rust programmers would want to take advantage of Rust's ecosystem.

Feedback is welcome.

## Layout
Solutions are stored in `src/bin/advent*.rs`. Each day's problem set has both parts of its solution along with unit tests in a single source file.

Problem input is always read from standard input. Input files are not committed to source control since everyone's input files are different.

## Running a specific solution
Assuming you store your personal input files as `input/input*.txt`:
```
cargo run --release --bin advent1 < input/input1.txt
```

To run an unoptimized build, omit --release.

## Running unit tests
Run all unit tests:
```
cargo test
```

Run unit tests for a single solution:
```
cargo test --bin advent1
```


