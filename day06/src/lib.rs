#![feature(array_windows)]

pub fn part1(input: &str) -> u32 {
    let res = input
        .as_bytes()
        .array_windows::<4>()
        .enumerate()
        .find(|&(_, w)| {
            let mask = (1u32 << (w[0] - b'a'))
                | (1u32 << (w[1] - b'a'))
                | (1u32 << (w[2] - b'a'))
                | (1u32 << (w[3] - b'a'));
            mask.count_ones() == 4
        });
    res.unwrap().0 as u32 + 4
}

pub fn part2(input: &str) -> u32 {
    let res = input
        .as_bytes()
        .array_windows::<14>()
        .enumerate()
        .find(|&(_, w)| {
            w.iter()
                .fold(0, |acc, e| acc | (1u32 << (e - b'a')))
                .count_ones() as usize
                == w.len()
        });
    res.unwrap().0 as u32 + 14
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let inputs = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
        ];

        for input in inputs {
            assert_eq!(input.1, part1(&input.0));
            assert_eq!(input.2, part2(&input.0));
        }
    }
}
