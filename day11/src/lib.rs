#![feature(iter_array_chunks)]

use rayon::prelude::*;

#[derive(Debug)]
enum Operation {
    Mul(u8),
    Add(u8),
    Square,
}

struct Item {
    worry_level: u32,
    monkey_index: u8,
}

struct Monkey {
    op: Operation,
    test: u8,
    targets: [u8; 2],
}

impl Monkey {
    fn inspect(&self, old: u32) -> u32 {
        match self.op {
            Operation::Mul(x) => old * x as u32,
            Operation::Add(x) => old + x as u32,
            Operation::Square => old * old,
        }
    }

    fn inspect_big(&self, old: u32, modulo: u32) -> u32 {
        (match self.op {
            Operation::Mul(x) => (old as u64 * x as u64) % modulo as u64,
            Operation::Add(x) => old as u64 + x as u64,
            Operation::Square => (old as u64 * old as u64) % modulo as u64,
        }) as u32
    }
}

pub fn part1(input: &str) -> u32 {
    let (monkeys, mut items) = parse(input);
    let mut inspected = vec![0u8; monkeys.len()];

    for _ in 0..20 {
        for (i, m) in monkeys.iter().enumerate() {
            items
                .iter_mut()
                .filter(|it| it.monkey_index as usize == i)
                .for_each(|it| {
                    let new_worry = m.inspect(it.worry_level) / 3;
                    inspected[i] += 1;

                    let test = (new_worry % m.test as u32 == 0) as usize;
                    it.worry_level = new_worry;
                    it.monkey_index = m.targets[test];
                });
        }
    }

    inspected.sort_by(|x, y| y.cmp(x));
    inspected[0] as u32 * inspected[1] as u32
}

pub fn part2(input: &str) -> u64 {
    let (monkeys, items) = parse(input);
    let modulo: u32 = monkeys.iter().map(|m| m.test as u32).product();

    let mut inspected: Vec<u32> = items
        .par_iter()
        .map(|k| {
            let mut inspected = vec![0u32; monkeys.len()];
            let mut worry = k.worry_level;
            let mut index = k.monkey_index as usize;
            for _ in 0..10_000 {
                loop {
                    worry = monkeys[index].inspect_big(worry, modulo);
                    inspected[index] += 1;

                    let test = (worry % monkeys[index].test as u32 == 0) as usize;
                    let new_index = monkeys[index].targets[test] as usize;
                    if new_index <= index {
                        index = new_index;
                        break;
                    }
                    index = new_index;
                }
            }
            inspected
        })
        .reduce(
            || vec![0u32; monkeys.len()],
            |x, y| x.iter().zip(y.iter()).map(|(a, b)| a + b).collect(),
        );

    inspected.sort_by(|x, y| y.cmp(x));
    inspected[0] as u64 * inspected[1] as u64
}

fn parse(input: &str) -> (Vec<Monkey>, Vec<Item>) {
    let mut items: Vec<Item> = vec![];
    let monkeys = input
        .as_bytes()
        .split(|&c| c == b'\n')
        .filter(|b| !b.is_empty())
        .array_chunks::<6>()
        .enumerate()
        .map(|(i, ls)| {
            items.extend(ls[1][18..].split(|&c| c == b',').map(move |b| Item {
                worry_level: int_from_bytes::<u32>(b),
                monkey_index: i as u8,
            }));
            let raw_op = ls[2][23..]
                .split(|&c| c == b' ')
                .array_chunks::<2>()
                .next()
                .unwrap();
            let op = match raw_op[0][0] {
                b'+' => Operation::Add(int_from_bytes::<u8>(raw_op[1])),
                b'*' => {
                    if raw_op[1][0] == b'o' {
                        Operation::Square
                    } else {
                        Operation::Mul(int_from_bytes::<u8>(raw_op[1]))
                    }
                }

                _ => unreachable!("unexpected operation: {}", raw_op[0][0] as char),
            };
            let test = int_from_bytes::<u8>(&ls[3][21..]);
            let targets = [
                int_from_bytes::<u8>(&ls[5][30..]),
                int_from_bytes::<u8>(&ls[4][29..]),
            ];
            Monkey { op, test, targets }
        })
        .collect();
    (monkeys, items)
}

fn int_from_bytes<T>(s: &[u8]) -> T
where
    T: From<u8> + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    s.iter().fold(T::from(0), |n, c| {
        let r = match c {
            b'0' => T::from(0),
            b'1' => T::from(1),
            b'2' => T::from(2),
            b'3' => T::from(3),
            b'4' => T::from(4),
            b'5' => T::from(5),
            b'6' => T::from(6),
            b'7' => T::from(7),
            b'8' => T::from(8),
            b'9' => T::from(9),
            _ => T::from(0),
        };
        n * T::from(10) + r
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
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        assert_eq!(10605, part1(&input));
        assert_eq!(2713310158, part2(&input));
    }
}
