#![feature(array_chunks)]
pub fn part1(input: &str) -> u32 {
    let mut seen = rustc_hash::FxHashSet::default();
    seen.insert(0);

    let mut head = (0i32, 0i32);
    let mut tail = (0i32, 0i32);
    // We spent most of the time inserting items in a map.
    // When we change the direction (~20% on real input) often the tail doesn't move.
    // So we remember the last key we have seen and do not try to insert it in the map
    // if it hasn't changed.
    let mut last_tail = tail;
    input.as_bytes().split(|&c| c == b'\n').for_each(|b| {
        if b.len() < 3 {
            return;
        }

        let d0 = (b[0] == b'R') as i32 - (b[0] == b'L') as i32;
        let d1 = (b[0] == b'U') as i32 - (b[0] == b'D') as i32;

        for _ in 0..int_from_bytes::<u8>(&b[2..]) {
            head = (head.0 + d0, head.1 + d1);

            let diff = move_points(head.0 - tail.0, head.1 - tail.1);
            if diff != (0, 0) {
                tail = (tail.0 + diff.0, tail.1 + diff.1);

                let key = ((tail.0 as i16) << 8) | (tail.1 as i16 & 0xFF);
                seen.insert(key);
                last_tail = tail;
            }
        }
    });

    seen.len() as u32
}

pub fn move_points(d0: i32, d1: i32) -> (i32, i32) {
    let h0eq2 = d0 & 0x3 == 2;
    let h1eq2 = d1 & 0x3 == 2;

    (
        d0.signum() * (h0eq2 || d0 & 0x1 == 1 && h1eq2) as i32,
        d1.signum() * (h1eq2 || d1 & 0x1 == 1 && h0eq2) as i32,
    )
}

pub fn part2(input: &str) -> u32 {
    let mut seen = rustc_hash::FxHashSet::default();
    seen.insert(0);

    let mut points = [(0, 0); 10];
    let mut last = [(0, 0); 10];
    input.as_bytes().split(|&c| c == b'\n').for_each(|b| {
        if b.len() < 3 {
            return;
        }

        let d0 = (b[0] == b'R') as i32 - (b[0] == b'L') as i32;
        let d1 = (b[0] == b'U') as i32 - (b[0] == b'D') as i32;

        'outer: for _ in 0..int_from_bytes::<u8>(&b[2..]) {
            points[0] = (points[0].0 + d0, points[0].1 + d1);

            for i in 1..points.len() {
                let diff =
                    move_points(points[i - 1].0 - points[i].0, points[i - 1].1 - points[i].1);
                if diff == (0, 0) {
                    continue 'outer;
                }
                points[i] = (points[i].0 + diff.0, points[i].1 + diff.1);
                last[i - 1] = points[i - 1];
            }

            let tail = points[points.len() - 1];
            let key = ((tail.0 as i16) << 8) | (tail.1 as i16 & 0xFF);
            seen.insert(key);
            last[last.len() - 1] = tail;
        }
    });
    seen.len() as u32
}

fn int_from_bytes<T>(s: &[u8]) -> T
where
    T: From<u8> + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    s.iter().fold(T::from(0), |n, c| {
        let r = match c {
            b'0' => Some(T::from(0)),
            b'1' => Some(T::from(1)),
            b'2' => Some(T::from(2)),
            b'3' => Some(T::from(3)),
            b'4' => Some(T::from(4)),
            b'5' => Some(T::from(5)),
            b'6' => Some(T::from(6)),
            b'7' => Some(T::from(7)),
            b'8' => Some(T::from(8)),
            b'9' => Some(T::from(9)),
            _ => None,
        };
        n * T::from(10) + r.unwrap()
    })
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
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(13, part1(&input));
        assert_eq!(1, part2(&input));
    }

    #[test]
    fn large_example() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(36, part2(&input));
    }
}
