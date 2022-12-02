# Advent of Code 2022 in Rust

This repo contains solutions to Advent of Code 2022 puzzles in Rust.

[Last year](https://github.com/fyrchik/aoc2021rust) I tried to stay within stable Rust.
This year goals are:
1. Be as fast as possible, possibly sacrificing input validation. Explore experimental features and const generics.
2. Still have little to no dependencies, but be more liberal than I did last year.

# Using

Firstly, put your input to `src/dayNN/input`.

```
# Run puzzle for a specific day.
make run/dayNN

# Run criterion benchmarks for a specific day.
make bench/dayNN
```
