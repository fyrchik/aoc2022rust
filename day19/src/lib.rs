pub fn part1(input: &str) -> u32 {
    let bs = parse(input);
    let mut sum = 0;
    for (i, &b) in bs.iter().enumerate() {
        let robots = [1, 0, 0, 0];
        let max_ore = b.ore.max(b.clay).max(b.obsidian_ore).max(b.geode_ore);
        let mut s = State {
            b,
            max_ore,
            max: 0,
            ro: robots,
            re: [0u32; 4],
        };
        s.count_max_geodes_dfs(24);
        sum += (i as u32 + 1) * s.max;
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let bs = parse(input);
    let mut p = 1;
    for &b in bs.iter().take(3) {
        let robots = [1, 0, 0, 0];
        let max_ore = b.ore.max(b.clay).max(b.obsidian_ore).max(b.geode_ore);
        let mut s = State {
            b,
            max_ore,
            max: 0,
            ro: robots,
            re: [0u32; 4],
        };
        s.count_max_geodes_dfs(32);
        p *= s.max;
    }
    p
}

const ORE: usize = 0;
const CLA: usize = 1;
const OBS: usize = 2;
const GEO: usize = 3;

struct State {
    b: Blueprint,
    ro: [u32; 4],
    re: [u32; 4],
    max_ore: u32,
    max: u32,
}

impl State {
    fn build(&mut self, robot: usize) {
        self.mine();

        self.ro[robot] += 1;
        match robot {
            ORE => self.re[ORE] -= self.b.ore,
            CLA => self.re[ORE] -= self.b.clay,
            OBS => {
                self.re[ORE] -= self.b.obsidian_ore;
                self.re[CLA] -= self.b.obsidian_clay;
            }
            GEO => {
                self.re[ORE] -= self.b.geode_ore;
                self.re[OBS] -= self.b.geode_obsidian;
            }
            _ => unreachable!(),
        }
    }

    fn can_build(&self, robot: usize) -> bool {
        match robot {
            ORE => self.b.ore <= self.re[ORE],
            CLA => self.b.clay <= self.re[ORE],
            OBS => self.b.obsidian_ore <= self.re[ORE] && self.b.obsidian_clay <= self.re[CLA],
            GEO => self.b.geode_ore <= self.re[ORE] && self.b.geode_obsidian <= self.re[OBS],
            _ => unreachable!(),
        }
    }

    fn mine(&mut self) {
        self.re[0] += self.ro[0];
        self.re[1] += self.ro[1];
        self.re[2] += self.ro[2];
        self.re[3] += self.ro[3];
    }

    fn count_max_geodes_dfs(&mut self, time: u32) {
        if time == 0 {
            self.max = self.max.max(self.re[GEO]);
            return;
        }

        let max_geodes = self.re[GEO] + self.ro[GEO] * time + time * (time - 1) / 2;
        if max_geodes == 0 || self.max > max_geodes || self.max > 0 && self.max == max_geodes {
            return;
        }

        let (ro, re) = (self.ro, self.re);
        if self.can_build(GEO) {
            self.build(GEO);
            self.count_max_geodes_dfs(time - 1);
            (self.ro, self.re) = (ro, re);
        }
        if time > 1 {
            if self.can_build(OBS) && self.ro[OBS] < self.b.geode_obsidian {
                self.build(OBS);
                self.count_max_geodes_dfs(time - 1);
                (self.ro, self.re) = (ro, re);
            }
            if self.can_build(CLA) && self.ro[CLA] < self.b.obsidian_clay {
                self.build(CLA);
                self.count_max_geodes_dfs(time - 1);
                (self.ro, self.re) = (ro, re);
            }
            if self.can_build(ORE) && self.ro[ORE] < self.max_ore {
                self.build(ORE);
                self.count_max_geodes_dfs(time - 1);
                (self.ro, self.re) = (ro, re);
            }
        }

        self.mine();
        self.count_max_geodes_dfs(time - 1);
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    input
        .trim_end()
        .as_bytes()
        .split(|c| *c == b'\n')
        .map(|b| {
            let mut start = b.iter().position(|c| *c == b':').unwrap();
            start += 23; // ": Each ore robot costs "

            let (ore, ln) = aoc::uint_from_bytes_prefix(&b[start..]);
            start += ln + 28; // " ore. Each clay robot costs "

            let (clay, ln) = aoc::uint_from_bytes_prefix(&b[start..]);
            start += ln + 32; // " ore. Each obsidian robot costs "

            let (obsidian_ore, ln) = aoc::uint_from_bytes_prefix(&b[start..]);
            start += ln + 9; // " ore and "

            let (obsidian_clay, ln) = aoc::uint_from_bytes_prefix(&b[start..]);
            start += ln + 30; // " clay. Each geode robot costs "

            let (geode_ore, ln) = aoc::uint_from_bytes_prefix(&b[start..]);
            start += ln + 9; // " ore and "

            let (geode_obsidian, _) = aoc::uint_from_bytes_prefix(&b[start..]);
            Blueprint {
                ore,
                clay,
                obsidian_ore,
                obsidian_clay,
                geode_ore,
                geode_obsidian,
            }
        })
        .collect()
}

#[derive(Copy, Clone, Debug)]
struct Blueprint {
    ore: u32,
    clay: u32,
    obsidian_ore: u32,
    obsidian_clay: u32,
    geode_ore: u32,
    geode_obsidian: u32,
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
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

        assert_eq!(33, part1(&input));
        assert_eq!(56 * 62, part2(&input));
    }
}
