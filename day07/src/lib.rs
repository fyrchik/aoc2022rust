use std::collections::HashMap;

enum Data {
    Dir(Vec<String>),
    File(u32),
}

pub fn part1(input: &str) -> u32 {
    let mut sizes = HashMap::<String, u32>::new();
    let mut dir = HashMap::<String, Data>::new();
    let mut current: Vec<String> = vec![];
    input.lines().for_each(|s| {
        let b = s.as_bytes();
        match b[0] {
            b'$' => {
                if b[2..4].eq(b"cd") {
                    if b[5..].eq(b"..") {
                        _ = current.pop()
                    } else {
                        current.push(s[5..].to_string())
                    }
                }
            }
            _ => {
                let cur = current.join("/");
                let (size, name) = s.split_once(' ').unwrap();
                if size == "dir" {
                    dir.entry(cur)
                        .and_modify(|e| {
                            if let Data::Dir(v) = e {
                                v.push(name.to_string())
                            }
                        })
                        .or_insert(Data::Dir(vec![name.to_string()]));
                } else {
                    let sz = size.parse::<u32>().unwrap();
                    for i in 0..current.len() {
                        let name = current[0..i].join("/");
                        sizes.entry(name).and_modify(|s| *s += sz).or_insert(sz);
                    }
                }
            }
        }
    });

    let mut sizes = HashMap::<String, u32>::new();
    for (k, v) in dir {
        match v {
            Data::File(s) => {
                for i in k.as_bytes().iter().enumerate().filter(|(_, &c)| c == b'/') {
                    sizes
                        .entry(k.as_str()[0..i.0].to_string())
                        .and_modify(|sz| *sz += s)
                        .or_insert(s);
                }
            }
            _ => {}
        }
    }

    let mut total = 0u32;
    for &s in sizes.values() {
        if s <= 100_000 {
            total += s;
        }
    }

    total
}

pub fn part2(input: &str) -> u32 {
    let mut dir = HashMap::<String, Data>::new();
    let mut current: Vec<String> = vec![];
    input.lines().for_each(|s| {
        let b = s.as_bytes();
        match b[0] {
            b'$' => {
                if b[2..4].eq("cd".as_bytes()) {
                    if b[5..].eq("..".as_bytes()) {
                        _ = current.pop()
                    } else {
                        current.push(s[5..].to_string())
                    }
                }
            }
            _ => {
                let cur = current.join("/");
                let (size, name) = s.split_once(' ').unwrap();
                if size == "dir" {
                    dir.entry(cur)
                        .and_modify(|e| {
                            if let Data::Dir(v) = e {
                                v.push(name.to_string())
                            }
                        })
                        .or_insert(Data::Dir(vec![name.to_string()]));
                } else {
                    let sz = size.parse::<u32>().unwrap();
                    dir.insert(cur + "/" + name, Data::File(sz));
                }
            }
        }
    });

    let mut sizes = HashMap::<String, u32>::new();
    for (k, v) in dir {
        match v {
            Data::File(s) => {
                for i in k.as_bytes().iter().enumerate().filter(|(_, &c)| c == b'/') {
                    sizes
                        .entry(k.as_str()[0..i.0].to_string())
                        .and_modify(|sz| *sz += s)
                        .or_insert(s);
                }
            }
            _ => {}
        }
    }

    let available = 70000000;
    let need = 30000000;
    let used = sizes.get("").unwrap();
    let free = available - used;
    if need <= free {
        return 0;
    }

    let min_remove = need - free;
    let mut min_size = u32::MAX;
    for (_, s) in sizes {
        if min_remove < s && s < min_size {
            min_size = s;
        }
    }

    min_size
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

        assert_eq!(95437, part1(&input));
        assert_eq!(24933642, part2(&input));
    }
}
