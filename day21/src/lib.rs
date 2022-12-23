use rustc_hash::FxHashMap;

use num::{rational::Ratio, Zero};

type Name = [u8; 4];
const ROOT: Name = [b'r', b'o', b'o', b't'];
const ME: Name = [b'h', b'u', b'm', b'n'];

pub fn part1(input: &str) -> i64 {
    let mut monkeys = FxHashMap::<Name, Monkey>::default();
    monkeys.reserve(input.as_bytes().iter().filter(|c| **c == b'\n').count());

    input
        .trim_end()
        .as_bytes()
        .split(|c| *c == b'\n')
        .for_each(|b| {
            let (yell, ln) = aoc::int_from_bytes_prefix::<i64>(&b[6..]);
            let name = b[0..4].try_into().unwrap();
            if ln != 0 {
                monkeys.insert(name, Monkey::Yell(yell));
                return;
            }

            let name1 = b[6..10].try_into().unwrap();
            let name2 = b[13..17].try_into().unwrap();
            let op = match b[11] {
                b'+' => Op::Add,
                b'-' => Op::Sub,
                b'*' => Op::Mul,
                b'/' => Op::Div,
                _ => unreachable!(),
            };
            monkeys.insert(name, Monkey::Calculate(op, name1, name2));
        });

    dfs(&mut monkeys, ROOT)
}

pub fn part2(input: &str) -> i64 {
    let mut monkeys = FxHashMap::<Name, PolyMonkey>::default();
    monkeys.reserve(input.as_bytes().iter().filter(|c| **c == b'\n').count());

    input
        .trim_end()
        .as_bytes()
        .split(|c| *c == b'\n')
        .for_each(|b| {
            let (yell, ln) = aoc::int_from_bytes_prefix::<i64>(&b[6..]);
            let name = b[0..4].try_into().unwrap();
            if ln != 0 {
                monkeys.insert(name, PolyMonkey::P((Ratio::zero(), Ratio::from(yell))));
                return;
            }

            let name1 = b[6..10].try_into().unwrap();
            let name2 = b[13..17].try_into().unwrap();
            let op = match b[11] {
                b'+' => Op::Add,
                b'-' => Op::Sub,
                b'*' => Op::Mul,
                b'/' => Op::Div,
                _ => unreachable!(),
            };
            monkeys.insert(name, PolyMonkey::M(Monkey::Calculate(op, name1, name2)));
        });

    monkeys.insert(ME, PolyMonkey::P((Ratio::from(1), Ratio::zero())));

    match *monkeys.get(&ROOT).unwrap() {
        PolyMonkey::M(m) => match m {
            Monkey::Calculate(_, a, b) => {
                let mut p1 = dfs2(&mut monkeys, a);
                let mut p2 = dfs2(&mut monkeys, b);
                assert!(p1.0.is_zero() || p2.0.is_zero());
                if p1.0.is_zero() {
                    (p1, p2) = (p2, p1);
                }
                let n = (p2.1 - p1.1) / p1.0;
                return *n.numer();
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Copy, Clone)]
enum Monkey {
    Yell(i64),
    Calculate(Op, Name, Name),
}

#[derive(Copy, Clone)]
enum PolyMonkey {
    P((Ratio<i64>, Ratio<i64>)),
    M(Monkey),
}

fn dfs(monkeys: &mut FxHashMap<Name, Monkey>, name: Name) -> i64 {
    match *monkeys.get(&name).unwrap() {
        Monkey::Yell(n) => n,
        Monkey::Calculate(op, a, b) => {
            let an = dfs(monkeys, a);
            let bn = dfs(monkeys, b);
            let r = match op {
                Op::Add => an + bn,
                Op::Sub => an - bn,
                Op::Mul => an * bn,
                Op::Div => an / bn,
            };
            monkeys.insert(name, Monkey::Yell(r));
            r
        }
    }
}

fn dfs2(monkeys: &mut FxHashMap<Name, PolyMonkey>, name: Name) -> (Ratio<i64>, Ratio<i64>) {
    match *monkeys.get(&name).unwrap() {
        PolyMonkey::M(m) => match m {
            Monkey::Yell(n) => (Ratio::zero(), Ratio::from(n)),
            Monkey::Calculate(op, a, b) => {
                let an = dfs2(monkeys, a);
                let bn = dfs2(monkeys, b);
                let r = match op {
                    Op::Add => (an.0 + bn.0, an.1 + bn.1),
                    Op::Sub => (an.0 - bn.0, an.1 - bn.1),
                    Op::Mul => (an.0 * bn.1 + bn.0 * an.1, an.1 * bn.1),
                    Op::Div => (an.0 / bn.1, an.1 / bn.1),
                };
                monkeys.insert(name, PolyMonkey::P(r));
                r
            }
        },
        PolyMonkey::P(p) => p,
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
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

        assert_eq!(152, part1(&input));
        assert_eq!(301, part2(&input));
    }
}
