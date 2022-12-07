#![feature(iter_array_chunks)]

pub fn part1(input: &str) -> u32 {
    input
        .as_bytes()
        .iter()
        .chain(std::iter::once(&0)) // Make it ok not to have trailing newline.
        .array_chunks::<4>()
        .map(|b| {
            let mine = *b[2] - b'X';
            let opponent = *b[0] - b'A';
            let result = (mine + 4 - opponent) % 3;

            (mine + 1 + result * 3) as u32
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .as_bytes()
        .iter()
        .chain(std::iter::once(&0)) // Make it ok not to have trailing newline.
        .array_chunks::<4>()
        .map(|b| {
            let result = *b[2] - b'X';
            let opponent = *b[0] - b'A';
            let mine = (opponent + result + 2) % 3;

            (mine + 1 + result * 3) as u32
        })
        .sum()
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
        let input = "A Y
B X
C Z";

        assert_eq!(15, part1(&input));
        assert_eq!(12, part2(&input));
    }
}
