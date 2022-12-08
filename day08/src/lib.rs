#![feature(array_chunks)]

pub fn part1(input: &str) -> u32 {
    let mut field: Vec<Vec<i8>> = input
        .lines()
        .map(|s| s.as_bytes().iter().map(|c| *c as i8).collect())
        .collect();

    let mut count = 0;
    for row in &mut field {
        let mut left_max = -1i8;
        let mut left_index = 0;
        for (j, t) in row.iter_mut().enumerate() {
            let h = *t & 0xF;
            if h > left_max {
                left_max = h;
                left_index = j;
                count += 1;
                *t |= i8::MIN;
            }
        }

        let mut right_max = -1i8;
        for j in (left_index + 1..row.len()).rev() {
            let h = row[j] & 0xF;
            if h > right_max {
                right_max = h;
                count += 1;
                row[j] |= i8::MIN;
            }
        }
    }

    for j in 0..field[0].len() {
        let mut up_max = -1i8;
        let mut up_index = 0;

        for (i, row) in field.iter().enumerate() {
            let h = row[j] & 0xF;
            if h > up_max {
                up_max = h;
                up_index = i;
                count += (row[j] > 0) as u32;
            }
        }

        let mut down_max = -1i8;
        for row in field.iter().skip(up_index + 1).rev() {
            let h = row[j] & 0xF;
            if h > down_max {
                down_max = h;
                count += (row[j] > 0) as u32;
            }
        }
    }

    count
}

pub fn part2(input: &str) -> u32 {
    let field: Vec<&[u8]> = input.lines().map(|s| s.as_bytes()).collect();

    let mut max_score = 0;
    for i in 1..field.len() - 1 {
        for j in 1..field[i].len() - 1 {
            let mid = field[i][j];
            let mut score = 1;

            let k = field[i][0..j]
                .iter()
                .rev()
                .position(|&c| c >= mid)
                .unwrap_or(0);
            score *= k + 1;

            let k = field[i][j + 1..]
                .iter()
                .position(|&c| c >= mid)
                .unwrap_or(field[i].len() - 1);
            score *= k;

            let k = (0..i).rev().position(|k| field[k][j] >= mid).unwrap_or(0);
            score *= k + 1;

            let k = (i + 1..field.len())
                .position(|k| field[k][j] >= mid)
                .unwrap_or(field.len() - 1);
            score *= k;

            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score as u32
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
        let input = "30373
25512
65332
33549
35390";

        assert_eq!(21, part1(&input));
        assert_eq!(8, part2(&input));
    }
}
