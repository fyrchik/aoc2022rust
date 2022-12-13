#![feature(iter_array_chunks)]

use std::cmp::Ordering;

pub fn part1(input: &str) -> u32 {
    input
        .as_bytes()
        .split(|c| *c == b'\n')
        .array_chunks::<3>()
        .enumerate()
        .filter_map(|(i, p)| {
            let (x, _) = parse_list(&p[0][1..]);
            let (y, _) = parse_list(&p[1][1..]);

            let xl = Item::List(x);
            let yl = Item::List(y);
            xl.compare(&yl).is_le().then_some(i + 1)
        })
        .sum::<usize>() as u32
}

#[derive(Debug)]
enum Item {
    Number(usize),
    List(Vec<Item>),
}

impl Item {
    fn compare(&self, other: &Item) -> Ordering {
        match (&self, other) {
            (Item::Number(x), Item::Number(y)) => x.cmp(y),
            (Item::List(x), Item::List(y)) => {
                let mut i = 0;
                while i < x.len() && i < y.len() {
                    let r = x[i].compare(&y[i]);
                    if r != Ordering::Equal {
                        return r;
                    }
                    i += 1;
                }
                x.len().cmp(&y.len())
            }
            (Item::Number(x), y) => (&Item::List(vec![Item::Number(*x)])).compare(y),
            (&x, Item::Number(y)) => x.compare(&Item::List(vec![Item::Number(*y)])),
        }
    }
}

fn parse_list(p: &[u8]) -> (Vec<Item>, usize) {
    let mut packet = vec![];
    let mut i = 0;
    while i < p.len() {
        match p[i] {
            b']' => return (packet, i + 1),
            b',' => {}
            b'[' => {
                let (it, n) = parse_list(&p[i + 1..]);
                packet.push(Item::List(it));
                i += n + 1;
                continue;
            }
            b'0'..=b'9' => {
                let (value, n) = int_from_bytes_prefix::<usize>(&p[i..]);
                i += n;
                packet.push(Item::Number(value));
                continue;
            }
            _ => unreachable!(),
        }
        i += 1;
    }

    (packet, i)
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

pub fn part2(input: &str) -> u32 {
    let mut packets: Vec<_> = input
        .as_bytes()
        .split(|c| *c == b'\n')
        .filter_map(|b| {
            if b.len() == 0 {
                None
            } else {
                let (x, _) = parse_list(&b[1..]);
                Some(Item::List(x))
            }
        })
        .collect();

    packets.sort_by(Item::compare);
    let two = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let six = Item::List(vec![Item::List(vec![Item::Number(6)])]);

    let two_index = packets.partition_point(|p| two.compare(p) == Ordering::Greater);
    let six_index = packets.partition_point(|p| six.compare(p) == Ordering::Greater);

    ((two_index + 1) * (six_index + 2)) as u32
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
