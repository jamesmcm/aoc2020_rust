use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    Empty,
    Occupied,
    Floor,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Floor),
            'L' => Ok(Self::Empty),
            '#' => Ok(Self::Occupied),
            x => Err(anyhow!("Unknown tile: {}", x)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct World {
    pub tiles: Vec<Vec<Tile>>,
}

impl FromStr for World {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(World {
            tiles: s
                .lines()
                .map(|l| l.trim().chars().map(|c| Tile::try_from(c)).collect())
                .collect::<Result<Vec<Vec<Tile>>>>()?,
        })
    }
}

impl World {
    pub fn step(&mut self) {
        use Tile::*;
        let newtiles: Vec<Vec<Tile>> = self
            .tiles
            .iter()
            .enumerate()
            .map(|row| {
                (row.1)
                    .iter()
                    .enumerate()
                    .map(|col| {
                        let neighbours = [
                            (-1, -1),
                            (-1, 0),
                            (-1, 1),
                            (0, -1),
                            (0, 1),
                            (1, -1),
                            (1, 0),
                            (1, 1),
                        ]
                        .iter()
                        .fold(0, |acc, x| {
                            let index: (i32, i32) = (row.0 as i32 + x.0, col.0 as i32 + x.1);
                            if index.0 >= 0 && index.1 >= 0 {
                                if self
                                    .tiles
                                    .get(index.0 as usize)
                                    .map(|z| z.get(index.1 as usize))
                                    .flatten()
                                    == Some(&Occupied)
                                {
                                    acc + 1
                                } else {
                                    acc
                                }
                            } else {
                                acc
                            }
                        });

                        match col.1 {
                            Empty if neighbours == 0 => Occupied,
                            Occupied if neighbours >= 4 => Empty,
                            x => *x,
                        }
                    })
                    .collect()
            })
            .collect();

        self.tiles = newtiles;
    }

    pub fn step2(&mut self) {
        use Tile::*;
        let newtiles: Vec<Vec<Tile>> = self
            .tiles
            .iter()
            .enumerate()
            .map(|row| {
                (row.1)
                    .iter()
                    .enumerate()
                    .map(|col| {
                        let seen = self.get_los(row.0, col.0);

                        let neighbours = seen.iter().filter(|&&x| x == Occupied).count();

                        match col.1 {
                            Empty if neighbours == 0 => Occupied,
                            Occupied if neighbours >= 5 => Empty,
                            x => *x,
                        }
                    })
                    .collect()
            })
            .collect();

        self.tiles = newtiles;
    }

    pub fn get_los(&self, row: usize, col: usize) -> Vec<Tile> {
        let size = self.tiles.len().max(self.tiles[0].len());
        let directions = &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        directions
            .iter()
            .map(|d| {
                let mut found = Tile::Floor;
                for i in 1..size {
                    let mut dir: (i32, i32) = (d.0 * i as i32, d.1 * i as i32);
                    let pos = (row as i32 + dir.0, col as i32 + dir.1);
                    if pos.0 >= 0 && pos.1 >= 0 {
                        match self
                            .tiles
                            .get(pos.0 as usize)
                            .map(|t| t.get(pos.1 as usize))
                            .flatten()
                        {
                            None => {
                                found = Tile::Floor;
                                break;
                            }
                            Some(Tile::Occupied) => {
                                found = Tile::Occupied;
                                break;
                            }
                            Some(Tile::Empty) => {
                                found = Tile::Empty;
                                break;
                            }
                            Some(Tile::Floor) => {}
                        }
                    }
                }
                found
            })
            .collect()
    }
}
#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Result<World> {
    World::from_str(input)
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &World) -> usize {
    let mut world = input.clone();
    loop {
        let old_world = world.clone();
        world.step();
        if world == old_world {
            break world.tiles.iter().fold(0, |acc, x| {
                acc + x
                    .iter()
                    .fold(0, |sum, y| if *y == Tile::Occupied { sum + 1 } else { sum })
            });
        }
    }
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &World) -> usize {
    let mut world = input.clone();
    loop {
        let old_world = world.clone();
        world.step2();
        if world == old_world {
            break world.tiles.iter().fold(0, |acc, x| {
                acc + x
                    .iter()
                    .fold(0, |sum, y| if *y == Tile::Occupied { sum + 1 } else { sum })
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tile::*;
    use super::*;

    #[test]
    fn test_ticks() -> Result<()> {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let mut world = input_generator(input)?;

        world.step();
        let new_world = World::from_str(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        )?;
        assert_eq!(world, new_world);
        world.step();
        let new_world = World::from_str(
            "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
        )?;
        assert_eq!(world, new_world);
        world.step();
        let new_world = World::from_str(
            "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
        )?;
        assert_eq!(world, new_world);
        world.step();
        let new_world = World::from_str(
            "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
        )?;
        assert_eq!(world, new_world);
        world.step();
        let new_world = World::from_str(
            "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
        )?;
        assert_eq!(world, new_world);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let world = input_generator(input)?;
        assert_eq!(solve_part1(&world), 37);
        Ok(())
    }
    #[test]
    fn test_los() -> Result<()> {
        let input = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";

        let world = input_generator(input)?;
        assert_eq!(world.get_los(4, 3), vec![Occupied; 8]);

        let input = ".............
.L.L.#.#.#.#.
.............";
        let world = input_generator(input)?;
        assert_eq!(
            world.get_los(1, 1),
            vec![Floor, Floor, Floor, Floor, Empty, Floor, Floor, Floor]
        );

        let input = ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";
        let world = input_generator(input)?;
        assert_eq!(world.get_los(3, 3), vec![Floor; 8]);
        Ok(())
    }

    #[test]
    fn test_ticks2() -> Result<()> {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let mut world = input_generator(input)?;

        world.step2();
        let new_world = World::from_str(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        )?;
        assert_eq!(world, new_world);
        world.step2();
        let new_world = World::from_str(
            "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
        )?;
        assert_eq!(world, new_world);
        world.step2();
        let new_world = World::from_str(
            "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#",
        )?;
        assert_eq!(world, new_world);
        world.step2();
        let new_world = World::from_str(
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#",
        )?;
        assert_eq!(world, new_world);
        world.step2();
        let new_world = World::from_str(
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
        )?;
        assert_eq!(world, new_world);
        world.step2();
        let new_world = World::from_str(
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
        )?;
        assert_eq!(world, new_world);
        Ok(())
    }
    #[test]
    fn test_part2() -> Result<()> {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let world = input_generator(input)?;
        assert_eq!(solve_part2(&world), 26);
        Ok(())
    }
}
