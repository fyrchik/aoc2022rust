use rustc_hash::FxHashMap;

pub fn part1(input: &str) -> u32 {
    let (dist, rates, start) = parse(input);
    let mut seen = u16::MAX;
    for (i, v) in rates.iter().enumerate() {
        if *v != 0 {
            seen = seen & !(1 << i);
        }
    }

    let g = Graph { dist, rates };
    g.dfs(seen, start, 0, 30)
}

pub fn part2(input: &str) -> u32 {
   let (dist, rates, start) = parse(input);
   let mut seen = u16::MAX;
   for (i, v) in rates.iter().enumerate() {
       if *v != 0 {
           seen = seen & !(1 << i);
       }
   }

   let g = Graph { dist, rates };
   let mut m = FxHashMap::<u16, u32>::default();
   g.dfs_multi(seen, start, 0, 26, &mut m);

   let mut max = 0;
   for (kx, vx) in m.iter() {
      for (ky, vy) in m.iter() {
         if kx & ky & !seen == 0 {
            max = max.max(vx + vy);
         }
      }
   }
   max
}

struct Graph {
    dist: Vec<Vec<u32>>,
    rates: Vec<u32>,
}

impl Graph {
    fn dfs(&self, seen: u16, current: usize, opened: u32, time: u32) -> u32 {
        let mut max = opened * time;
        for i in (0..16).filter(|&t| seen & (1 << t) == 0 && time > self.dist[current][t]) {
            let path = self.dist[current][i];
            let m = self.dfs(seen | (1 << i), i, opened + self.rates[i], time - path - 1);
            max = max.max(m + opened * (path + 1));
        }
        max
    }


    fn dfs_multi(&self, seen: u16, current: usize, total: u32, time: u32, m: &mut FxHashMap<u16, u32>) {
      m.entry(seen).and_modify(|v| *v = total.max(*v)).or_insert(total);

      for i in (0..16).filter(move |&t| seen & (1 << t) == 0 && time > self.dist[current][t]) {
          let time_left = time - self.dist[current][i] - 1;
          self.dfs_multi(seen | (1 << i), i, total + self.rates[i] * time_left, time_left, m);
      }
   }
}

fn parse(input: &str) -> (Vec<Vec<u32>>, Vec<u32>, usize) {
    let mut names = FxHashMap::<&[u8], usize>::default();
    let mut rates: Vec<u32> = vec![];
    let mut neighbours: Vec<Vec<usize>> = vec![];
    input
        .trim_end()
        .as_bytes()
        .split(|c| *c == b'\n')
        .for_each(|b| {
            let name = &b[6..8];
            let index = *names.entry(name).or_insert_with(|| {
                rates.push(0);
                neighbours.push(vec![]);
                rates.len() - 1
            });

            let (rate, rate_len) = int_from_bytes_prefix::<u32>(&b[23..]);
            rates[index] = rate;

            let mut i = 23 + rate_len + 25;
            while i + 2 <= b.len() {
                let name = &b[i..i + 2];
                let id = *names.entry(name).or_insert_with(|| {
                    rates.push(0);
                    neighbours.push(vec![]);
                    rates.len() - 1
                });
                neighbours[index].push(id);

                i += 2 + 2; // ", "
            }
        });

    let mut start = *names.get("AA".as_bytes()).unwrap();

    let mut graph = vec![];
    for v in neighbours.iter() {
        let mut lst = vec![u32::MAX; neighbours.len()];
        for n in v.iter() {
            lst[*n] = 1
        }
        lst[graph.len()] = 0;

        graph.push(lst);
    }

    // Floyd-Warshall algorithm.
    for k in 0..graph.len() {
        for i in 0..graph.len() {
            for j in 0..graph.len() {
                let sum = graph[i][k].saturating_add(graph[k][j]);
                if sum < graph[i][j] {
                    graph[i][j] = sum;
                    graph[j][i] = sum;
                }
            }
        }
    }

    let mut i = 0;
    while i < graph.len() {
        if i == start || rates[i] != 0 {
            i += 1;
            continue;
        }

        if i < start {
            start -= 1;
        }

        rates.remove(i);
        graph.remove(i);
        for r in graph.iter_mut() {
            r.remove(i);
        }
    }
    (graph, rates, start)
}

fn int_from_bytes_prefix<T>(s: &[u8]) -> (T, usize)
where
    T: From<u8> + std::ops::MulAssign + std::ops::AddAssign,
{
    let mut n = T::from(0);
    for (i, &c) in s.iter().enumerate() {
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
            _ => return (n, i),
        };
        n *= T::from(10);
        n += T::from(r);
    }

    (n, s.len())
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
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        assert_eq!(1651, part1(&input));
        assert_eq!(1707, part2(&input));
    }
}
