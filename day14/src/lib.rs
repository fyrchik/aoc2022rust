#![feature(array_windows)]

use std::collections::VecDeque;

pub fn part1(input: &str) -> u32 {
    let (mut field, min_x, width, depth) = parse(input, false);
    let initial = Point {
        x: 500 - min_x,
        y: 0,
    };

    let mut count = 0;
    // Trail contains all visited cells on a field.
    // After calling `sieve` the last item will be the position of a node.
    let mut trail = vec![initial; 1];
    loop {
        let start = trail.pop().unwrap();
        if let Some(p) = sieve(&field, &mut trail, start, width, depth) {
            let c = &mut field[p.y * width + p.x];

            debug_assert_eq!(State::Air, *c);
            *c = State::Sand;

            let x = trail.pop();
            debug_assert_eq!(Some(p), x);

            count += 1
        } else {
            break;
        }
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let (mut field, min_x, width, depth) = parse(input, true);
    let initial = Point {
        x: 500 - min_x,
        y: 0,
    };

    bfs(&mut field, initial, width, depth) as u32
}

#[allow(dead_code)]
fn print_field(field: &[State], width: usize) {
    for (i, s) in field.iter().enumerate() {
        if i % width == 0 {
            println!();
        }
        match *s {
            State::Air => print!("."),
            State::Rock => print!("#"),
            State::Sand => print!("o"),
        }
    }
}

fn bfs(field: &mut [State], start: Point, width: usize, depth: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    field[start.y * width + start.x] = State::Sand;

    let mut count = 0;
    while let Some(p) = queue.pop_front() {
        count += 1;

        let y = p.y + 1;
        if depth < y {
            continue;
        }
        let base = y * width + p.x;
        if field[base] == State::Air {
            field[base] = State::Sand;
            queue.push_back(Point { x: p.x, y });
        }
        if 0 < p.x && field[base - 1] == State::Air {
            field[base - 1] = State::Sand;
            queue.push_back(Point { x: p.x - 1, y });
        }
        if p.x + 1 < width && field[base + 1] == State::Air {
            field[base + 1] = State::Sand;
            queue.push_back(Point { x: p.x + 1, y });
        }
    }
    count
}

fn sieve(
    field: &[State],
    trail: &mut Vec<Point>,
    start: Point,
    width: usize,
    depth: usize,
) -> Option<Point> {
    let mut current = start;
    loop {
        trail.push(current);

        let y = current.y + 1;
        if depth < y {
            return None;
        }

        if field[y * width + current.x] == State::Air {
            current.y += 1;
            continue;
        }

        if current.x == 0 {
            return None;
        }

        if field[y * width + current.x - 1] == State::Air {
            current.x -= 1;
            current.y += 1;
            continue;
        }

        if width <= current.x + 1 {
            return None;
        }

        if field[y * width + current.x + 1] == State::Air {
            current.x += 1;
            current.y += 1;
            continue;
        }

        return Some(current);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Air,
    Rock,
    Sand,
}

fn parse(input: &str, is_second_part: bool) -> (Vec<State>, usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    let mut points = vec![];
    input.as_bytes().split(|c| *c == b'\n').for_each(|b| {
        let mut i = 0;
        while i < b.len() {
            let (x, x_len) = aoc::uint_from_bytes_prefix::<usize>(&b[i..]);
            i += x_len + 1;

            let (y, y_len) = aoc::uint_from_bytes_prefix::<usize>(&b[i..]);
            i += y_len + 4; // + " -> "

            min_x = min_x.min(x);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            points.push(Point { x, y });
        }
        points.push(Point {
            x: usize::MAX,
            y: usize::MAX,
        });
    });

    if is_second_part {
        min_x = min_x.min(500 - max_y - 2);
        max_x = max_x.max(500 + max_y + 2);
        max_y += 2;
    }
    let width = max_x - min_x + 1;

    let mut field: Vec<State> = vec![State::Air; (max_y + 1) * width];

    for l in points.split(|p| p.x == usize::MAX && p.y == usize::MAX) {
        l.array_windows::<2>().for_each(|p| {
            if p[0].x == p[1].x {
                let x = p[0].x - min_x;
                for y in p[0].y.min(p[1].y)..=p[0].y.max(p[1].y) {
                    field[y * width + x] = State::Rock;
                }
            } else {
                debug_assert_eq!(p[0].y, p[1].y);

                let y = p[0].y * width;
                for x in p[0].x.min(p[1].x)..=p[0].x.max(p[1].x) {
                    field[y + x - min_x] = State::Rock;
                }
            }
        })
    }

    if is_second_part {
        field
            .iter_mut()
            .skip(max_y * width)
            .for_each(|c| *c = State::Rock);
    }

    (field, min_x, width, max_y)
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
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        assert_eq!(24, part1(&input));
        assert_eq!(93, part2(&input));
    }
}
