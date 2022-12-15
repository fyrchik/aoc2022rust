use std::ops::Range;

pub fn part1(input: &str, row: isize) -> u32 {
    let mut beacons = vec![];
    let mut ranges: Vec<Range<isize>> = input
        .lines()
        .filter_map(|s| {
            // Example: Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            let b = s.as_bytes();
            let mut start = 12;

            let (sx, sx_len) = int_from_bytes_prefix::<isize>(&b[start..]);
            start += sx_len + 4; // ", y="

            let (sy, sy_len) = int_from_bytes_prefix::<isize>(&b[start..]);
            start += sy_len + 25; // ": closest beacon is at x="

            let (bx, bx_len) = int_from_bytes_prefix::<isize>(&b[start..]);
            start += bx_len + 4; // ", y="

            let (by, _) = int_from_bytes_prefix::<isize>(&b[start..]);
            if by == row {
                let index = beacons.partition_point(|&x| x < bx);
                if index == beacons.len() || beacons[index] != bx {
                    beacons.insert(index, bx);
                }
            }

            let dist = (sx.abs_diff(bx) + sy.abs_diff(by)) as isize;
            if row < sy - dist || sy + dist < row {
                return None;
            }

            let d = dist - sy.abs_diff(row) as isize;
            Some(sx - d..sx + d + 1)
        })
        .collect();
    ranges.sort_by(|a, b| a.start.cmp(&b.start).then(b.end.cmp(&a.end)));
    if ranges.len() == 0 {
        return 0;
    }

    let mut count = 0;
    let mut last = ranges[0].start;
    let mut i = 0;
    for r in ranges {
        let start = r.start.max(last);
        if start < r.end {
            count += r.end - start;
            while i < beacons.len() && beacons[i] < start {
                i += 1;
            }
            while i < beacons.len() && beacons[i] < r.end {
                i += 1;
                count -= 1;
            }
            last = r.end;
        }
    }
    count as u32
}

pub fn part2(input: &str, max: isize) -> u64 {
    let mut beacons = vec![];
    let mut sensors = vec![];
    input.lines().for_each(|s| {
        // Example: Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let b = s.as_bytes();
        let mut start = 12;

        let (sx, sx_len) = int_from_bytes_prefix::<isize>(&b[start..]);
        start += sx_len + 4; // ", y="

        let (sy, sy_len) = int_from_bytes_prefix::<isize>(&b[start..]);
        start += sy_len + 25; // ": closest beacon is at x="

        let (bx, bx_len) = int_from_bytes_prefix::<isize>(&b[start..]);
        start += bx_len + 4; // ", y="

        let (by, _) = int_from_bytes_prefix::<isize>(&b[start..]);
        let index = beacons.partition_point(|&x| x < bx);
        if index == beacons.len() || beacons[index] != bx {
            beacons.insert(index, bx);
        }

        let dist = (sx.abs_diff(bx) + sy.abs_diff(by)) as isize;
        if max < sy - dist || sy + dist < 0 {
            return;
        }
        sensors.push((sx, sy, dist));
    });

    for y in 0..=max {
        let mut x = 0;
        'xloop: while x <= max {
            for s in &sensors {
                let d = s.2 - s.1.abs_diff(y) as isize;
                if s.0 - d <= x && x <= s.0 + d {
                    x = s.0 + d + 1;
                    continue 'xloop;
                }
            }
            return x as u64 * 4_000_000 + y as u64;
        }
    }

    u64::MAX
}

pub fn run_part1() {
    println!("{}", part1(include_str!("../input"), 2_000_000));
}

pub fn run_part2() {
    println!("{}", part2(include_str!("../input"), 4_000_000));
}

fn int_from_bytes_prefix<T>(s: &[u8]) -> (T, usize)
where
    T: From<i8> + std::ops::MulAssign + std::ops::AddAssign,
{
    let mut n = T::from(0);
    let mut signum = 1;
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
            b'-' => {
                debug_assert_eq!(i, 0);
                signum = -1;
                continue;
            }
            _ => {
                n *= T::from(signum);
                return (n, i);
            }
        };
        n *= T::from(10);
        n += T::from(r);
    }

    n *= T::from(signum);
    (n, s.len())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(26, part1(&input, 10));
        assert_eq!(56_000_011, part2(&input, 20));
    }
}
