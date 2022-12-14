#![feature(array_windows)]

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

    let mut count = 0;
    let mut trail = vec![initial; 1];
    while let Some(start) = trail.pop() {
        if let Some(p) = sieve(&field, &mut trail, start, width, depth) {
            let c = &mut field[p.y * width + p.x];

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
            let (x, x_len) = int_from_bytes_prefix::<usize>(&b[i..]);
            i += x_len + 1;

            let (y, y_len) = int_from_bytes_prefix::<usize>(&b[i..]);
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

fn int_from_bytes_prefix<T>(s: &[u8]) -> (T, usize)
where
    T: From<u8> + std::ops::MulAssign + std::ops::AddAssign,
{
    let mut n = T::from(0);
    for (i, &c) in s.iter().enumerate() {
        let r = match c {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            _ => return (n, i),
        };
        n *= T::from(10);
        n += T::from(r);
    }
    (n, s.len())
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
