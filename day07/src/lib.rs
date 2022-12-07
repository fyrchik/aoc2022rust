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
            let i = b.iter().enumerate().find(|p| *p.1 == b' ').unwrap().0;
            let sz = int_from_bytes::<u32>(&b[..i]);
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
            let i = b.iter().enumerate().find(|p| *p.1 == b' ').unwrap().0;
            let sz = int_from_bytes::<u32>(&b[..i]);
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

fn int_from_bytes<T>(s: &[u8]) -> T
where
    T: From<u8> + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>,
{
    s.iter().fold(T::from(0), |n, c| {
        let r = match c {
            b'0' => Some(T::from(0)),
            b'1' => Some(T::from(1)),
            b'2' => Some(T::from(2)),
            b'3' => Some(T::from(3)),
            b'4' => Some(T::from(4)),
            b'5' => Some(T::from(5)),
            b'6' => Some(T::from(6)),
            b'7' => Some(T::from(7)),
            b'8' => Some(T::from(8)),
            b'9' => Some(T::from(9)),
            _ => None,
        };
        n * T::from(10) + r.unwrap()
    })
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
