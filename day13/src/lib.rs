#![feature(iter_array_chunks)]

use std::cmp::Ordering;

pub fn part1(input: &str) -> u32 {
    input
        .as_bytes()
        .split(|c| *c == b'\n')
        .array_chunks::<3>()
        .enumerate()
        .filter_map(|(i, p)| compare(p[0], p[1]).is_le().then_some(i + 1))
        .sum::<usize>() as u32
}

pub fn part2(input: &str) -> u32 {
    let two = "[[2]]".as_bytes();
    let six = "[[6]]".as_bytes();

    let mut two_index = 0;
    let mut six_index = 0;
    input.as_bytes().split(|c| *c == b'\n').for_each(|b| {
        if b.len() == 0 {
            return;
        }
        two_index += (compare(two, b).is_gt()) as u32;
        six_index += (compare(six, b).is_gt()) as u32;
    });

    ((two_index + 1) * (six_index + 2)) as u32
}

fn compare(a: &[u8], b: &[u8]) -> Ordering {
    let mut ad = 0;
    let mut bd = 0;
    let mut ai = 0;
    let mut bi = 0;
    'outer: loop {
        while ai < a.len() && a[ai] == b'[' {
            ai += 1;
            ad += 1;
        }
        while bi < b.len() && b[bi] == b'[' {
            bi += 1;
            bd += 1;
        }
        if ai >= a.len() && bi < b.len() {
            return Ordering::Less;
        }
        if bi >= b.len() && ai < a.len() {
            return Ordering::Greater;
        }
        if ai >= a.len() && bi >= b.len() {
            return Ordering::Equal;
        }

        ai += (a[ai] == b',') as usize;
        bi += (b[bi] == b',') as usize;

        if a[ai] == b']' || b[bi] == b']' {
            if ad == bd && a[ai] == b']' && b[bi] == b']' {
                ad -= 1;
                bd -= 1;
                ai += 1;
                bi += 1;
                continue 'outer;
            }

            // Check if a deeper list has more elements.
            if ad < bd && a[ai] == b']' {
                return Ordering::Less;
            }
            if bd < ad && b[bi] == b']' {
                return Ordering::Greater;
            }

            // Check if a deeper list has less elements. This can't be merged with a condition above
            // because the order is important: both elements can be `]`.
            if a[ai] == b']' {
                return Ordering::Less;
            }
            if b[bi] == b']' {
                return Ordering::Greater;
            }
        }

        let (na, alen) = aoc::uint_from_bytes_prefix::<usize>(&a[ai..]);
        let (nb, blen) = aoc::uint_from_bytes_prefix::<usize>(&b[bi..]);
        let r = na.cmp(&nb);
        if r != Ordering::Equal {
            return r;
        }

        ai += alen;
        bi += blen;

        while ad < bd {
            if b[bi] != b']' {
                return Ordering::Less;
            }
            bi += 1;
            bd -= 1;
        }

        while bd < ad {
            if a[ai] != b']' {
                return Ordering::Greater;
            }
            ai += 1;
            ad -= 1;
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
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(13, part1(&input));
        assert_eq!(140, part2(&input));
    }
}
