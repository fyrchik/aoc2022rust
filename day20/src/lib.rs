pub fn part1(input: &str) -> i64 {
    let numbers = parse(input, 1);
    let mut indexes: Vec<usize> = (0..numbers.len()).collect();
    for (i, &n) in numbers.iter().enumerate() {
        let old_index = indexes.iter().position(|&k| k == i).unwrap();
        let new_index = (old_index as i64 + n).rem_euclid(indexes.len() as i64 - 1) as usize;
        let n = indexes[old_index];
        if old_index < new_index {
            indexes.copy_within(old_index + 1..=new_index, old_index);
        } else {
            indexes.copy_within(new_index..old_index, new_index + 1);
        }
        indexes[new_index] = n;
    }

    let zero_curr = numbers.iter().position(|&k| k == 0).unwrap();
    let zero_orig = indexes.iter().position(|&k| k == zero_curr).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| numbers[indexes[(zero_orig + i) % indexes.len()]])
        .sum::<i64>()
}

pub fn part2(input: &str) -> i64 {
    let numbers = parse(input, 811589153);
    let mut indexes: Vec<usize> = (0..numbers.len()).collect();
    for _ in 0..10 {
        for (i, &n) in numbers.iter().enumerate() {
            let old_index = indexes.iter().position(|&k| k == i).unwrap();
            let new_index = (old_index as i64 + n).rem_euclid(indexes.len() as i64 - 1) as usize;
            let n = indexes[old_index];
            if old_index < new_index {
                indexes.copy_within(old_index + 1..=new_index, old_index);
            } else {
                indexes.copy_within(new_index..old_index, new_index + 1);
            }
            indexes[new_index] = n;
        }
    }

    let zero_curr = numbers.iter().position(|&k| k == 0).unwrap();
    let zero_orig = indexes.iter().position(|&k| k == zero_curr).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| numbers[indexes[(zero_orig + i) % indexes.len()]])
        .sum::<i64>()
}

fn parse(input: &str, key: i64) -> Vec<i64> {
    input
        .trim_end()
        .as_bytes()
        .split(|c| *c == b'\n')
        .map(|b| aoc::int_from_bytes::<i64>(b) * key)
        .collect()
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
        let input = "1
2
-3
3
-2
0
4";

        assert_eq!(3, part1(&input));
        assert_eq!(1623178306, part2(&input));
    }
}
