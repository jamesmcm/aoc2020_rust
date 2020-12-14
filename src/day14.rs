use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum InstructionType {
    Mask,
    Write,
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub addr: usize,
    pub val: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref mask_regex: Regex =
                Regex::new(r"mask = ([01X]{36})").expect("Bad mask regex");
            static ref write_regex: Regex =
                Regex::new(r"mem\[([0-9]+)\] = ([0-9]+)").expect("Bad write regex");
        }

        if s.starts_with("mask") {
            let mask = mask_regex.captures(s).unwrap().get(1).unwrap();

            let (mask_m, mask_actual) =
                mask.as_str()
                    .chars()
                    .enumerate()
                    .fold((0, 0), |acc, c| match c.1 {
                        'X' => (acc.0 + usize::pow(2, (36 - (c.0 + 1)) as u32), acc.1),
                        '1' => (acc.0, acc.1 + usize::pow(2, (36 - (c.0 + 1)) as u32)),
                        _ => acc,
                    });

            Ok(Self {
                instruction_type: InstructionType::Mask,
                addr: mask_m,
                val: mask_actual,
            })
        } else if s.starts_with("mem") {
            let caps = write_regex.captures(s).unwrap();
            Ok(Self {
                instruction_type: InstructionType::Write,
                addr: caps.get(1).unwrap().as_str().parse()?,
                val: caps.get(2).unwrap().as_str().parse()?,
            })
        } else {
            Err(anyhow!("Bad line: {}", s))
        }
    }
}

#[derive(Debug, Clone)]
pub struct VM {
    pub mask_actual: usize,
    pub mask_m: usize,
    pub memory: HashMap<usize, usize>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            mask_actual: 0,
            mask_m: 0,
            memory: HashMap::new(),
        }
    }
    pub fn step(&mut self, ins: &Instruction) {
        use InstructionType::*;
        match ins.instruction_type {
            Mask => {
                self.mask_actual = ins.val;
                self.mask_m = ins.addr;
            }
            Write => {
                // Calculate mask of mask i.e. 1s where X, 0 elsewhere and AND this with value
                // Calculate actual mask i.e. 0s where X, and OR this with above result
                let writeval = (ins.val & self.mask_m) | self.mask_actual;
                *self.memory.entry(ins.addr).or_insert(writeval) = writeval;
            }
        }
    }
    pub fn step2(&mut self, ins: &Instruction) {
        use InstructionType::*;
        match ins.instruction_type {
            Mask => {
                self.mask_actual = ins.val;
                self.mask_m = ins.addr;
            }
            Write => {
                let before_xs = (ins.addr | self.mask_actual) & !self.mask_m;
                // Work out number of Xs set (i.e. number of 1s in self.mask_m)
                // Iterate up to 2**n but setting bits in right places

                // All Xs as 0s:
                *self.memory.entry(before_xs).or_insert(ins.val) = ins.val;

                let mut powers = Vec::new();
                let mut lim = self.mask_m;
                let mut i = 0;

                while lim > 0 {
                    if lim % 2 != 0 {
                        powers.push(usize::pow(2, i));
                        for s in (1..=powers.len()) {
                            let it = powers.iter().combinations(s);
                            for x in it {
                                let mut index = 0;
                                for z in x {
                                    index = index | z;
                                }
                                *self.memory.entry(index | before_xs).or_insert(ins.val) = ins.val;
                            }
                        }
                    }
                    lim /= 2;
                    i += 1;
                }

                // Dumb brute force
                // for i in (0..self.mask_m) {
                //     *self
                //         .memory
                //         .entry((i & self.mask_m) | before_xs)
                //         .or_insert(ins.val) = ins.val;
                // }
            }
        }
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(|x| Instruction::from_str(x)).collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Instruction]) -> usize {
    let mut vm = VM::new();
    for ins in input {
        vm.step(ins);
        // println!("{:?}", vm);
    }
    vm.memory.values().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Instruction]) -> usize {
    let mut vm = VM::new();
    for ins in input {
        vm.step2(ins);
        // println!("{:?}", vm);
    }
    vm.memory.values().sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() -> Result<()> {
        let ex = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let input = input_generator(ex)?;
        assert_eq!(solve_part1(&input), 165);
        Ok(())
    }
    #[ignore]
    #[test]
    fn test2() -> Result<()> {
        let ex = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let input = input_generator(ex)?;
        assert_eq!(solve_part2(&input), 208);
        Ok(())
    }
    #[test]
    fn test2_2() -> Result<()> {
        let ex = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let input = input_generator(ex)?;
        assert_eq!(solve_part2(&input), 208);
        Ok(())
    }
}
