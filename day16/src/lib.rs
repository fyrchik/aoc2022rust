use rustc_hash::FxHashMap;

pub fn part1(input: &str) -> u32 {
    use pathfinding::prelude::*;
    let (dist, rates, start) = parse(input);
    let mut seen = u16::MAX;
    for (i, v) in rates.iter().enumerate() {
        if *v != 0 {
            seen &= !(1 << i);
        }
    }

    // Calculate "shortest" path by having "u32::MAX - flow" as a metric.
    let res = astar(
        &(start, seen, 30u32),
        |&(node, seen, time)| {
            let mut v = [((0, 0, 0), ReverseU32(0)); 16];
            let mut n = 16;

            // Allow to end immediately with zero cost.
            n -= 1;
            v[n] = ((start, u16::MAX, 0), ReverseU32(0));

            for t in (0..16).filter(|&t| seen & (1 << t) == 0 && time > dist[node][t]) {
                let d = time - dist[node][t] - 1;

                n -= 1;
                v[n] = ((t, seen | (1 << t), d), ReverseU32(d * rates[t]))
            }
            aoc::Iter { v, n }
        },
        |&(n, seen, time)| {
            let mut flow = 0;
            let mut min_path = u32::MAX;
            for t in (0..16).filter(|&t| seen & (1 << t) == 0) {
                flow += rates[t];
                min_path = min_path.min(dist[n][t])
            }
            ReverseU32(if min_path < time {
                flow * (time - min_path)
            } else {
                0
            })
        },
        |&(_, _, time)| time == 0,
    );
    res.unwrap().1 .0
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct ReverseU32(u32);

impl core::ops::Add for ReverseU32 {
    type Output = ReverseU32;
    fn add(self, rhs: Self) -> Self::Output {
        ReverseU32(self.0 + rhs.0)
    }
}

impl num_traits::Zero for ReverseU32 {
    fn zero() -> Self {
        ReverseU32(0)
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl std::cmp::PartialOrd for ReverseU32 {
    fn partial_cmp(&self, other: &ReverseU32) -> Option<std::cmp::Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl std::cmp::Ord for ReverseU32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

pub fn part2(input: &str) -> u32 {
    let (dist, rates, start) = parse(input);
    let mut seen = u16::MAX;
    for (i, v) in rates.iter().enumerate() {
        if *v != 0 {
            seen &= !(1 << i);
        }
    }

    let g = Graph { dist, rates };

    // We use 2 maps to store routes with and without one of the nodes.
    // This allows us to find the maximum a 4 times faster (n^2 vs (n/2)^2).
    // Here we assume that all nodes should be visited by the end of the time.
    let mut m1 = FxHashMap::<u16, u32>::default();
    let mut m2 = FxHashMap::<u16, u32>::default();
    let mask = 1 << (start == 0) as usize;
    g.dfs_multi(mask, seen, start, 0, 26, &mut m1, &mut m2);

    let mut max = 0;
    for (kx, vx) in m1.iter() {
        for (ky, vy) in m2.iter() {
            if kx & ky == seen {
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
    fn dfs_multi(
        &self,
        mask: u16,
        seen: u16,
        current: usize,
        total: u32,
        time: u32,
        m1: &mut FxHashMap<u16, u32>,
        m2: &mut FxHashMap<u16, u32>,
    ) {
        if seen & mask == mask {
            m1.entry(seen)
                .and_modify(|v| *v = total.max(*v))
                .or_insert(total);
        } else {
            m2.entry(seen)
                .and_modify(|v| *v = total.max(*v))
                .or_insert(total);
        }

        for i in (0..16).filter(move |&t| seen & (1 << t) == 0 && time > self.dist[current][t]) {
            let time_left = time - self.dist[current][i] - 1;
            self.dfs_multi(
                mask,
                seen | (1 << i),
                i,
                total + self.rates[i] * time_left,
                time_left,
                m1,
                m2,
            );
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

            let (rate, rate_len) = aoc::uint_from_bytes_prefix::<u32>(&b[23..]);
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

    let mut graph = Vec::with_capacity(neighbours.len());
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
