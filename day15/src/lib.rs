use std::ops::Range;

pub fn part1(input: &str, row: isize) -> u32 {
    let mut beacons = vec![];
    let mut ranges: Vec<Range<isize>> = input
        .lines()
        .filter_map(|s| {
            // Example: Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            let b = s.as_bytes();
            let mut start = 12;

            let (sx, sx_len) = aoc::int_from_bytes_prefix::<isize>(&b[start..]);
            start += sx_len + 4; // ", y="

            let (sy, sy_len) = aoc::int_from_bytes_prefix::<isize>(&b[start..]);
            start += sy_len + 25; // ": closest beacon is at x="

            let (bx, bx_len) = aoc::int_from_bytes_prefix::<isize>(&b[start..]);
            start += bx_len + 4; // ", y="

            let (by, _) = aoc::int_from_bytes_prefix::<isize>(&b[start..]);
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

    if ranges.is_empty() {
        return 0;
    }

    ranges.sort_by(|a, b| a.start.cmp(&b.start).then(b.end.cmp(&a.end)));

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
    let mut sensors = vec![];
    input.lines().for_each(|s| {
        // Example: Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let b = s.as_bytes();
        let mut start = 12;

        let (sx, sx_len) = aoc::int_from_bytes_prefix::<isize>(&b[start..]);
        start += sx_len + 4; // ", y="

        let (sy, sy_len) = aoc::int_from_bytes_prefix::<isize>(&b[start..]);
        start += sy_len + 25; // ": closest beacon is at x="

        let (bx, bx_len) = aoc::int_from_bytes_prefix::<isize>(&b[start..]);
        start += bx_len + 4; // ", y="

        let (by, _) = aoc::int_from_bytes_prefix::<isize>(&b[start..]);

        let dist = (sx.abs_diff(bx) + sy.abs_diff(by)) as isize;
        if max < sy - dist || sy + dist < 0 {
            return;
        }
        sensors.push(((sx, sy), dist));
    });

    let mut edges = vec![];
    for (i, a) in sensors.iter().enumerate() {
        for b in sensors.iter().skip(i + 1) {
            // If there is a _single_ point, it must be located between two boundaries.
            let dist = (a.0 .0.abs_diff(b.0 .0) + a.0 .1.abs_diff(b.0 .1)) as isize;
            if dist == a.1 + b.1 + 2 {
                edges.push(if a.0 < b.0 { (*a, *b) } else { (*b, *a) });
            }
        }
    }
    for (i, &(s1, s2)) in edges.iter().enumerate() {
        for &(s3, s4) in edges.iter().skip(i + 1) {
            // if p0.2.0 == p1.2.0 || p0.2.0 == p1.2.1 || p0.2.1 == p1.2.0 || p0.2.1 == p1.2.1 {
            //    continue
            // }
            if intersect2d(s1.0, s2.0, s3.0, s4.0) {
                let (x1, y1) = (s1.0 .0 + s1.1 + 1, s1.0 .1);
                let (x2, y2) = (s2.0 .0 - s1.1 - 1, s2.0 .1);
                let dist = (x1.abs_diff(s3.0 .0) + y1.abs_diff(s3.0 .1)) as isize;
                let dist_diff = dist.abs_diff(s3.1 + 1) as isize / 2;
                let x = x1 + (x2 - x1).signum() * dist_diff;
                let y = y1 + (y2 - y1).signum() * dist_diff;
                return x as u64 * 4_000_000 + y as u64;
            }
        }
    }

    u64::MAX
}

fn intersect2d(a: (isize, isize), b: (isize, isize), c: (isize, isize), d: (isize, isize)) -> bool {
    if !intersect1d(a.0, b.0, c.0, d.0) || !intersect1d(a.1, b.1, c.1, d.1) {
        return false;
    }

    let v_ab = (b.0 - a.0, b.1 - a.1);
    let v_ac = (c.0 - a.0, c.1 - a.1);
    let v_ad = (d.0 - a.0, d.1 - a.1);

    let det_c = v_ab.0 * v_ac.1 - v_ab.1 * v_ac.0;
    let det_d = v_ab.0 * v_ad.1 - v_ab.1 * v_ad.0;
    det_c.signum() * det_d.signum() == -1
}

fn intersect1d(a: isize, b: isize, c: isize, d: isize) -> bool {
    let (a, b) = (a.min(b), a.max(b));
    let (c, d) = (c.min(d), c.max(d));
    a.max(c) <= b.min(d)
}

pub fn run_part1() {
    println!("{}", part1(include_str!("../input"), 2_000_000));
}

pub fn run_part2() {
    println!("{}", part2(include_str!("../input"), 4_000_000));
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
