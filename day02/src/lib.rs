pub fn part1(input: &str) -> u32 {
    input.lines().fold(0, |sum, s| {
        let b = s.as_bytes();
        let mine = b[2] as u32 - 'X' as u32;
        let opponent = b[0] as u32 - 'A' as u32;

        sum + mine + 1 + (mine + 4 - opponent) % 3 * 3
    })
}

pub fn part2(input: &str) -> u32 {
    input.lines().fold(0, |sum, s| {
        let b = s.as_bytes();
        let result = b[2] as u32 - 'X' as u32;
        let opponent = b[0] as u32 - 'A' as u32;

        sum + result * 3 + 1 + (opponent + result + 2) % 3
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
