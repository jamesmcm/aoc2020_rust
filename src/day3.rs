use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};

use std::convert::TryFrom;

#[derive(Debug)]
pub enum Tile {
    Open,
    Tree,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;
    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Tile::Open),
            '#' => Ok(Tile::Tree),
            _ => Err(anyhow!("Unknown tile: {}", c)),
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<Tile>>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Tile::try_from(c))
                .collect::<Result<Vec<Tile>, _>>()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(map: &[Vec<Tile>]) -> i32 {
    let slope = (3, 1);
    common_solve(slope, map)
}

#[aoc(day3, part2)]
pub fn solve_part2(map: &[Vec<Tile>]) -> i32 {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().fold(1, |acc, x| acc * common_solve(*x, map))
}

fn common_solve(slope: (usize, usize), map: &[Vec<Tile>]) -> i32 {
    let width = map[0].len();
    let height = map.len();
    let mut pos = (0, 0);
    let mut sum = 0;
    while pos.1 < height {
        pos = ((pos.0 + slope.0 as usize) % width, pos.1 + slope.1);
        if pos.1 >= height {
            break;
        }

        if let Tile::Tree = map[pos.1][pos.0] {
            sum += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() -> Result<()> {
        let ex = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let input = input_generator(ex)?;
        assert_eq!(solve_part1(&input), 7);
        Ok(())
    }
    #[test]
    fn test2() -> Result<()> {
        let ex = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let map = input_generator(ex)?;

        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let vecsols: Vec<i32> = slopes.iter().map(|x| common_solve(*x, &map)).collect();
        assert_eq!(vecsols, vec![2, 7, 3, 4, 2]);
        Ok(())
    }
}
