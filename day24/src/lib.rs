use rustc_hash::FxHashSet;

#[derive(Copy, Clone, Default)]
struct Blizzard {
    up: u8,
    down: u8,
    left: u8,
    right: u8,
}

impl Blizzard {
    fn is_empty(&self) -> bool {
        (self.up | self.down | self.left | self.right) == 0
    }
}

pub fn part1(input: &str) -> u32 {
    let mut field = parse(input);

    let start_x = 0;
    let finish_x = field.last().unwrap().len() - 1;
    let finish_y = field.len();
    let mut next = field.clone();

    let mut minute = 0;
    let mut positions = vec![(start_x, usize::MAX)];
    let mut seen = rustc_hash::FxHashSet::<(usize, usize)>::default();
    loop {
        advance(&field, &mut next);
        minute += 1;

        let until = positions.len();
        seen.clear();
        for i in 0..until {
            let p = positions[i];
            if p.1 == usize::MAX {
                if next[0][p.0].is_empty() {
                    if seen.insert((p.0, 0)) {
                        positions.push((p.0, 0))
                    }
                }
                positions.push(p);
            } else {
                assert!(field[p.1][p.0].is_empty());
                if p.0 == finish_x && p.1 + 1 == finish_y {
                    return minute;
                }
                normal_step(p, &next, &mut positions, &mut seen);
            }
        }

        let ln = positions.len();
        positions.copy_within(until..ln, 0);
        positions.truncate(ln - until);

        (next, field) = (field, next);
    }
}

pub fn part2(input: &str) -> u32 {
    let mut field = parse(input);

    let start_x = 0;
    let finish_x = field[0].len() - 1;
    let finish_y = field.len();
    let mut next = field.clone();

    let mut minute = 0;
    let mut positions = vec![(start_x, usize::MAX)];
    let mut seen = rustc_hash::FxHashSet::<(usize, usize)>::default();
    let mut step = 0;
    'outer: loop {
        advance(&field, &mut next);
        minute += 1;

        let until = positions.len();
        seen.clear();

        for i in 0..until {
            let p = positions[i];
            if p.1 == usize::MAX {
                assert!(p.0 == start_x || p.0 == finish_x);
                if (step == 0 || step == 2) && next[0][p.0].is_empty() {
                    if seen.insert((p.0, 0)) {
                        positions.push((p.0, 0));
                    }
                } else if step == 1 && next[next.len() - 1][p.0].is_empty() {
                    if seen.insert((p.0, next.len() - 1)) {
                        positions.push((p.0, next.len() - 1));
                    }
                }
                positions.push(p);
            } else {
                assert!(field[p.1][p.0].is_empty());
                if step == 1 && p.0 == start_x && p.1 == 0 {
                    step = 2;
                    positions[0] = (p.0, usize::MAX);
                    positions.truncate(1);
                    (next, field) = (field, next);
                    continue 'outer;
                } else if p.0 == finish_x && p.1 + 1 == finish_y {
                    if step == 2 {
                        return minute;
                    } else if step == 0 {
                        step = 1;
                        positions[0] = (p.0, usize::MAX);
                        positions.truncate(1);
                        (next, field) = (field, next);
                        continue 'outer;
                    }
                }
                normal_step(p, &next, &mut positions, &mut seen);
            }
        }

        let ln = positions.len();
        positions.copy_within(until..ln, 0);
        positions.truncate(ln - until);

        (next, field) = (field, next);
    }
}

fn normal_step(
    from: (usize, usize),
    field: &[Vec<Blizzard>],
    positions: &mut Vec<(usize, usize)>,
    seen: &mut FxHashSet<(usize, usize)>,
) {
    if 0 < from.0 && field[from.1][from.0 - 1].is_empty() && seen.insert((from.0 - 1, from.1)) {
        positions.push((from.0 - 1, from.1));
    }
    if from.0 < field[from.1].len() - 1
        && field[from.1][from.0 + 1].is_empty()
        && seen.insert((from.0 + 1, from.1))
    {
        positions.push((from.0 + 1, from.1));
    }
    if 0 < from.1 && field[from.1 - 1][from.0].is_empty() && seen.insert((from.0, from.1 - 1)) {
        positions.push((from.0, from.1 - 1))
    }
    if from.1 < field.len() - 1
        && field[from.1 + 1][from.0].is_empty()
        && seen.insert((from.0, from.1 + 1))
    {
        positions.push((from.0, from.1 + 1))
    }
    if field[from.1][from.0].is_empty() && seen.insert((from.0, from.1)) {
        positions.push(from)
    }
}

fn parse(input: &str) -> Vec<Vec<Blizzard>> {
    input
        .trim_end()
        .as_bytes()
        .split(|c| *c == b'\n')
        .skip(1)
        .filter_map(|b| {
            if b[1] == b'#' {
                return None;
            }
            Some(
                b.iter()
                    .skip(1)
                    .take(b.len() - 2)
                    .map(|c| match *c {
                        b'.' => Blizzard::default(),
                        b'>' => Blizzard {
                            right: 1,
                            ..Default::default()
                        },
                        b'<' => Blizzard {
                            left: 1,
                            ..Default::default()
                        },
                        b'^' => Blizzard {
                            up: 1,
                            ..Default::default()
                        },
                        b'v' => Blizzard {
                            down: 1,
                            ..Default::default()
                        },
                        _ => unreachable!(),
                    })
                    .collect(),
            )
        })
        .collect()
}

#[allow(dead_code)]
fn print_field(f: &[Vec<Blizzard>]) {
    for row in f.iter() {
        for c in row.iter() {
            let sum = c.up + c.down + c.left + c.right;
            if sum == 0 {
                print!(".");
            } else if sum != 1 {
                print!("{}", sum);
            } else if c.up == 1 {
                print!("^");
            } else if c.down == 1 {
                print!("v");
            } else if c.left == 1 {
                print!("<");
            } else {
                print!(">");
            }
        }
        println!();
    }
}

fn advance(curr: &[Vec<Blizzard>], next: &mut [Vec<Blizzard>]) {
    for i in 0..next.len() {
        for j in 0..next[i].len() {
            next[i][j] = Blizzard {
                up: curr[(i + 1) % curr.len()][j].up,
                down: curr[(i + curr.len() - 1) % curr.len()][j].down,
                left: curr[i][(j + 1) % curr[i].len()].left,
                right: curr[i][(j + curr[i].len() - 1) % curr[i].len()].right,
            }
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
        let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

        assert_eq!(18, part1(&input));
        assert_eq!(54, part2(&input));
    }
}
