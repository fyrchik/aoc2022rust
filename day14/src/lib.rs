#![feature(array_windows)]

pub fn part1(input: &str) -> u32 {
    let (mut field, min_x, max) = parse(input, false);
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
        if let Some(p) = sieve(&field, &mut trail, start, max) {
            let c = &mut field[p.y][p.x];

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
    let (mut field, min_x, max) = parse(input, true);
    let initial = Point {
        x: 500 - min_x,
        y: 0,
    };

    let mut count = 0;
    let mut trail = vec![initial; 1];
    while let Some(start) = trail.pop() {
        if let Some(p) = sieve(&field, &mut trail, start, max) {
            let c = &mut field[p.y][p.x];

            debug_assert_eq!(State::Air, *c);
            *c = State::Sand;

            let x = trail.pop();
            debug_assert_eq!(Some(p), x);

            count += 1
        } else {
            unreachable!();
        }
    }
    count
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

fn sieve(field: &[Vec<State>], trail: &mut Vec<Point>, start: Point, max: Point) -> Option<Point> {
    let mut current = start;
    trail.push(current);
    loop {
        let x = current.x;
        let y = current.y + 1;
        if y > max.y {
            return None;
        }
        match field[y][x] {
            State::Air => {
                current.y += 1;
                trail.push(current);
                continue;
            }
            State::Rock | State::Sand => {
                if current.x > 0 {
                    let x = current.x - 1;
                    if field[y][x] == State::Air {
                        current.x -= 1;
                        current.y += 1;
                        trail.push(current);
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
                        trail.push(current);
                        continue;
                    }

                    return Some(current);
                } else {
                    return None;
                }
            }
        }
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

fn parse(input: &str, is_second_part: bool) -> (Vec<Vec<State>>, usize, Point) {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    let points: Vec<Vec<Point>> = input
        .lines()
        .map(|s| {
            s.split(" -> ")
                .map(|p| {
                    let pt = p.split_once(',').unwrap();
                    let x = pt.0.parse().unwrap();
                    let y = pt.1.parse().unwrap();
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                    Point { x, y }
                })
                .collect()
        })
        .collect();

    let mut height = max_y + 1;
    if is_second_part {
        min_x = min_x.min(500 - max_y - 2);
        max_x = max_x.max(500 + max_y + 2);
        height += 2;
    }

    let mut field: Vec<Vec<State>> = Vec::with_capacity(height);
    for _ in 0..=max_y {
        field.push(vec![State::Air; max_x - min_x + 1]);
    }

    for l in points.iter() {
        l.array_windows::<2>().for_each(|p| {
            if p[0].x == p[1].x {
                for y in p[0].y.min(p[1].y)..=p[0].y.max(p[1].y) {
                    field[y][p[0].x - min_x] = State::Rock;
                }
            } else {
                debug_assert_eq!(p[0].y, p[1].y);

                for x in p[0].x.min(p[1].x)..=p[0].x.max(p[1].x) {
                    field[p[0].y][x - min_x] = State::Rock;
                }
            }
        })
    }

    if is_second_part {
        field.push(vec![State::Air; max_x - min_x + 1]);
        field.push(vec![State::Rock; max_x - min_x + 1]);
        max_y += 2;
    }

    (field, min_x, Point { x: max_x, y: max_y })
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
