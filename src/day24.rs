use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::str::FromStr;
use std::string::ToString;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Vec<Direction>> {
    let mut outvec = Vec::new();
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut linevec = Vec::new();
        use Direction::*;
        let mut i = 0;
        while i < chars.len() {
            let dir = match chars[i] {
                'e' => {
                    i += 1;
                    East
                }
                'w' => {
                    i += 1;
                    West
                }
                's' => {
                    let res = match chars[i + 1] {
                        'e' => SouthEast,
                        'w' => SouthWest,
                        _ => panic!("Unknown char {:?}", chars[i + 1]),
                    };
                    i += 2;
                    res
                }
                'n' => {
                    let res = match chars[i + 1] {
                        'e' => NorthEast,
                        'w' => NorthWest,
                        _ => panic!("Unknown char {:?}", chars[i + 1]),
                    };
                    i += 2;
                    res
                }
                _ => panic!("Unknown char {:?}", chars[i]),
            };
            linevec.push(dir);
        }
        outvec.push(linevec);
    }
    outvec
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &[Vec<Direction>]) -> usize {
    let world: World = World::new(input);
    world.count_active()
}

pub struct World {
    map: HashMap<(i32, i32), bool>,
}

impl World {
    pub fn new(init: &[Vec<Direction>]) -> Self {
        use Direction::*;
        let mut world: HashMap<(i32, i32), bool> = HashMap::new();
        init.iter().for_each(|l| {
            let mut pos = (0, 0);
            for d in l {
                match d {
                    East => {
                        pos = (pos.0 + 2, pos.1);
                    }
                    West => {
                        pos = (pos.0 - 2, pos.1);
                    }
                    NorthEast => {
                        pos = (pos.0 + 1, pos.1 + 1);
                    }
                    NorthWest => {
                        pos = (pos.0 - 1, pos.1 + 1);
                    }
                    SouthEast => {
                        pos = (pos.0 + 1, pos.1 - 1);
                    }
                    SouthWest => {
                        pos = (pos.0 - 1, pos.1 - 1);
                    }
                }
            }

            world.entry(pos).and_modify(|x| *x = !*x).or_insert(true);
        });
        Self { map: world }
    }

    pub fn step(&mut self) {
        let mut all_possible: HashSet<(i32, i32)> = HashSet::new();
        self.map
            .keys()
            .map(|k| (k, self.get_neighbours(*k)))
            .for_each(|x| {
                all_possible.insert(*x.0);
                (x.1).iter().for_each(|y| {
                    all_possible.insert(*y);
                });
            });

        let mut new_map = self.map.clone();

        all_possible.iter().for_each(|x| {
            let num_active: usize = self
                .get_neighbours(*x)
                .iter()
                .map(|p| self.map.get(&p).unwrap_or(&false))
                .map(|&x| if x { 1 } else { 0 })
                .sum();
            if *self.map.get(&x).unwrap_or(&false) {
                if num_active == 0 || num_active > 2 {
                    new_map.entry(*x).and_modify(|x| *x = false);
                }
            } else if num_active == 2 {
                new_map.entry(*x).and_modify(|x| *x = true).or_insert(true);
            }
        });

        self.map = new_map;
    }
    pub fn get_neighbours(&self, pos: (i32, i32)) -> Vec<(i32, i32)> {
        vec![
            (pos.0 + 2, pos.1),
            (pos.0 - 2, pos.1),
            (pos.0 + 1, pos.1 + 1),
            (pos.0 - 1, pos.1 + 1),
            (pos.0 + 1, pos.1 - 1),
            (pos.0 - 1, pos.1 - 1),
        ]
    }

    pub fn count_active(&self) -> usize {
        self.map.values().map(|x| if *x { 1 } else { 0 }).sum()
    }
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &[Vec<Direction>]) -> usize {
    // Initialisation
    let mut world = World::new(input);
    for _i in 0..100 {
        world.step();
    }
    world.count_active()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_parse1() {
        use Direction::*;
        let ex = "esenee";
        let inp = input_generator(ex);
        assert_eq!(inp[0], vec![East, SouthEast, NorthEast, East]);
    }
    #[test]
    fn test1_0() {
        let ex = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        let inp = input_generator(ex);
        assert_eq!(solve_part1(&inp), 10);
    }
    #[test]
    fn test2_0() {
        let ex = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        let inp = input_generator(ex);
        let mut world = World::new(&inp);
        assert_eq!(world.count_active(), 10);
        world.step();
        assert_eq!(world.count_active(), 15);
        world.step();
        assert_eq!(world.count_active(), 12);
        world.step();
        assert_eq!(world.count_active(), 25);
        world.step();
        assert_eq!(world.count_active(), 14);
        world.step();
        assert_eq!(world.count_active(), 23);
        world.step();
        assert_eq!(world.count_active(), 28);
        world.step();
        assert_eq!(world.count_active(), 41);
        world.step();
        assert_eq!(world.count_active(), 37);
        world.step();
        assert_eq!(world.count_active(), 49);
        world.step();
        assert_eq!(world.count_active(), 37);
    }
    #[test]
    fn test2_1() {
        let ex = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        let inp = input_generator(ex);
        assert_eq!(solve_part2(&inp), 2208);
    }
}
