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
                    let prio = priority(*c - 1);
                    (p & (1 << prio) != 0).then_some(prio + 1)
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
        .map(|p| (p[0] & p[1] & p[2]).trailing_zeros() + 1)
        .sum()
}

/// Calculates priority mask from the items contained in b.
fn calculate_mask(b: &[u8]) -> u64 {
    b.iter().fold(0u64, |p, c| p | 1 << priority(c - 1))
}

/// Converts item type to a priority:
/// - Lowercase item types a through z have priorities 1 through 26.
/// - Uppercase item types A through Z have priorities 27 through 52.
fn priority(a: u8) -> u8 {
    // 'a'..'z' == 0x61..0x7A == 0b110_0001..0b111_1010
    // 'A'..'Z' == 0x41..0x5A == 0b100_0001..0b101_1010
    (1 - ((a >> 5) & 1)) * 26 + (a & 0x1F)
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
