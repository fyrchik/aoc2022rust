pub fn part1(input: &str) -> u32 {
    let mut field = vec![0u8; 10];
    let mut highest_row = 0;
    let jets = input.trim_end().as_bytes();
    let mut jet_index = 0usize;

    'outer: for i in 0..2022 {
        let mut bottom = highest_row + 3;
        for _ in field.len()..bottom + 4 {
            field.push(0);
        }
        let mut left = 2;
        let r = match i % 5 {
            0 => Rock::HLine,
            1 => Rock::Crest,
            2 => Rock::Corner,
            3 => Rock::VLine,
            4 => Rock::Square,
            _ => unreachable!(),
        };

        bottom -= 3;
        for _ in 0..3 {
            match jets[jet_index % jets.len()] {
                b'<' => {
                    if can_move(&field, r, Dir::Left, bottom, left) {
                        left -= 1
                    }
                }
                b'>' => {
                    if can_move(&field, r, Dir::Right, bottom, left) {
                        left += 1
                    }
                }
                _ => unreachable!(),
            }
            jet_index += 1;
        }
        loop {
            match jets[jet_index % jets.len()] {
                b'<' => {
                    if can_move(&field, r, Dir::Left, bottom, left) {
                        left -= 1
                    }
                }
                b'>' => {
                    if can_move(&field, r, Dir::Right, bottom, left) {
                        left += 1
                    }
                }
                _ => unreachable!(),
            };
            jet_index += 1;

            let dir = Dir::Down;
            if !can_move(&field, r, dir, bottom, left) {
                let row = freeze(&mut field, r, bottom, left);
                if row + 1 > highest_row {
                    highest_row = row + 1;
                }
                continue 'outer;
            } else {
                bottom -= 1;
            }
        }
    }

    highest_row as u32
}

pub fn part2(input: &str) -> u64 {
    use rustc_hash::FxHashMap;

    let mut field = vec![0u8; 10];
    let mut profile = [0usize; 7];
    let mut highest_row = 0usize;
    let jets = input.trim_end().as_bytes();
    let mut jet_index = 0usize;
    let mut seen: FxHashMap<([usize; 7], usize, Rock), (u64, u64)> = FxHashMap::default();
    let mut addendum = 0u64;

    const ITER_COUNT: u64 = 1000000000000;

    let mut i = 0;
    'outer: while i < ITER_COUNT {
        let mut bottom = highest_row + 3;
        for _ in field.len()..bottom + 4 {
            field.push(0);
        }
        let mut left = 2;
        let r = match i % 5 {
            0 => Rock::HLine,
            1 => Rock::Crest,
            2 => Rock::Corner,
            3 => Rock::VLine,
            4 => Rock::Square,
            _ => unreachable!(),
        };

        bottom -= 3;
        for _ in 0..3 {
            match jets[jet_index % jets.len()] {
                b'<' => {
                    if can_move(&field, r, Dir::Left, bottom, left) {
                        left -= 1
                    }
                }
                b'>' => {
                    if can_move(&field, r, Dir::Right, bottom, left) {
                        left += 1
                    }
                }
                _ => unreachable!(),
            }
            jet_index += 1;
        }

        loop {
            if jets[jet_index % jets.len()] == b'<' {
                if can_move(&field, r, Dir::Left, bottom, left) {
                    left -= 1
                }
            } else {
                if can_move(&field, r, Dir::Right, bottom, left) {
                    left += 1
                }
            }
            jet_index += 1;

            let dir = Dir::Down;
            if can_move(&field, r, dir, bottom, left) {
                bottom -= 1;
                continue;
            }
            let row = freeze(&mut field, r, bottom, left);
            match r {
                Rock::HLine => {
                    profile[left] = profile[left].max(bottom + 1);
                    profile[left + 1] = profile[left + 1].max(bottom + 1);
                    profile[left + 2] = profile[left + 2].max(bottom + 1);
                    profile[left + 3] = profile[left + 3].max(bottom + 1);
                }
                Rock::Crest => {
                    profile[left] = profile[left].max(bottom + 2);
                    profile[left + 1] = profile[left + 1].max(bottom + 3);
                    profile[left + 2] = profile[left + 2].max(bottom + 2);
                }
                Rock::Corner => {
                    profile[left] = profile[left].max(bottom + 1);
                    profile[left + 1] = profile[left + 1].max(bottom + 1);
                    profile[left + 2] = profile[left + 2].max(bottom + 3);
                }
                Rock::VLine => {
                    profile[left] = profile[left].max(bottom + 4);
                }
                Rock::Square => {
                    profile[left] = profile[left].max(bottom + 2);
                    profile[left + 1] = profile[left + 1].max(bottom + 2);
                }
            }

            let mut k = profile;
            let m = profile.iter().min().unwrap();
            for v in k.iter_mut() {
                *v -= m;
            }
            if row + 1 > highest_row {
                highest_row = row + 1;
            }
            if addendum == 0 {
                // Use `entry` to avoid searching for it twice.
                let e = seen.entry((k, jet_index % jets.len(), r));
                e.and_modify(|(j, h)| {
                    let step = i - *j;
                    let count = (ITER_COUNT - i) / step;
                    i += step * count;
                    addendum = (highest_row as u64 - *h) * count;
                })
                .or_insert((i, highest_row as u64));
            }
            i += 1;
            continue 'outer;
        }
    }

    highest_row as u64 + addendum
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Rock {
    HLine,
    Crest,
    Corner,
    VLine,
    Square,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Dir {
    Down,
    Left,
    Right,
}

fn freeze(field: &mut [u8], r: Rock, row: usize, col: usize) -> usize {
    match r {
        Rock::HLine => {
            field[row] |= 0xF << (7 - col - 3);
            row
        }
        Rock::Crest => {
            field[row] |= 1 << (7 - col - 1);
            field[row + 1] |= 0x7 << (7 - col - 2);
            field[row + 2] |= 1 << (7 - col - 1);
            row + 2
        }
        Rock::Corner => {
            field[row] |= 0x7 << (7 - col - 2);
            field[row + 1] |= 1 << (7 - col - 2);
            field[row + 2] |= 1 << (7 - col - 2);
            row + 2
        }
        Rock::VLine => {
            field[row] |= 1 << (7 - col);
            field[row + 1] |= 1 << (7 - col);
            field[row + 2] |= 1 << (7 - col);
            field[row + 3] |= 1 << (7 - col);
            row + 3
        }
        Rock::Square => {
            field[row] |= 3 << (7 - col - 1);
            field[row + 1] |= 3 << (7 - col - 1);
            row + 1
        }
    }
}

fn can_move(field: &[u8], r: Rock, dir: Dir, row: usize, col: usize) -> bool {
    assert!(col < 7);

    match dir {
        Dir::Down => {
            0 < row
                && match r {
                    Rock::HLine => field[row - 1] & (0xF << (7 - col - 3)) == 0,
                    Rock::Crest => {
                        field[row - 1] & (1 << (7 - col - 1)) == 0
                            && field[row] & (0x5 << (7 - col - 2)) == 0
                    }
                    Rock::Corner => field[row - 1] & (0x7 << (7 - col - 2)) == 0,
                    Rock::VLine => field[row - 1] & (1 << (7 - col)) == 0,
                    Rock::Square => field[row - 1] & (0x3 << (7 - col - 1)) == 0,
                }
        }
        Dir::Left => {
            0 < col
                && match r {
                    Rock::HLine => field[row] & (1 << (7 - col + 1)) == 0,
                    Rock::Crest => {
                        field[row] & (1 << (7 - col)) == 0
                            && field[row + 1] & (1 << (7 - col + 1)) == 0
                            && field[row + 2] & (1 << (7 - col)) == 0
                    }
                    Rock::Corner => {
                        field[row] & (1 << (7 - col + 1)) == 0
                            && field[row + 1] & (1 << (7 - col - 1)) == 0
                            && field[row + 2] & (1 << (7 - col - 1)) == 0
                    }
                    Rock::VLine => {
                        field[row] & (1 << (7 - col + 1)) == 0
                            && field[row + 1] & (1 << (7 - col + 1)) == 0
                            && field[row + 2] & (1 << (7 - col + 1)) == 0
                            && field[row + 3] & (1 << (7 - col + 1)) == 0
                    }
                    Rock::Square => {
                        field[row] & (1 << (7 - col + 1)) == 0
                            && field[row + 1] & (1 << (7 - col + 1)) == 0
                    }
                }
        }
        Dir::Right => match r {
            Rock::HLine => col + 4 < 7 && field[row] & (1 << (7 - col - 4)) == 0,
            Rock::Crest => {
                col + 3 < 7
                    && field[row] & (1 << (7 - col - 2)) == 0
                    && field[row + 1] & (1 << (7 - col - 3)) == 0
                    && field[row + 2] & (1 << (7 - col - 2)) == 0
            }
            Rock::Corner => {
                col + 3 < 7
                    && field[row] & (1 << (7 - col - 3)) == 0
                    && field[row + 1] & (1 << (7 - col - 3)) == 0
                    && field[row + 2] & (1 << (7 - col - 3)) == 0
            }
            Rock::VLine => {
                col + 1 < 7
                    && field[row] & (1 << (7 - col - 1)) == 0
                    && field[row + 1] & (1 << (7 - col - 1)) == 0
                    && field[row + 2] & (1 << (7 - col - 1)) == 0
                    && field[row + 3] & (1 << (7 - col - 1)) == 0
            }
            Rock::Square => {
                col + 2 < 7
                    && field[row] & (1 << (7 - col - 2)) == 0
                    && field[row + 1] & (1 << (7 - col - 2)) == 0
            }
        },
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
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        assert_eq!(3068, part1(&input));
        assert_eq!(1514285714288, part2(&input));
    }
}
