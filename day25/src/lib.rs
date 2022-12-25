pub fn part1(input: &str) -> String {
    let n = input.trim_end().as_bytes().split(|c| *c == b'\n').map(snafu_decode).sum();
    snafu_encode(n)
}

pub fn part2(input: &str) -> u32 {
    0
}

fn snafu_decode(b: &[u8]) -> u64 {
   let mut n = 0u64;
   for c in b {
      n = match c {
         b'=' => n * 5 - 2,
         b'-' => n * 5 - 1,
         b'0' => n * 5,
         b'1' => n * 5 + 1,
         b'2' => n * 5 + 2,
         _ => unreachable!(),
      };
   }
   n
}

fn snafu_encode(n: u64) -> String {
   let mut a = vec![];
   let mut n = n;
   let mut d = 0u8;
   while n > 0 {
      (n, d) = match n % 5 {
         0 => (n / 5, b'0'),
         1 => (n / 5, b'1'),
         2 => (n / 5, b'2'),
         3 => (n / 5 + 1, b'='),
         4 => (n / 5 + 1, b'-'),
         _ => unreachable!(),
      };
      a.push(d)
   }
   a.reverse();
   String::from_utf8_lossy(a.as_slice()).to_string()
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
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

        assert_eq!("2=-1=0", part1(&input));
        assert_eq!(0, part2(&input));
    }
}
