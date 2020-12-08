use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum InstructionType {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub val: i32,
}

impl FromStr for InstructionType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Self::Nop),
            "acc" => Ok(Self::Acc),
            "jmp" => Ok(Self::Jmp),
            _ => Err(anyhow!("Unknown instruction type: {}", s)),
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let ins = InstructionType::from_str(split.next().ok_or(anyhow!("No instruction: {}", s))?)?;
        let val = split
            .next()
            .ok_or(anyhow!("No value part: {}", s))?
            .parse()?;
        Ok(Self {
            instruction_type: ins,
            val,
        })
    }
}

#[derive(Debug, Clone)]
pub struct VM {
    pub pc: usize,
    pub acc: i32,
    pub program: Vec<Instruction>,
}

impl VM {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            pc: 0,
            acc: 0,
            program,
        }
    }
    pub fn step(&mut self) {
        use InstructionType::*;
        let ins = &self.program[self.pc];
        match ins.instruction_type {
            Acc => {
                self.acc += ins.val;
                self.pc += 1;
            }
            Jmp => {
                self.pc = (self.pc as i32 + ins.val) as usize;
            }
            Nop => {
                self.pc += 1;
            }
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(|x| Instruction::from_str(x)).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Instruction]) -> i32 {
    let mut vm = VM::new(input.to_vec());
    let mut seen = HashSet::new();
    loop {
        // println!("{:?}", vm.pc);
        if seen.contains(&vm.pc) {
            break vm.acc;
        }
        seen.insert(vm.pc);
        vm.step();
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Instruction]) -> Result<i32> {
    let mut vm = VM::new(input.to_vec());
    let program_length = vm.program.len();
    let mut seen = HashSet::new();
    loop {
        if vm.pc == program_length {
            break Ok(vm.acc);
        }
        if seen.contains(&vm.pc) {
            break Err(anyhow!("Infinite loop, pc: {}, acc: {}", vm.pc, vm.acc));
        }
        use InstructionType::*;
        match vm.program[vm.pc].instruction_type {
            Acc => {}
            Nop => {
                let mut newvm = vm.clone();
                newvm.program[newvm.pc].instruction_type = Jmp;
                let out = run_vm(newvm, seen.clone());
                if let Ok(newacc) = out {
                    break Ok(newacc);
                }
            }
            Jmp => {
                let mut newvm = vm.clone();
                newvm.program[newvm.pc].instruction_type = Nop;
                let out = run_vm(newvm, seen.clone());
                if let Ok(newacc) = out {
                    break Ok(newacc);
                }
            }
        }

        seen.insert(vm.pc);
        vm.step();
    }
}

// Accumulator never used itself in jmp, so if we ever see previous instruction
// Then we have an infinite loop
fn run_vm(mut vm: VM, mut seen: HashSet<usize>) -> Result<i32> {
    let program_length = vm.program.len();
    loop {
        if vm.pc == program_length {
            break Ok(vm.acc);
        }
        // println!("{:?}", vm.pc);
        if seen.contains(&vm.pc) {
            break Err(anyhow!("Infinite loop, pc: {}, acc: {}", vm.pc, vm.acc));
        }
        seen.insert(vm.pc);
        vm.step();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() -> Result<()> {
        let ex = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let input = input_generator(ex)?;
        assert_eq!(solve_part1(&input), 5);
        Ok(())
    }
    #[test]
    fn test2() -> Result<()> {
        let ex = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let input = input_generator(ex)?;
        assert_eq!(solve_part2(&input)?, 8);
        Ok(())
    }
}
