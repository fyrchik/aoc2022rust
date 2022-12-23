#[derive(Clone, Copy, Debug)]
enum Point {
    Elf,
    Empty,
    Intention(usize),
    Blocked,
}

impl Point {
    #[inline]
    fn is_empty(&self) -> bool {
        matches!(*self, Point::Empty)
    }
}

#[derive(Copy, Clone)]
enum Dir {
    N,
    S,
    W,
    E,
}

const DIRECTIONS: [Dir; 4] = [Dir::N, Dir::S, Dir::W, Dir::E];

pub fn part1(input: &str) -> u32 {
    const ROUNDS: usize = 10;

    let raw = input.trim_end().as_bytes();
    let width = raw.iter().position(|c| *c == b'\n').unwrap() + ROUNDS;
    let mut field = vec![Point::Empty; width * ROUNDS];
    field.extend(raw.split(|c| *c == b'\n').flat_map(|b| {
        [Point::Empty; ROUNDS / 2]
            .into_iter()
            .chain(
                b.iter()
                    .map(|c| if *c == b'#' { Point::Elf } else { Point::Empty }),
            )
            .chain([Point::Empty; ROUNDS / 2])
    }));
    field.extend(std::iter::repeat(Point::Empty).take(width * ROUNDS));

    let mut next = vec![Point::Empty; field.len()];

    for round in 0..ROUNDS {
        let direction = round % 4;
        for i in field
            .iter()
            .enumerate()
            .filter_map(|(i, p)| matches!(*p, Point::Elf).then_some(i))
        {
            assert!(i + width + 1 < field.len());

            let n11 = field[i - width - 1].is_empty();
            let n12 = field[i - width].is_empty();
            let n13 = field[i - (width - 1)].is_empty();
            let n21 = field[i - 1].is_empty();
            let n23 = field[i + 1].is_empty();
            let n31 = field[i + width - 1].is_empty();
            let n32 = field[i + width].is_empty();
            let n33 = field[i + width + 1].is_empty();
            let target = [
                n11 && n12 && n13,
                n31 && n32 && n33,
                n11 && n21 && n31,
                n13 && n23 && n33,
            ];

            let all_empty_or_full = target.iter().all(|v| *v) || target.iter().all(|v| !*v);
            if all_empty_or_full {
                next[i] = Point::Elf;
                continue;
            }

            let d = direction + (0..4).position(|d| target[(direction + d) % 4]).unwrap();
            let first_empty = match DIRECTIONS[d % 4] {
                Dir::N => i - width,
                Dir::S => i + width,
                Dir::W => i - 1,
                Dir::E => i + 1,
            };
            match next[first_empty] {
                Point::Empty => next[first_empty] = Point::Intention(i),
                Point::Intention(x) => {
                    next[first_empty] = Point::Blocked;
                    next[i] = Point::Elf;
                    next[x] = Point::Elf;
                }
                Point::Blocked => next[i] = Point::Elf,
                _ => unreachable!(),
            }
        }

        for p in next.iter_mut() {
            match *p {
                Point::Intention(_) => *p = Point::Elf,
                Point::Blocked => *p = Point::Empty,
                _ => {}
            }
        }

        field.fill(Point::Empty);
        (next, field) = (field, next);
    }

    let mut min_col = usize::MAX;
    let mut max_col = usize::MIN;
    let mut min_row = usize::MAX;
    let mut max_row = usize::MIN;
    let mut count = 0;
    for i in field
        .iter()
        .enumerate()
        .filter_map(|p| matches!(p.1, Point::Elf).then_some(p.0))
    {
        min_col = min_col.min(i % width);
        max_col = max_col.max(i % width);
        min_row = min_row.min(i / width);
        max_row = max_row.max(i / width);
        count += 1;
    }
    ((max_row - min_row + 1) * (max_col - min_col + 1) - count) as u32
}

pub fn part2(input: &str) -> u32 {
    let raw = input.trim_end().as_bytes();
    let mut field: Vec<_> = raw
        .split(|c| *c == b'\n')
        .flat_map(|b| {
            b.iter()
                .map(|c| if *c == b'#' { Point::Elf } else { Point::Empty })
        })
        .collect();

    let mut width = raw.iter().position(|c| *c == b'\n').unwrap();
    let mut height = field.len() / width;
    let mut next_width = width + 2;
    let mut next_height = height + 2;
    let mut next = vec![Point::Empty; next_width * next_height];
    let mut round = 0;
    let mut last_update = [true; 4];
    loop {
        let direction = round % 4;
        round += 1;

        let additive = (last_update[0] as usize) * next_width + last_update[2] as usize;

        for i in field
            .iter()
            .enumerate()
            .filter_map(|(i, p)| matches!(*p, Point::Elf).then_some(i))
        {
            let n11 = i % width == 0 || i < width + 1 || field[i - width - 1].is_empty();
            let n12 = i < width || field[i - width].is_empty();
            let n13 = i % width == width - 1 || i < width - 1 || field[i - (width - 1)].is_empty();
            let n21 = i % width == 0 || field[i - 1].is_empty();
            let n23 = i % width == width - 1 || i % width == (width - 1) || field[i + 1].is_empty();
            let n31 =
                i % width == 0 || field.len() <= i + width - 1 || field[i + width - 1].is_empty();
            let n32 = field.len() <= i + width || field[i + width].is_empty();
            let n33 = i % width == width - 1
                || field.len() <= i + width + 1
                || field[i + width + 1].is_empty();
            let ii = transform(i, width, next_width, additive);
            let target = [
                n11 && n12 && n13,
                n31 && n32 && n33,
                n11 && n21 && n31,
                n13 && n23 && n33,
            ];

            let all_empty_or_full = target.iter().all(|v| *v) || target.iter().all(|v| !*v);
            if all_empty_or_full {
                next[ii] = Point::Elf;
                continue;
            }

            let d = direction + (0..4).position(|d| target[(direction + d) % 4]).unwrap();
            let first_empty = match DIRECTIONS[d % 4] {
                Dir::N => ii - next_width,
                Dir::S => ii + next_width,
                Dir::W => ii - 1,
                Dir::E => ii + 1,
            };

            match next[first_empty] {
                Point::Empty => next[first_empty] = Point::Intention(ii),
                Point::Intention(x) => {
                    next[first_empty] = Point::Blocked;
                    next[ii] = Point::Elf;
                    next[x] = Point::Elf;
                }
                Point::Blocked => next[ii] = Point::Elf,
                _ => unreachable!(),
            }
        }

        let mut moved = 0;
        for p in next.iter_mut() {
            match *p {
                Point::Intention(_) => {
                    *p = Point::Elf;
                    moved += 1;
                }
                Point::Blocked => *p = Point::Empty,
                _ => {}
            }
        }

        if moved == 0 {
            return round as u32;
        }

        let update = [
            next.iter() // Do we need to add a row in the North?
                .take(next_width)
                .any(|c| !matches!(*c, Point::Empty)),
            next.iter() // Do we need to add a row in the South?
                .skip(next_width * (next_height - 1))
                .any(|c| !matches!(*c, Point::Empty)),
            next.iter() // Do we need to add a column in the West?
                .step_by(next_width)
                .any(|c| !matches!(*c, Point::Empty)),
            next.iter() // Do we need to add a column in the East?
                .skip(next_width - 1)
                .step_by(next_width)
                .any(|c| !matches!(*c, Point::Empty)),
        ];

        width = next_width + update[2] as usize + update[3] as usize;
        height = next_height + update[0] as usize + update[1] as usize;

        field.fill(Point::Empty);
        if width * height > field.len() {
            let diff = width * height - field.len();
            field.extend(std::iter::repeat(Point::Empty).take(diff));
        }

        (next, field) = (field, next);
        (width, next_width) = (next_width, width);
        next_height = height;
        last_update = update;
    }
}

#[inline]
fn transform(n: usize, width: usize, next_width: usize, additive: usize) -> usize {
    (n / width) * next_width + (n % width) + additive
}

#[allow(dead_code)]
fn print_field(field: &[Point], width: usize) {
    for (i, p) in field.iter().enumerate() {
        print!(
            "{}",
            match *p {
                Point::Empty => '.',
                Point::Elf => '#',
                Point::Blocked => 'x',
                Point::Intention(_) => '+',
            }
        );
        if i % width == width - 1 {
            println!();
        }
    }
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
        let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

        assert_eq!(110, part1(&input));
        assert_eq!(20, part2(&input));
    }
}
