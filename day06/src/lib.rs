#![feature(array_windows)]

pub fn part1(input: &str) -> u32 {
    let res = input
        .as_bytes()
        .array_windows::<4>()
        .enumerate()
        .find(|&(_, w)| {
            w[0] != w[1]
                && w[0] != w[2]
                && w[0] != w[3]
                && w[1] != w[2]
                && w[1] != w[3]
                && w[2] != w[3]
        });
    res.unwrap().0 as u32 + 4
}

pub fn part2(input: &str) -> u32 {
    let b = input.as_bytes();

    let mut counts = [0; 0x20];
    let mut total = 0u32; // Amount of different letters in a window.
    for i in 0..14.min(b.len()) {
        let index = b[i] as usize & 0x1F;
        counts[index] += 1;
        total += (counts[index] == 1) as u32;
    }

    if total == 14 {
        return 15;
    }

    for i in 14..b.len() {
        let prev = b[i - 14] as usize & 0x1F;
        let next = b[i] as usize & 0x1F;

        counts[prev] -= 1;
        total -= (counts[prev] == 0) as u32;

        counts[next] += 1;
        total += (counts[next] == 1) as u32;

        if total == 14 {
            return i as u32 + 1;
        }
    }
    return u32::MAX;
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
