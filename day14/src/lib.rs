#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(iter_array_chunks)]

pub fn part1(input: &str) -> u32 {
    let (mut field, min, max) = parse(input);
    let start = Point {
        x: 500 - min.x,
        y: 0,
    };

    let mut count = 0;
    while let Some(p) = sieve(&field, start, max) {
        let c = &mut field[p.y][p.x];

        assert_eq!(State::Air, *c);
        *c = State::Sand;

        count += 1
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let (mut field, min, mut max) = parse(input);

    let minx = min.x.min(500 - max.y - 2);
    let maxx = max.x.max(500 + max.y + 2);

    field.iter_mut().for_each(|row| {
        let mut new_row = vec![State::Air; maxx - minx + 1];
        for i in 0..row.len() {
            new_row[min.x - minx + i] = row[i];
        }
        *row = new_row;
    });
    field.push(vec![State::Air; maxx - minx + 1]);
    field.push(vec![State::Rock; maxx - minx + 1]);

    max.y += 2;
    let start = Point {
        x: 500 - minx,
        y: 0,
    };

    let mut count = 0;
    while let Some(p) = sieve(&field, start, max) {
        if p == start {
            break;
        }
        let c = &mut field[p.y][p.x];
        *c = State::Sand;

        count += 1
    }
    count + 1
}

#[allow(dead_code)]
fn print_field(field: &[Vec<State>]) {
    field.iter().for_each(|row| {
        row.iter().for_each(|c| match c {
            State::Air => print!("."),
            State::Rock => print!("#"),
            State::Sand => print!("o"),
        });
        println!();
    })
}

fn sieve(field: &[Vec<State>], start: Point, max: Point) -> Option<Point> {
    let mut current = start;
    loop {
        let x = current.x;
        let y = current.y + 1;
        if y > max.y {
            return None;
        }
        match field[y][x] {
            State::Air => {
                current.y += 1;
                continue;
            }
            State::Rock | State::Sand => {
                if current.x > 0 {
                    let x = current.x - 1;
                    if field[y][x] == State::Air {
                        current.x -= 1;
                        current.y += 1;
                        continue;
                    }
                } else {
                    return None;
                }
                let x = current.x + 1;
                if x <= max.x {
                    if field[y][x] == State::Air {
                        current.x += 1;
                        current.y += 1;
                        continue;
                    }

                    return Some(current);
                }
            }
        }
        return None;
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

fn parse(input: &str) -> (Vec<Vec<State>>, Point, Point) {
    let mut min = Point {
        x: usize::MAX,
        y: usize::MAX,
    };
    let mut max = Point {
        x: usize::MIN,
        y: usize::MIN,
    };
    let points: Vec<Vec<Point>> = input
        .lines()
        .map(|s| {
            s.split(" -> ")
                .map(|p| {
                    let pt = p.split_once(',').unwrap();
                    let x = pt.0.parse().unwrap();
                    let y = pt.1.parse().unwrap();
                    min.x = min.x.min(x);
                    max.x = max.x.max(x);
                    max.y = max.y.max(y);
                    Point { x, y }
                })
                .collect()
        })
        .collect();

    let mut field: Vec<Vec<State>> = Vec::with_capacity(max.y + 1);
    for _ in 0..=max.y {
        field.push(vec![State::Air; max.x - min.x + 1]);
    }

    for l in points.iter() {
        l.array_windows::<2>().for_each(|p| {
            if p[0].x == p[1].x {
                for y in p[0].y.min(p[1].y)..=p[0].y.max(p[1].y) {
                    field[y][p[0].x - min.x] = State::Rock;
                }
            } else {
                debug_assert_eq!(p[0].y, p[1].y);

                for x in p[0].x.min(p[1].x)..=p[0].x.max(p[1].x) {
                    field[p[0].y][x - min.x] = State::Rock;
                }
            }
        })
    }

    (field, min, max)
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
