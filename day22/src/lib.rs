const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;

fn move_from(x: usize, y: usize, dir: usize, size: usize) -> Option<(usize, usize, usize)> {
    if size == 4 {
        if y <= 3 {
            if x == 8 && dir == LEFT {
                return Some((4 + y, 4, DOWN));
            } else if x == RIGHT {
                return Some((15, 11 - y, LEFT));
            }
        } else if x <= 3 {
            if y == 4 && dir == UP {
                return Some((11 - x, 0, DOWN));
            } else if y == 7 && dir == DOWN {
                return Some((11 - x, 11, UP));
            } else if x == 0 && dir == LEFT {
                return Some((15 - (x - 4), 11, UP));
            }
        } else if 3 < x && x <= 7 {
            if y == 4 && dir == UP {
                return Some((8, x - 4, RIGHT));
            } else if y == 7 && dir == DOWN {
                return Some((8, 11 - (x - 4), RIGHT));
            }
        } else if x == 11 && 4 <= y && y <= 7 && dir == RIGHT {
            return Some((15 - (y - 4), 8, DOWN));
        } else if 8 <= y && x == 8 && dir == LEFT {
            return Some((7 - (y - 8), 7, UP));
        } else if y == 11 && 8 <= x && x <= 11 && dir == DOWN {
            return Some((3 - (x - 8), 7, UP));
        } else if y == 11 && 12 <= x && dir == DOWN {
            return Some((0, 7 - (y - 12), RIGHT));
        }
        return None;
    }

    // My custom input.
    return if x == size && 0 <= y && y < size && dir == LEFT {
        Some((0, 3 * size - 1 - y, RIGHT))
    } else if x == size && size <= y && y < 2 * size && dir == LEFT {
        Some((y - size, 2 * size, DOWN))
    } else if y == size * 2 && 0 <= x && x < size && dir == UP {
        Some((size, size + x, RIGHT))
    } else if x == 0 && 2 * size <= y && y < 3 * size && dir == LEFT {
        Some((size, size - 1 - (y - 2 * size), RIGHT))
    } else if x == 0 && 3 * size <= y && y < 4 * size && dir == LEFT {
        Some((y - 3 * size + size, 0, DOWN))
    } else if y == 4 * size - 1 && x < size && dir == DOWN {
        Some((2 * size + x, 0, DOWN))
    } else if x == size - 1 && 3 * size <= y && y < 4 * size && dir == RIGHT {
        Some((size + y - size * 3, 3 * size - 1, UP))
    } else if y == 3 * size - 1 && size <= x && x < 2 * size && dir == DOWN {
        Some((size - 1, 3 * size + x - size, LEFT))
    } else if x == 2 * size - 1 && 2 * size <= y && y < 3 * size && dir == RIGHT {
        Some((3 * size - 1, size - 1 - (y - 2 * size), LEFT))
    } else if x == 2 * size - 1 && size <= y && y < 2 * size && dir == RIGHT {
        Some((2 * size + y - size, size - 1, UP))
    } else if y == size - 1 && 2 * size <= x && dir == DOWN {
        Some((2 * size - 1, size + x - 2 * size, LEFT))
    } else if x == 3 * size - 1 && y < size && dir == RIGHT {
        Some((2 * size - 1, 3 * size - 1 - y, LEFT))
    } else if y == 0 && 2 * size <= x && dir == UP {
        Some((x - 2 * size, 4 * size - 1, UP))
    } else if y == 0 && size <= x && x < 2 * size && dir == UP {
        Some((0, 3 * size + x - size, RIGHT))
    } else {
        None
    };
}

pub fn part1(input: &str) -> u32 {
    let mut instructions = None;
    let mut parsed_field = false;
    let mut field = vec![];
    input
        .trim_end()
        .as_bytes()
        .split(|c| *c == b'\n')
        .for_each(|b| {
            if b.len() == 0 {
                parsed_field = true;
                return;
            } else if parsed_field {
                instructions = Some(b);
                return;
            }

            let start = b.iter().position(|c| *c != b' ').unwrap();
            field.push((start, b));
        });

    let mut start = 0;
    let inst = instructions.unwrap();
    let mut x = field[0].0;
    let mut y = 0;
    let mut dir = 0;
    while start < inst.len() {
        let (mut n, n_len) = aoc::uint_from_bytes_prefix(&inst[start..]);
        debug_assert!(n_len > 0);

        start += n_len;
        match dir {
            0 => {
                while n > 0 {
                    if x + 1 == field[y].1.len() {
                        if field[y].1[field[y].0] == b'.' {
                            n -= 1;
                            x = field[y].0;
                        } else {
                            n = 0;
                        }
                    } else if field[y].1[x + 1] == b'.' {
                        n -= 1;
                        x += 1;
                    } else {
                        n = 0;
                    }
                }
            }
            2 => {
                while n > 0 {
                    if x == field[y].0 {
                        if field[y].1[field[y].1.len() - 1] == b'.' {
                            n -= 1;
                            x = field[y].1.len() - 1;
                        } else {
                            n = 0;
                        }
                    } else if field[y].1[x - 1] == b'.' {
                        n -= 1;
                        x -= 1;
                    } else {
                        n = 0;
                    }
                }
            }
            1 => {
                while n > 0 {
                    if y == field.len() - 1
                        || field[y + 1].1.len() <= x
                        || field[y + 1].1[x] == b' '
                    {
                        let wrapped = field
                            .iter()
                            .position(|(_, field)| x < field.len() && field[x] != b' ')
                            .unwrap();
                        if field[wrapped].1[x] == b'.' {
                            n -= 1;
                            y = wrapped;
                        } else {
                            n = 0;
                        }
                    } else if field[y + 1].1[x] == b'.' {
                        n -= 1;
                        y += 1;
                    } else {
                        n = 0;
                    }
                }
            }
            3 => {
                while n > 0 {
                    if y == 0 || field[y - 1].1.len() <= x || field[y - 1].1[x] == b' ' {
                        let wrapped = y + field
                            .iter()
                            .skip(y)
                            .position(|(_, row)| row.len() <= x || row[x] == b' ')
                            .unwrap_or(field.len() - y)
                            - 1;
                        if field[wrapped].1[x] == b'.' {
                            n -= 1;
                            y = wrapped;
                        } else {
                            n = 0;
                        }
                    } else if field[y - 1].1[x] == b'.' {
                        n -= 1;
                        y -= 1;
                    } else {
                        n = 0;
                    }
                }
            }
            _ => unreachable!(),
        }

        if start < inst.len() {
            match inst[start] {
                b'R' => dir = (dir + 1) % 4,
                b'L' => dir = (dir + 3) % 4,
                _ => unreachable!(),
            }
            start += 1;
        }
    }

    (1000 * (y + 1) + 4 * (x + 1) + dir) as u32
}

pub fn part2(input: &str, size: usize) -> u32 {
    let mut instructions = None;
    let mut parsed_field = false;
    let mut field = vec![];
    input
        .trim_end()
        .as_bytes()
        .split(|c| *c == b'\n')
        .for_each(|b| {
            if b.len() == 0 {
                parsed_field = true;
                return;
            } else if parsed_field {
                instructions = Some(b);
                return;
            }

            let start = b.iter().position(|c| *c != b' ').unwrap();
            field.push((start, b));
        });

    let mut start = 0;
    let inst = instructions.unwrap();
    let mut x = field[0].0;
    let mut y = 0;
    let mut dir = 0;

    while start < inst.len() {
        let (mut n, n_len) = aoc::uint_from_bytes_prefix::<usize>(&inst[start..]);
        debug_assert!(n_len > 0);

        start += n_len;
        while n > 0 {
            let (new_x, new_y, new_dir) = if let Some(m) = move_from(x, y, dir, size) {
                m
            } else {
                match dir {
                    LEFT => (x - 1, y, dir),
                    RIGHT => (x + 1, y, dir),
                    DOWN => (x, y + 1, dir),
                    UP => (x, y - 1, dir),
                    _ => unreachable!(),
                }
            };
            if field[new_y].1[new_x] == b'.' {
                x = new_x;
                y = new_y;
                dir = new_dir;
                n -= 1;
            } else {
                break;
            }
        }

        if start < inst.len() {
            match inst[start] {
                b'R' => dir = (dir + 1) % 4,
                b'L' => dir = (dir + 3) % 4,
                _ => unreachable!(),
            }
            start += 1;
        }
    }

    (1000 * (y + 1) + 4 * (x + 1) + dir) as u32
}

pub fn run_part1() {
    println!("{}", part1(include_str!("../input")));
}

pub fn run_part2() {
    println!("{}", part2(include_str!("../input"), 50));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

        assert_eq!(6032, part1(&input));
        assert_eq!(5031, part2(&input, 4));
    }
}
