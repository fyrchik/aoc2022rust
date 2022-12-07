#![feature(iter_array_chunks)]

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let b = s.as_bytes();
            let (h1, h2) = b.split_at(b.len() / 2);
            let p = calculate_mask(h1);
            h2.iter()
                .find_map(|c| {
                    let prio = fake_priority(*c);
                    (p & (1 << prio) != 0).then_some(fake_to_real(prio))
                })
                .unwrap_or(0) as u32
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| calculate_mask(s.as_bytes()))
        .array_chunks::<3>()
        .map(|p| fake_to_real((p[0] & p[1] & p[2]).trailing_zeros() as u8) as u32)
        .sum()
}

/// Calculates priority mask from the items contained in b.
fn calculate_mask(b: &[u8]) -> u64 {
    b.iter().fold(0u64, |p, c| p | 1 << fake_priority(*c))
}

/// Converts item type to a "fake" priority:
/// - Lowercase item types a through z have fake priorities 32 through 57.
/// - Uppercase item types A through Z have fake priorities 1 through 26.
/// The trick here is to simplify mask calculation and postpone real priority
/// calculation until the final step. It should work because mask calculation is done
/// for every _character_, while we need real priority value only once per _line_.
fn fake_priority(a: u8) -> u8 {
    a & 0x3F
}

/// Converts "fake" priority to a real one:
/// - Lowercase item types a through z have priorities 1 through 26.
/// - Uppercase item types A through Z have priorities 27 through 52.
fn fake_to_real(p: u8) -> u8 {
    // 'a'..'z' have fake priorities in range 0b10_0001..0b11_1010
    // 'A'..'Z' have fake priorities in range 0b00_0001..0b01_1010
    (1 - ((p >> 5) & 1)) * 26 + (p & 0x1F)
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
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, part1(&input));
        assert_eq!(70, part2(&input));
    }
}
