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
pub fn solve_part1(input: &[Vec<Direction>]) -> i64 {
    use Direction::*;
    let mut world: HashMap<(i32, i32), bool> = HashMap::new();
    input.iter().for_each(|l| {
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

    world.values().map(|x| if *x { 1 } else { 0 }).sum()
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
}
