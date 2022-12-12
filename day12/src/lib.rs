use pathfinding::prelude::*;

struct Graph<'a> {
    raw: Vec<&'a [u8]>,
}

struct Iter {
    v: [usize; 4],
    n: u8,
}

impl Iterator for Iter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.n == 4 {
            return None;
        }
        let item = self.v[(self.n as usize) & 0x3];
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
    let g = Graph {
        raw: input
            .as_bytes()
            .split(|&c| c == b'\n')
            .filter(|b| b.len() != 0)
            .collect(),
    };
    let height = g.raw.len();
    let width = g.raw[0].len();
    let mut start = 0;
    let mut target = 0;
    for i in 0..height {
        for j in 0..width {
            if g.raw[i][j] == b'S' {
                start = i * width + j;
            } else if g.raw[i][j] == b'E' {
                target = i * width + j;
            }
        }
    }

    let p = bfs(
        &start,
        |&x| {
            let mut v = [0usize; 4];
            let mut n = 4;
            let row = x / width;
            let col = x % width;
            let h = height_of(g.raw[row][col]);
            if 0 < row && height_of(g.raw[row - 1][col]) <= h + 1 {
                n -= 1;
                v[n] = (row - 1) * width + col;
            }
            if row < height - 1 && height_of(g.raw[row + 1][col]) <= h + 1 {
                n -= 1;
                v[n] = (row + 1) * width + col;
            }
            if 0 < col && height_of(g.raw[row][col - 1]) <= h + 1 {
                n -= 1;
                v[n] = row * width + (col - 1);
            }
            if col < width - 1 && height_of(g.raw[row][col + 1]) <= h + 1 {
                n -= 1;
                v[n] = row * width + (col + 1);
            }
            Iter { v, n: n as u8 }
        },
        |&n| n == target,
    );

    p.unwrap().len() as u32 - 1
}

pub fn part2(input: &str) -> u32 {
    let g = Graph {
        raw: input
            .as_bytes()
            .split(|&c| c == b'\n')
            .filter(|b| b.len() != 0)
            .collect(),
    };
    let height = g.raw.len();
    let width = g.raw[0].len();
    let mut start = 0;
    'outer: for i in 0..height {
        for j in 0..width {
            if g.raw[i][j] == b'E' {
                start = i * width + j;
                break 'outer;
            }
        }
    }

    let p = bfs(
        &start,
        |&x| {
            let mut v = [0usize; 4];
            let mut n = 4;
            let row = x / width;
            let col = x % width;
            let h = height_of(g.raw[row][col]);
            if 0 < row && h - 1 <= height_of(g.raw[row - 1][col]) {
                n -= 1;
                v[n] = (row - 1) * width + col;
            }
            if row < height - 1 && h - 1 <= height_of(g.raw[row + 1][col]) {
                n -= 1;
                v[n] = (row + 1) * width + col;
            }
            if 0 < col && h - 1 <= height_of(g.raw[row][col - 1]) {
                n -= 1;
                v[n] = row * width + (col - 1);
            }
            if col < width - 1 && h - 1 <= height_of(g.raw[row][col + 1]) {
                n -= 1;
                v[n] = row * width + (col + 1);
            }
            Iter { v, n: n as u8 }
        },
        |&x| {
            let row = x / width;
            let col = x % width;
            height_of(g.raw[row][col]) == 0
        },
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
