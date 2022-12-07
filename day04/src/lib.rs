#![feature(iter_array_chunks)]

pub fn part1(input: &str) -> u32 {
    input
        .as_bytes()
        .split(|c| matches!(c, b'-' | b',' | b'\n'))
        .map(int_from_bytes::<u8>)
        .array_chunks::<4>()
        .map(|[a1, b1, a2, b2]| (a1 <= a2 && b2 <= b1 || a2 <= a1 && b1 <= b2) as u32)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .as_bytes()
        .split(|c| matches!(c, b'-' | b',' | b'\n'))
        .map(int_from_bytes::<u8>)
        .array_chunks::<4>()
        .map(|[a1, b1, a2, b2]| (a1 <= b2 && a2 <= b1) as u32)
        .sum()
}

fn int_from_bytes<T>(s: &[u8]) -> T
where
    T: std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T> + From<u8>,
{
    s.iter()
        .fold(T::from(0), |n, &c| n * T::from(10) + T::from(c - b'0'))
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
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(2, part1(&input));
        assert_eq!(4, part2(&input));
    }
}
