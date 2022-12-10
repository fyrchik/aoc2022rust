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
| [day 9](https://adventofcode.com/2022/day/9)   | [` 0.320 ms`](./day08/src/lib.rs) | [` 0.420 ms`](./day09/src/lib.rs) |
| [day 10](https://adventofcode.com/2022/day/10)   | [` 0.002 ms`](./day08/src/lib.rs) | [` 0.005 ms`](./day10/src/lib.rs) |


|              | one-by-one (1 CPU core)                  | parallel                                     |
|:-------------|:-----------------------------------------|:---------------------------------------------|
| _everything_ | [`1.710 ms`](./runner/src/bin/runner.rs) | [`0.917 ms`](./runner/src/bin/runner-par.rs) |

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
