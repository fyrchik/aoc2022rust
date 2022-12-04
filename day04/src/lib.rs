pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let (s1, s2) = s.split_once(',').unwrap();
            let (a1s, b1s) = s1.split_once('-').unwrap();
            let (a2s, b2s) = s2.split_once('-').unwrap();
            let a1 = a1s.parse::<u32>().unwrap();
            let b1 = b1s.parse::<u32>().unwrap();
            let a2 = a2s.parse::<u32>().unwrap();
            let b2 = b2s.parse::<u32>().unwrap();

            (a1 <= a2 && b2 <= b1 || a2 <= a1 && b1 <= b2) as u32
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let (s1, s2) = s.split_once(',').unwrap();
            let (a1s, b1s) = s1.split_once('-').unwrap();
            let (a2s, b2s) = s2.split_once('-').unwrap();
            let a1 = a1s.parse::<u32>().unwrap();
            let b1 = b1s.parse::<u32>().unwrap();
            let a2 = a2s.parse::<u32>().unwrap();
            let b2 = b2s.parse::<u32>().unwrap();

            (a1 <= b2 && a2 <= b1) as u32
        })
        .sum()
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
