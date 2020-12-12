use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Bearing {
    East,
    South,
    West,
    North,
}

#[derive(Debug, Copy, Clone)]
pub enum InstructionType {
    Cardinal(Bearing),
    Left,
    Right,
    Forward,
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub val: i32,
}

impl FromStr for InstructionType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Bearing::*;
        match s {
            "N" => Ok(Self::Cardinal(North)),
            "S" => Ok(Self::Cardinal(South)),
            "E" => Ok(Self::Cardinal(East)),
            "W" => Ok(Self::Cardinal(West)),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "F" => Ok(Self::Forward),
            _ => Err(anyhow!("Unknown instruction type: {}", s)),
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_at(1);
        let ins = InstructionType::from_str(split.0)?;
        let val = split.1.parse()?;
        Ok(Self {
            instruction_type: ins,
            val,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Ship {
    pub bearing: Bearing,
    pub position: (i32, i32),
    pub waypoint: (i32, i32),
}

impl Ship {
    pub fn new() -> Self {
        Self {
            bearing: Bearing::East,
            position: (0, 0),
            waypoint: (10, 1),
        }
    }
    pub fn step_instruction(&mut self, ins: Instruction) {
        use Bearing::*;
        use InstructionType::*;

        match ins.instruction_type {
            Cardinal(b) => match b {
                North => {
                    self.position.1 += ins.val;
                }
                South => {
                    self.position.1 -= ins.val;
                }
                East => {
                    self.position.0 += ins.val;
                }
                West => {
                    self.position.0 -= ins.val;
                }
            },
            Forward => match self.bearing {
                North => {
                    self.position.1 += ins.val;
                }
                South => {
                    self.position.1 -= ins.val;
                }
                East => {
                    self.position.0 += ins.val;
                }
                West => {
                    self.position.0 -= ins.val;
                }
            },
            Left => {
                self.bearing = unsafe {
                    std::mem::transmute((self.bearing as i32 - (ins.val / 90)).rem_euclid(4) as u8)
                }
            }
            Right => {
                self.bearing = unsafe {
                    std::mem::transmute((self.bearing as i32 + (ins.val / 90)).rem_euclid(4) as u8)
                }
            }
        }
    }

    pub fn step_waypoint(&mut self, ins: Instruction) {
        use Bearing::*;
        use InstructionType::*;

        match ins.instruction_type {
            Cardinal(b) => match b {
                North => {
                    self.waypoint.1 += ins.val;
                }
                South => {
                    self.waypoint.1 -= ins.val;
                }
                East => {
                    self.waypoint.0 += ins.val;
                }
                West => {
                    self.waypoint.0 -= ins.val;
                }
            },
            Forward => {
                self.position.0 += ins.val * self.waypoint.0;
                self.position.1 += ins.val * self.waypoint.1;
            }
            Left => {
                let mut bearing = vec_to_bearing(self.waypoint);
                unsafe {
                    (bearing.0).1 = std::mem::transmute(
                        ((bearing.0).1 as i32 - (ins.val / 90)).rem_euclid(4) as u8,
                    );
                    (bearing.1).1 = std::mem::transmute(
                        ((bearing.1).1 as i32 - (ins.val / 90)).rem_euclid(4) as u8,
                    );
                }
                self.waypoint = bearing_to_vec(bearing);
            }
            Right => {
                let mut bearing = vec_to_bearing(self.waypoint);
                // println!("Bearing before: {:?}", bearing);
                unsafe {
                    (bearing.0).1 = std::mem::transmute(
                        ((bearing.0).1 as i32 + (ins.val / 90)).rem_euclid(4) as u8,
                    );
                    (bearing.1).1 = std::mem::transmute(
                        ((bearing.1).1 as i32 + (ins.val / 90)).rem_euclid(4) as u8,
                    );
                }
                // println!("Bearing after: {:?}", bearing);
                self.waypoint = bearing_to_vec(bearing);
            }
        }
    }
}

fn vec_to_bearing(waypoint: (i32, i32)) -> ((u32, Bearing), (u32, Bearing)) {
    use Bearing::*;
    let ew = if waypoint.0 >= 0 {
        (waypoint.0 as u32, East)
    } else {
        (waypoint.0.abs() as u32, West)
    };
    let ns = if waypoint.1 >= 0 {
        (waypoint.1 as u32, North)
    } else {
        (waypoint.1.abs() as u32, South)
    };

    (ew, ns)
}

fn bearing_to_vec(bearing: ((u32, Bearing), (u32, Bearing))) -> (i32, i32) {
    use Bearing::*;
    let mut x = 0;
    let mut y = 0;

    match (bearing.0).1 {
        East => {
            x = (bearing.0).0 as i32;
        }
        West => {
            x = -((bearing.0).0 as i32);
        }
        North => {
            y = (bearing.0).0 as i32;
        }
        South => {
            y = -((bearing.0).0 as i32);
        }
    };
    match (bearing.1).1 {
        East => {
            x = (bearing.1).0 as i32;
        }
        West => {
            x = -((bearing.1).0 as i32);
        }
        North => {
            y = (bearing.1).0 as i32;
        }
        South => {
            y = -((bearing.1).0 as i32);
        }
    };

    (x, y)
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(|x| Instruction::from_str(x)).collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Instruction]) -> i32 {
    let mut ship = Ship::new();
    for ins in input {
        ship.step_instruction(*ins)
    }
    ship.position.0.abs() + ship.position.1.abs()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Instruction]) -> i32 {
    let mut ship = Ship::new();
    for ins in input {
        ship.step_waypoint(*ins);
        // println!("Ship: {:?}, Waypoint: {:?}", ship.position, ship.waypoint);
    }
    ship.position.0.abs() + ship.position.1.abs()
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() -> Result<()> {
        let ex = "F10
N3
F7
R90
F11";
        let input = input_generator(ex)?;
        assert_eq!(solve_part1(&input), 25);
        Ok(())
    }
    #[test]
    fn test2() -> Result<()> {
        let ex = "F10
N3
F7
R90
F11";
        let input = input_generator(ex)?;
        assert_eq!(solve_part2(&input), 286);
        Ok(())
    }
}
