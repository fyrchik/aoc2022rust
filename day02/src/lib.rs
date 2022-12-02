#![feature(iter_array_chunks)]

pub fn part1(input: &str) -> u32 {
    input
        .as_bytes()
        .iter()
        .chain(std::iter::once(&0)) // Make it ok not to have trailing newline.
        .array_chunks::<4>()
        .fold(0, |sum, b| {
            let mine = *b[2] as u32 - 'X' as u32;
            let opponent = *b[0] as u32 - 'A' as u32;
            let result = (mine + 4 - opponent) % 3;

            sum + mine + 1 + result * 3
        })
}

pub fn part2(input: &str) -> u32 {
    input
        .as_bytes()
        .iter()
        .chain(std::iter::once(&0)) // Make it ok not to have trailing newline.
        .array_chunks::<4>()
        .fold(0, |sum, b| {
            let result = *b[2] as u32 - 'X' as u32;
            let opponent = *b[0] as u32 - 'A' as u32;
            let mine = (opponent + result + 2) % 3;

            sum + mine + 1 + result * 3
        })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "A Y
B X
C Z";

        assert_eq!(15, part1(&input));
        assert_eq!(12, part2(&input));
    }
}
