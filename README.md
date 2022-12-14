# Advent of Code 2022 in Rust

This repo contains solutions to Advent of Code 2022 puzzles in Rust.

[Last year](https://github.com/fyrchik/aoc2021rust) I tried to stay within stable Rust.
This year goals are:
1. Be as fast as possible, possibly sacrificing input validation.
2. Explore experimental features and const generics.
3. Still have little to no dependencies, but be more liberal than I did last year.

# Using

Firstly, put your input to `src/dayNN/input`.

```
# Run puzzle for a specific day.
make run/dayNN

# Run criterion benchmarks for a specific day.
make bench/dayNN
```

# Timings

I copied the structure of this section from https://github.com/timvisee/advent-of-code-2022 . I also recommend that repo for seing some clean Rust code!

Here is how long each solution runs. All solutions are measured (non
scientifically) in [`bench.rs`](./runner/src/bin/bench.rs) on an
`Intel(R) Core(TM) i7-8550U CPU @ 1.80GHz` laptop running Linux. Having that in mind, the timings
are close to those obtained (scientifically) with `cargo criterion`.

|                                                | part A                              | part B                              |
|:-----------------------------------------------|:------------------------------------|:------------------------------------|
| [day 1](https://adventofcode.com/2022/day/1)   | [` 0.011 ms`](./day01/src/lib.rs) | [` 0.012 ms`](./day01/src/lib.rs) |
| [day 2](https://adventofcode.com/2022/day/2)   | [` 0.012 ms`](./day02/src/lib.rs) | [` 0.012 ms`](./day02/src/lib.rs) |
| [day 3](https://adventofcode.com/2022/day/3)   | [` 0.011 ms`](./day03/src/lib.rs) | [` 0.010 ms`](./day03/src/lib.rs) |
| [day 4](https://adventofcode.com/2022/day/4)   | [` 0.020 ms`](./day04/src/lib.rs) | [` 0.020 ms`](./day04/src/lib.rs) |
| [day 5](https://adventofcode.com/2022/day/5)   | [` 0.024 ms`](./day05/src/lib.rs) | [` 0.024 ms`](./day05/src/lib.rs) |
| [day 6](https://adventofcode.com/2022/day/6)   | [` 0.004 ms`](./day06/src/lib.rs) | [` 0.007 ms`](./day06/src/lib.rs) |
| [day 7](https://adventofcode.com/2022/day/7)   | [` 0.018 ms`](./day07/src/lib.rs) | [` 0.018 ms`](./day07/src/lib.rs) |
| [day 8](https://adventofcode.com/2022/day/8)   | [` 0.044 ms`](./day08/src/lib.rs) | [` 0.285 ms`](./day08/src/lib.rs) |
| [day 9](https://adventofcode.com/2022/day/9)   | [` 0.191 ms`](./day09/src/lib.rs) | [` 0.301 ms`](./day09/src/lib.rs) |
| [day 10](https://adventofcode.com/2022/day/10)   | [` 0.002 ms`](./day10/src/lib.rs) | [` 0.005 ms`](./day10/src/lib.rs) |
| [day 11](https://adventofcode.com/2022/day/11)   | [` 0.010 ms`](./day11/src/lib.rs) | [` 1.220 ms`](./day11/src/lib.rs) |
| [day 12](https://adventofcode.com/2022/day/12)   | [` 0.511 ms`](./day12/src/lib.rs) | [` 0.399 ms`](./day12/src/lib.rs) |
| [day 13](https://adventofcode.com/2022/day/13)   | [` 0.026 ms`](./day13/src/lib.rs) | [` 0.029 ms`](./day13/src/lib.rs) |
| [day 14](https://adventofcode.com/2022/day/14)   | [` 0.067 ms`](./day14/src/lib.rs) | [` 0.195 ms`](./day14/src/lib.rs) |
| [day 15](https://adventofcode.com/2022/day/15)   | [` 0.005 ms`](./day15/src/lib.rs) | [` 0.006 ms`](./day15/src/lib.rs) |
| [day 16](https://adventofcode.com/2022/day/16)   | [` 11.45 ms`](./day16/src/lib.rs) | [` 12.73 ms`](./day16/src/lib.rs) |
| [day 17](https://adventofcode.com/2022/day/17)   | [` 0.164 ms`](./day17/src/lib.rs) | [` 0.520 ms`](./day17/src/lib.rs) |
| [day 18](https://adventofcode.com/2022/day/18)   | [` 0.168 ms`](./day18/src/lib.rs) | [` 0.578 ms`](./day18/src/lib.rs) |
| [day 19](https://adventofcode.com/2022/day/19)   | [` 3.20 ms`](./day19/src/lib.rs) | [` 9.37 ms`](./day19/src/lib.rs) |
| [day 20](https://adventofcode.com/2022/day/20)   | [` 3.66 ms`](./day20/src/lib.rs) | [` 71.010 ms`](./day20/src/lib.rs) |
| [day 21](https://adventofcode.com/2022/day/21)   | [` 0.101 ms`](./day21/src/lib.rs) | [` 0.199 ms`](./day21/src/lib.rs) |
| [day 22](https://adventofcode.com/2022/day/22)   | [` 0.085 ms`](./day22/src/lib.rs) | [` 0.163 ms`](./day22/src/lib.rs) |
| [day 23](https://adventofcode.com/2022/day/23)   | [` 0.619 ms`](./day23/src/lib.rs) | [` 81.66 ms`](./day23/src/lib.rs) |
| [day 24](https://adventofcode.com/2022/day/24)   | [` 13.14 ms`](./day24/src/lib.rs) | [` 38.55 ms`](./day24/src/lib.rs) |
| [day 25](https://adventofcode.com/2022/day/25)   | [` 0.008 ms`](./day25/src/lib.rs) | Single mouse click                |


|              | one-by-one (1 CPU core)                  | parallel                                     |
|:-------------|:-----------------------------------------|:---------------------------------------------|
| _everything_ | [`263.20 ms`](./runner/src/bin/runner.rs) | [`132.64 ms`](./runner/src/bin/runner-par.rs) |

## Run all solutions

I don't include input files because of https://old.reddit.com/r/adventofcode/wiki/faqs/copyright/inputs.
This way the benchmarks are not _completely_ reproducible, but I am not going publish a scientific paper anyway.
Simply put your input files and run the project to see the solution appear.

```bash
# Run everything
cd ../runner
cargo +nightly run --release --bin runner

# or run everything in parallel
cd ../runner
cargo +nightly run --release --bin runner-par

# or benchmark every day
cd ../runner
cargo +nightly run --release --bin bench
```

# License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.
