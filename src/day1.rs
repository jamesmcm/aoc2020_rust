use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn two_sum(input: &[i32], target: i32) -> Result<(i32, i32)> {
    let mut hashset = HashSet::with_capacity(input.len());
    for &i in input {
        if hashset.contains(&i) {
            return Ok((i, target - i));
        }
        hashset.insert(target - i);
    }
    Err(anyhow!("No solution found!"))
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> Result<i32> {
    let (a, b) = two_sum(input, 2020)?;
    Ok(a * b)
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> Result<i32> {
    for (index, val) in input.iter().enumerate() {
        let res = two_sum(&input[index + 1..], 2020 - val);
        if let Ok(x) = res {
            return Ok(x.0 * x.1 * val);
        }
    }
    Err(anyhow!("No solution found!"))
}
