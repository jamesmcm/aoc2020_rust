use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    Active,
    Inactive,
}

impl TryFrom<char> for State {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Self::Active),
            '.' => Ok(Self::Inactive),
            _ => Err(anyhow!("Invalid char: {}", c)),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Active => "#",
                State::Inactive => ".",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct World {
    pub tiles: Vec<Vec<Vec<State>>>,
    pub tiles4d: Vec<Vec<Vec<Vec<State>>>>,
}

impl FromStr for World {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = get_input_slice(s)?;
        let inner_height = inner.len();
        let inner_width = inner[0].len();

        let mut base = vec![vec![vec![State::Inactive; inner_width + 12]; inner_height + 12]; 13];

        for row in inner.iter().enumerate() {
            for val in row.1.iter().enumerate() {
                base[6][6 + row.0][6 + val.0] = *val.1;
            }
        }

        let mut base4d =
            vec![vec![vec![vec![State::Inactive; inner_width + 12]; inner_height + 12]; 13]; 13];

        for row in inner.iter().enumerate() {
            for val in row.1.iter().enumerate() {
                base4d[6][6][6 + row.0][6 + val.0] = *val.1;
            }
        }
        Ok(World {
            tiles: base,
            tiles4d: base4d,
        })
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tiles
            .iter()
            .enumerate()
            .try_for_each(|plane| -> std::fmt::Result {
                write!(f, "z={}\n", plane.0)?;
                plane.1.iter().try_for_each(|row| -> std::fmt::Result {
                    write!(
                        f,
                        "{}\n",
                        row.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join("")
                    )?;
                    Ok(())
                })?;
                write!(f, "\n\n")?;
                Ok(())
            })?;
        Ok(())
    }
}

impl World {
    pub fn step(&mut self) {
        use State::*;
        let newtiles: Vec<Vec<Vec<State>>> = self
            .tiles
            .iter()
            .enumerate()
            .map(|plane| {
                (plane.1)
                    .iter()
                    .enumerate()
                    .map(|row| {
                        (row.1)
                            .iter()
                            .enumerate()
                            .map(|col| {
                                let neighbours = [
                                    (-1, -1, -1),
                                    (-1, -1, 0),
                                    (-1, -1, 1),
                                    (-1, 0, -1),
                                    (-1, 0, 0),
                                    (-1, 0, 1),
                                    (-1, 1, -1),
                                    (-1, 1, 0),
                                    (-1, 1, 1),
                                    (0, -1, -1),
                                    (0, -1, 0),
                                    (0, -1, 1),
                                    (0, 0, -1),
                                    (0, 0, 1),
                                    (0, 1, -1),
                                    (0, 1, 0),
                                    (0, 1, 1),
                                    (1, -1, -1),
                                    (1, -1, 0),
                                    (1, -1, 1),
                                    (1, 0, -1),
                                    (1, 0, 0),
                                    (1, 0, 1),
                                    (1, 1, -1),
                                    (1, 1, 0),
                                    (1, 1, 1),
                                ]
                                .iter()
                                .fold(0, |acc, x| {
                                    let index: (i32, i32, i32) = (
                                        plane.0 as i32 + x.0,
                                        row.0 as i32 + x.1,
                                        col.0 as i32 + x.2,
                                    );
                                    if index.0 >= 0 && index.1 >= 0 && index.2 >= 0 {
                                        if self
                                            .tiles
                                            .get(index.0 as usize)
                                            .map(|z| {
                                                z.get(index.1 as usize)
                                                    .map(|pz| pz.get(index.2 as usize))
                                                    .flatten()
                                            })
                                            .flatten()
                                            == Some(&Active)
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
                                    Active if neighbours == 2 || neighbours == 3 => Active,
                                    Inactive if neighbours == 3 => Active,
                                    _ => Inactive,
                                }
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        self.tiles = newtiles;
    }

    pub fn step_4d(&mut self) {
        use State::*;
        let newtiles: Vec<Vec<Vec<Vec<State>>>> = self
            .tiles4d
            .iter()
            .enumerate()
            .map(|hyper| {
                (hyper.1)
                    .iter()
                    .enumerate()
                    .map(|plane| {
                        (plane.1)
                            .iter()
                            .enumerate()
                            .map(|row| {
                                (row.1)
                                    .iter()
                                    .enumerate()
                                    .map(|col| {
                                        let neighbours = [
                                            (-1, -1, -1, -1),
                                            (-1, -1, -1, 0),
                                            (-1, -1, -1, 1),
                                            (-1, -1, 0, -1),
                                            (-1, -1, 0, 0),
                                            (-1, -1, 0, 1),
                                            (-1, -1, 1, -1),
                                            (-1, -1, 1, 0),
                                            (-1, -1, 1, 1),
                                            (-1, 0, -1, -1),
                                            (-1, 0, -1, 0),
                                            (-1, 0, -1, 1),
                                            (-1, 0, 0, -1),
                                            (-1, 0, 0, 0),
                                            (-1, 0, 0, 1),
                                            (-1, 0, 1, -1),
                                            (-1, 0, 1, 0),
                                            (-1, 0, 1, 1),
                                            (-1, 1, -1, -1),
                                            (-1, 1, -1, 0),
                                            (-1, 1, -1, 1),
                                            (-1, 1, 0, -1),
                                            (-1, 1, 0, 0),
                                            (-1, 1, 0, 1),
                                            (-1, 1, 1, -1),
                                            (-1, 1, 1, 0),
                                            (-1, 1, 1, 1),
                                            (0, -1, -1, -1),
                                            (0, -1, -1, 0),
                                            (0, -1, -1, 1),
                                            (0, -1, 0, -1),
                                            (0, -1, 0, 0),
                                            (0, -1, 0, 1),
                                            (0, -1, 1, -1),
                                            (0, -1, 1, 0),
                                            (0, -1, 1, 1),
                                            (0, 0, -1, -1),
                                            (0, 0, -1, 0),
                                            (0, 0, -1, 1),
                                            (0, 0, 0, -1),
                                            (0, 0, 0, 1),
                                            (0, 0, 1, -1),
                                            (0, 0, 1, 0),
                                            (0, 0, 1, 1),
                                            (0, 1, -1, -1),
                                            (0, 1, -1, 0),
                                            (0, 1, -1, 1),
                                            (0, 1, 0, -1),
                                            (0, 1, 0, 0),
                                            (0, 1, 0, 1),
                                            (0, 1, 1, -1),
                                            (0, 1, 1, 0),
                                            (0, 1, 1, 1),
                                            (1, -1, -1, -1),
                                            (1, -1, -1, 0),
                                            (1, -1, -1, 1),
                                            (1, -1, 0, -1),
                                            (1, -1, 0, 0),
                                            (1, -1, 0, 1),
                                            (1, -1, 1, -1),
                                            (1, -1, 1, 0),
                                            (1, -1, 1, 1),
                                            (1, 0, -1, -1),
                                            (1, 0, -1, 0),
                                            (1, 0, -1, 1),
                                            (1, 0, 0, -1),
                                            (1, 0, 0, 0),
                                            (1, 0, 0, 1),
                                            (1, 0, 1, -1),
                                            (1, 0, 1, 0),
                                            (1, 0, 1, 1),
                                            (1, 1, -1, -1),
                                            (1, 1, -1, 0),
                                            (1, 1, -1, 1),
                                            (1, 1, 0, -1),
                                            (1, 1, 0, 0),
                                            (1, 1, 0, 1),
                                            (1, 1, 1, -1),
                                            (1, 1, 1, 0),
                                            (1, 1, 1, 1),
                                        ]
                                        .iter()
                                        .fold(0, |acc, x| {
                                            let index: (i32, i32, i32, i32) = (
                                                hyper.0 as i32 + x.0,
                                                plane.0 as i32 + x.1,
                                                row.0 as i32 + x.2,
                                                col.0 as i32 + x.3,
                                            );
                                            if index.0 >= 0
                                                && index.1 >= 0
                                                && index.2 >= 0
                                                && index.3 >= 0
                                            {
                                                if self
                                                    .tiles4d
                                                    .get(index.0 as usize)
                                                    .map(|z| {
                                                        z.get(index.1 as usize).map(|pz| {
                                                            pz.get(index.2 as usize)
                                                                .map(|vv| vv.get(index.3 as usize))
                                                                .flatten()
                                                        })
                                                    })
                                                    .flatten()
                                                    .flatten()
                                                    == Some(&Active)
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
                                            Active if neighbours == 2 || neighbours == 3 => Active,
                                            Inactive if neighbours == 3 => Active,
                                            _ => Inactive,
                                        }
                                    })
                                    .collect()
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        self.tiles4d = newtiles;
    }

    pub fn count_active(&self) -> usize {
        self.tiles
            .iter()
            .map(|plane| -> usize {
                plane
                    .iter()
                    .map(|row| row.iter().filter(|x| **x == State::Active).count())
                    .sum()
            })
            .sum()
    }
    pub fn count_active_4d(&self) -> usize {
        self.tiles4d
            .iter()
            .map(|hyper| -> usize {
                hyper
                    .iter()
                    .map(|plane| -> usize {
                        plane
                            .iter()
                            .map(|row| row.iter().filter(|x| **x == State::Active).count())
                            .sum()
                    })
                    .sum()
            })
            .sum()
    }
}

fn get_input_slice(s: &str) -> Result<Vec<Vec<State>>> {
    Ok(s.lines()
        .map(|x| {
            x.chars()
                .map(State::try_from)
                .collect::<Result<Vec<State>>>()
        })
        .collect::<Result<Vec<Vec<State>>>>()?)
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Result<World> {
    World::from_str(input)
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &World) -> usize {
    let mut world = input.clone();
    for _i in 0..6 {
        world.step();
    }
    world.count_active()
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &World) -> usize {
    let mut world = input.clone();
    for _i in 0..6 {
        world.step_4d();
    }
    world.count_active_4d()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[ignore]
    #[test]
    fn test1_1() -> Result<()> {
        let ex = ".#.
..#
###";
        let inp = input_generator(ex)?;
        println!("{}", inp);
        Ok(())
    }
    #[test]
    fn test1_counts() -> Result<()> {
        let ex = ".#.
..#
###";
        let mut inp = input_generator(ex)?;
        assert_eq!(inp.count_active(), 5);
        inp.step();
        assert_eq!(inp.count_active(), 11);
        inp.step();
        assert_eq!(inp.count_active(), 21);
        inp.step();
        assert_eq!(inp.count_active(), 38);
        Ok(())
    }
    #[test]
    fn test1_part1() -> Result<()> {
        let ex = ".#.
..#
###";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 112);
        Ok(())
    }
    #[test]
    fn test2_part2() -> Result<()> {
        let ex = ".#.
..#
###";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 848);
        Ok(())
    }
}
