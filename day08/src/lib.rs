pub fn part1(input: &str) -> u32 {
    let mut field: Vec<Vec<(i8, bool)>> = input
        .lines()
        .map(|s| s.as_bytes().iter().map(|c| (*c as i8, false)).collect())
        .collect();

    let mut count = 0;
    for row in &mut field {
        let mut left_max = -1i8;
        let mut left_index = 0;
        for (j, t) in row.iter_mut().enumerate() {
            if t.0 > left_max {
                left_max = t.0;
                left_index = j;
                count += 1;
                t.1 = true;
            }
        }

        let mut right_max = -1i8;
        for j in (left_index + 1..row.len()).rev() {
            if row[j].0 > right_max {
                right_max = row[j].0;
                count += 1;
                row[j].1 = true;
            }
        }
    }

    for j in 0..field[0].len() {
        let mut up_max = -1i8;
        let mut up_index = 0;

        for (i, row) in field.iter_mut().enumerate() {
            if row[j].0 > up_max {
                up_max = row[j].0;
                up_index = i;
                if !row[j].1 {
                    count += 1;
                    row[j].1 = true;
                }
            }
        }

        let mut down_max = -1i8;
        for i in (up_index + 1..field.len()).rev() {
            if field[i][j].0 > down_max {
                down_max = field[i][j].0;
                if !field[i][j].1 {
                    count += 1;
                    field[i][j].1 = true;
                }
            }
        }
    }

    count
}

pub fn part2(input: &str) -> u32 {
    let field: Vec<Vec<u8>> = input
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect();

    let mut max_score = 0;
    for i in 1..field.len() - 1 {
        for j in 1..field[i].len() - 1 {
            let mut score = 1;
            for k in (0..j).rev() {
                if field[i][k] >= field[i][j] || k == 0 {
                    score *= j - k;
                    break;
                }
            }
            for k in j + 1..field[i].len() {
                if field[i][k] >= field[i][j] || k == field[i].len() - 1 {
                    score *= k - j;
                    break;
                }
            }
            for k in (0..i).rev() {
                if field[k][j] >= field[i][j] || k == 0 {
                    score *= i - k;
                    break;
                }
            }
            for k in i + 1..field.len() {
                if field[k][j] >= field[i][j] || k == field.len() - 1 {
                    score *= k - i;
                    break;
                }
            }
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
