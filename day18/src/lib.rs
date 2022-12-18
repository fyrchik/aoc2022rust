use std::collections::VecDeque;

pub fn part1(input: &str) -> u32 {
    let cubes: rustc_hash::FxHashSet<(isize, isize, isize)> = input
        .trim_end()
        .as_bytes()
        .split(|c| *c == b'\n')
        .map(|b| {
            let mut start = 0;
            let (x, x_len) = int_from_bytes_prefix(&b[start..]);
            start += x_len + 1;

            let (y, y_len) = int_from_bytes_prefix(&b[start..]);
            start += y_len + 1;

            let (z, _) = int_from_bytes_prefix(&b[start..]);
            (x, y, z)
        })
        .collect();

    let mut sides = cubes.len() * 6;
    for c in cubes.iter() {
        sides -= cubes.contains(&(c.0 + 1, c.1, c.2)) as usize;
        sides -= cubes.contains(&(c.0 - 1, c.1, c.2)) as usize;
        sides -= cubes.contains(&(c.0, c.1 + 1, c.2)) as usize;
        sides -= cubes.contains(&(c.0, c.1 - 1, c.2)) as usize;
        sides -= cubes.contains(&(c.0, c.1, c.2 + 1)) as usize;
        sides -= cubes.contains(&(c.0, c.1, c.2 - 1)) as usize;
    }
    sides as u32
}

pub fn part2(input: &str) -> u32 {
    let mut min = (isize::MAX, isize::MAX, isize::MAX);
    let mut max = (isize::MIN, isize::MIN, isize::MIN);
    let cubes: rustc_hash::FxHashSet<(isize, isize, isize)> = input
        .trim_end()
        .as_bytes()
        .split(|c| *c == b'\n')
        .map(|b| {
            let mut start = 0;
            let (x, x_len) = int_from_bytes_prefix(&b[start..]);
            start += x_len + 1;

            let (y, y_len) = int_from_bytes_prefix(&b[start..]);
            start += y_len + 1;

            let (z, _) = int_from_bytes_prefix(&b[start..]);

            min.0 = min.0.min(x);
            min.1 = min.1.min(y);
            min.2 = min.2.min(z);
            max.0 = max.0.max(x);
            max.1 = max.1.max(y);
            max.2 = max.2.max(z);
            (x, y, z)
        })
        .collect();

    let mut queue = VecDeque::new();
    queue.push_back((max.0 + 1, max.1 + 1, max.2 + 1));

    let mut seen = rustc_hash::FxHashSet::default();
    let mut count = 0;
    while let Some(cell) = queue.pop_front() {
        if seen.contains(&cell) {
            continue;
        }

        let cc = (cell.0 + 1, cell.1, cell.2);
        if cell.0 <= max.0 && !seen.contains(&cc) {
            if cubes.contains(&cc) {
                count += 1
            } else {
                queue.push_back(cc);
            }
        }

        let cc = (cell.0 - 1, cell.1, cell.2);
        if min.0 <= cell.0 && !seen.contains(&cc) {
            if cubes.contains(&cc) {
                count += 1
            } else {
                queue.push_back(cc);
            }
        }

        let cc = (cell.0, cell.1 + 1, cell.2);
        if cell.1 <= max.1 && !seen.contains(&cc) {
            if cubes.contains(&cc) {
                count += 1
            } else {
                queue.push_back(cc);
            }
        }

        let cc = (cell.0, cell.1 - 1, cell.2);
        if min.1 <= cell.1 && !seen.contains(&cc) {
            if cubes.contains(&cc) {
                count += 1
            } else {
                queue.push_back(cc);
            }
        }

        let cc = (cell.0, cell.1, cell.2 + 1);
        if cell.2 <= max.2 && !seen.contains(&cc) {
            if cubes.contains(&cc) {
                count += 1
            } else {
                queue.push_back(cc);
            }
        }

        let cc = (cell.0, cell.1, cell.2 - 1);
        if min.2 <= cell.2 && !seen.contains(&cc) {
            if cubes.contains(&cc) {
                count += 1
            } else {
                queue.push_back(cc);
            }
        }
        seen.insert(cell);
    }

    count as u32
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
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

        assert_eq!(64, part1(&input));
        assert_eq!(58, part2(&input));
    }
}
