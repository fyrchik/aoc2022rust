#![feature(iter_array_chunks)]
#![feature(array_windows)]

pub fn part1(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let mut stack = Vec::with_capacity(9);
    crates.lines().rev().enumerate().for_each(|(i, s)| {
        if i == 0 {
            for _ in 0..=s.len() / 4 {
                stack.push(vec![]);
            }
            return;
        }
        s.as_bytes()
            .iter()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, &c)| c != b' ')
            .for_each(|(i, &c)| stack[i].push(c));
    });

    instructions
        .as_bytes()
        .split(|c| matches!(c, b' ' | b'\n'))
        .skip(1)
        .step_by(2)
        .map(int_from_bytes::<usize>)
        .array_chunks::<3>()
        .for_each(|[n, from, to]| {
            let (f, t) = get_mut_elements(&mut stack, from - 1, to - 1);
            let sz = f.len() - n;
            f[sz..].reverse();
            t.extend_from_slice(&f[sz..]);
            f.truncate(sz);
        });

    let s: Vec<u8> = stack.iter().map(|s| *s.last().unwrap()).collect();
    String::from_utf8(s).unwrap()
}

pub fn part2(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let mut stack = Vec::with_capacity(9);
    crates.lines().rev().enumerate().for_each(|(i, s)| {
        if i == 0 {
            for _ in 0..=s.len() / 4 {
                stack.push(vec![]);
            }
            return;
        }
        s.as_bytes()
            .iter()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, &c)| c != b' ')
            .for_each(|(i, &c)| stack[i].push(c));
    });

    instructions
        .as_bytes()
        .split(|c| matches!(c, b' ' | b'\n'))
        .skip(1)
        .step_by(2)
        .map(int_from_bytes::<usize>)
        .array_chunks::<3>()
        .for_each(|[n, from, to]| {
            let (f, t) = get_mut_elements(&mut stack, from - 1, to - 1);
            let sz = f.len() - n;
            t.extend_from_slice(&f[sz..]);
            f.truncate(sz);
        });

    let s: Vec<u8> = stack.iter().map(|s| *s.last().unwrap()).collect();
    String::from_utf8(s).unwrap()
}

fn get_mut_elements<T>(v: &mut Vec<T>, a: usize, b: usize) -> (&mut T, &mut T) {
    assert!(a < v.len());
    assert!(b < v.len());
    assert!(a != b);

    if a < b {
        let (x, y) = v.split_at_mut(b);
        (&mut x[a], &mut y[0])
    } else {
        let (x, y) = v.split_at_mut(a);
        (&mut y[0], &mut x[b])
    }
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
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!("CMZ", part1(&input).as_str());
        assert_eq!("MCD", part2(&input));
    }
}
