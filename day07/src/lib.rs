#![feature(extend_one)]

/// Pair represents a directory in a tree. Parent always points to a previously seen element.
/// In both parts we assume that each directory is visited (`ls`) only once.
struct Pair {
    parent: usize,
    size: u32,
}

pub fn part1(input: &str) -> u32 {
    let mut tree: Vec<Pair> = vec![Pair { parent: 0, size: 0 }];
    let mut current = 0;

    input.as_bytes().split(|&c| c == b'\n').for_each(|b| {
        if b.is_empty() {
            return;
        }
        if b[0] == b'$' && b[2] == b'c' {
            current = if b[5] == b'.' {
                tree[current].parent
            } else {
                tree.push(Pair {
                    parent: current,
                    size: 0,
                });
                tree.len() - 1
            }
        } else if b[0] != b'$' && b[0] != b'd' {
            let sz = int_from_bytes_prefix::<u32>(b);
            tree[current].size += sz;
        }
    });

    for i in (1..tree.len()).rev() {
        let parent = tree[i].parent;
        tree[parent].size += tree[i].size;
    }

    tree.iter()
        .map(|p| p.size)
        .filter(|&size| size <= 100_000)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut tree: Vec<Pair> = vec![Pair { parent: 0, size: 0 }];
    let mut current = 0;

    input.as_bytes().split(|&c| c == b'\n').for_each(|b| {
        if b.is_empty() {
            return;
        }
        if b[0] == b'$' && b[2] == b'c' {
            current = if b[5] == b'.' {
                tree[current].parent
            } else {
                tree.push(Pair {
                    parent: current,
                    size: 0,
                });
                tree.len() - 1
            }
        } else if b[0] != b'$' && b[0] != b'd' {
            let sz = int_from_bytes_prefix::<u32>(b);
            tree[current].size += sz;
        }
    });

    for i in (1..tree.len()).rev() {
        let parent = tree[i].parent;
        tree[parent].size += tree[i].size;
    }

    let available = 70000000;
    let need = 30000000;
    let used = tree[0].size;
    let free = available - used;
    if need <= free {
        return 0;
    }

    let min_remove = need - free;
    tree.iter()
        .map(|p| p.size)
        .filter(|&size| min_remove <= size)
        .min()
        .unwrap_or(u32::MAX)
}

fn int_from_bytes_prefix<T>(s: &[u8]) -> T
where
    T: From<u8> + std::ops::MulAssign + std::ops::AddAssign,
{
    let mut n = T::from(0);
    for &c in s {
        let r = match c {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            _ => return n,
        };
        n *= T::from(10);
        n += T::from(r);
    }
    n
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(95437, part1(input));
        assert_eq!(24933642, part2(input));
    }
}
