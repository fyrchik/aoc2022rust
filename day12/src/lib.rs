use pathfinding::prelude::*;

// Iter represents an iterator over nodes neighbours.
// It is constant in size and we should perform no allocations when returning it.
struct Iter {
    v: [usize; 4],
    n: usize,
}

impl Iterator for Iter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.n == 4 {
            return None;
        }
        // I didn't check `& 0x3` part, but this way we _could_ omit bound checks in theory.
        let item = self.v[self.n & 0x3];
        self.n += 1;
        Some(item)
    }
}

fn height_of(b: u8) -> u8 {
    match b {
        b'a'..=b'z' => b - b'a',
        b'S' => 0,
        b'E' => b'z' - b'a',
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> u32 {
    let g = input.as_bytes();

    // Don't perform complex row/col arithmetic, just assume `\n` are unreachable points.
    let width = g.iter().position(|&c| c == b'\n').unwrap() + 1;
    let mut start = 0;
    let mut target = 0;
    for (i, &c) in g.iter().enumerate() {
        if c == b'S' {
            start = i;
        } else if c == b'E' {
            target = i;
        }
    }

    let p = bfs(
        &start,
        |&x| {
            let mut v = [0usize; 4];
            let mut n = 4;
            let h = height_of(g[x]);
            if width <= x && g[x - width] != b'\n' && height_of(g[x - width]) <= h + 1 {
                n -= 1;
                v[n] = x - width;
            }
            if x + width < g.len() && g[x + width] != b'\n' && height_of(g[x + width]) <= h + 1 {
                n -= 1;
                v[n] = x + width;
            }
            if 0 < x % width && g[x - 1] != b'\n' && height_of(g[x - 1]) <= h + 1 {
                n -= 1;
                v[n] = x - 1;
            }
            // Perform second check because input can miss trailing '\n'.
            if x % width + 1 < width
                && x + 1 < g.len()
                && g[x + 1] != b'\n'
                && height_of(g[x + 1]) <= h + 1
            {
                n -= 1;
                v[n] = x + 1;
            }
            Iter { v, n }
        },
        |&n| n == target,
    );

    p.unwrap().len() as u32 - 1
}

pub fn part2(input: &str) -> u32 {
    let g = input.as_bytes();
    let width = g.iter().position(|&c| c == b'\n').unwrap() + 1;
    let start = g.iter().position(|&c| c == b'E').unwrap();

    let p = bfs(
        &start,
        |&x| {
            let mut v = [0usize; 4];
            let mut n = 4;
            let h = height_of(g[x]);
            if width <= x && g[x - width] != b'\n' && h - 1 <= height_of(g[x - width]) {
                n -= 1;
                v[n] = x - width;
            }
            if x + width < g.len() && g[x + width] != b'\n' && h - 1 <= height_of(g[x + width]) {
                n -= 1;
                v[n] = x + width;
            }
            if 0 < x % width && g[x - 1] != b'\n' && h - 1 <= height_of(g[x - 1]) {
                n -= 1;
                v[n] = x - 1;
            }
            // Perform second check because input can miss trailing '\n'.
            if x % width + 1 < width
                && x + 1 < g.len()
                && g[x + 1] != b'\n'
                && h - 1 <= height_of(g[x + 1])
            {
                n -= 1;
                v[n] = x + 1;
            }
            Iter { v, n }
        },
        |&x| height_of(g[x]) == 0,
    );

    p.unwrap().len() as u32 - 1
}

pub fn run_part1() {
    println!("{}", part1(include_str!("../input")));
}

pub fn run_part2() {
    println!("{}", part2(include_str!("../input")));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!(31, part1(&input));
        assert_eq!(29, part2(&input));
    }
}
