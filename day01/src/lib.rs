pub fn part1(input: &str) -> u32 {
    let r = input
        .lines()
        .map(|s| s.parse::<u32>().ok())
        .fold((0, 0), |(max, curr), s| match s {
            None => (curr.max(max), 0),
            Some(x) => (max, curr + x),
        });
    r.0.max(r.1)
}

pub fn part2(input: &str) -> u32 {
    let r = input
        .lines()
        .map(|s| s.parse::<u32>().ok())
        .fold(((0, 0, 0), 0), |(top3, curr), s| match s {
            None => (insert_max(top3, curr), 0),
            Some(x) => (top3, curr + x),
        });
    let x = insert_max(r.0, r.1);
    x.0 + x.1 + x.2
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
