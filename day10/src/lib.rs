pub fn part1(input: &str) -> i32 {
    let mut cycle = 0;
    let mut register = 1;
    let mut strength = 0;

    input.as_bytes().split(|&c| c == b'\n').for_each(|b| {
        if b.len() < 4 {
            return;
        }

        let rem = cycle % 40;

        strength += (rem == 19) as i32 * register * (cycle + 1);
        cycle += 1;

        if b[0] == b'a' {
            strength += (rem == 18) as i32 * register * (cycle + 1);
            register += int_from_bytes::<i32>(&b[5..]);
            cycle += 1
        }
    });

    strength
}

pub fn part2(input: &str) -> String {
    let mut out = vec![b'.'; 240 + 5];
    for i in (40..245).step_by(41) {
        out[i] = b'\n'
    }

    let mut cycle = 0isize;
    let mut lo = 0isize;
    let mut hi = 2isize;
    input.as_bytes().split(|&c| c == b'\n').for_each(|b| {
        if b.len() < 4 {
            return;
        }

        let row = cycle / 40;
        let col = cycle % 40;
        if lo <= col && col <= hi {
            out[(row * 41 + col) as usize] = b'#'
        }

        cycle += 1;
        if b[0] == b'a' {
            let row = cycle / 40;
            let col = cycle % 40;
            if lo <= col && col <= hi {
                out[(row * 41 + col) as usize] = b'#'
            }

            cycle += 1;

            let n = int_from_bytes::<isize>(&b[5..]);
            lo += n;
            hi += n;
        }
    });

    String::from_utf8(out).unwrap()
}

fn int_from_bytes<T>(s: &[u8]) -> T
where
    T: From<i8> + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    T::from((s[0] != b'-') as i8 * 2 - 1)
        * s.iter().fold(T::from(0), |n, c| {
            let r = match c {
                b'0' => T::from(0),
                b'1' => T::from(1),
                b'2' => T::from(2),
                b'3' => T::from(3),
                b'4' => T::from(4),
                b'5' => T::from(5),
                b'6' => T::from(6),
                b'7' => T::from(7),
                b'8' => T::from(8),
                b'9' => T::from(9),
                _ => T::from(0),
            };
            n * T::from(10) + r
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
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        assert_eq!(13140, part1(&input));
        assert_eq!(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....",
            part2(&input)
        );
    }
}
