pub fn part1(input: &str) -> u32 {
    input
        .as_bytes()
        .iter()
        .chain(&[b'\n'; 2]) // Avoid having custom logic for the last group.
        .fold((0, 0, 0), |(max, sum, curr), b| {
            if *b == b'\n' {
                if curr == 0 {
                    (max.max(sum), 0, 0)
                } else {
                    (max, sum + curr, 0)
                }
            } else {
                (max, sum, curr * 10 + (*b - b'0') as u32)
            }
        })
        .0
}

pub fn part2(input: &str) -> u32 {
    let s = input
        .as_bytes()
        .iter()
        .chain(&[b'\n'; 2]) // Avoid having custom logic for the last group.
        .fold(((0, 0, 0), 0, 0), |(top3, sum, curr), b| {
            if *b == b'\n' {
                if curr == 0 {
                    (insert_max(top3, sum), 0, 0)
                } else {
                    (top3, sum + curr, 0)
                }
            } else {
                (top3, sum, curr * 10 + (*b - b'0') as u32)
            }
        })
        .0;
    s.0 + s.1 + s.2
}

fn insert_max(top3: (u32, u32, u32), e: u32) -> (u32, u32, u32) {
    if e < top3.2 {
        top3
    } else if e < top3.1 {
        (top3.0, top3.1, e)
    } else if e < top3.0 {
        (top3.0, e, top3.1)
    } else {
        (e, top3.0, top3.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(24000, part1(&input));
        assert_eq!(45000, part2(&input));
    }
}
