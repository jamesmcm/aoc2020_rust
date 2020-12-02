use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Policy {
    pub character: char,
    pub min: u32,
    pub max: u32,
}

impl FromStr for Policy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut siter = s.split(' ');
        let range_str = siter
            .next()
            .ok_or_else(|| anyhow!("No space in policy string"))?;
        let mut range_split = range_str.split('-');
        let min: u32 = range_split
            .next()
            .ok_or_else(|| anyhow!("No - in range"))?
            .parse()?;
        let max: u32 = range_split
            .next()
            .ok_or_else(|| anyhow!("No - in range"))?
            .parse()?;
        let character = siter
            .next()
            .ok_or_else(|| anyhow!("No char"))?
            .chars()
            .next()
            .ok_or_else(|| anyhow!("No char"))?;
        // let s = siter.next().ok_or_else(|| anyhow!("No string part"))?.trim().to_string();

        Ok(Self {
            character,
            min,
            max,
        })
    }
}

fn split_input(l: &str) -> Result<(Policy, String)> {
    let mut split = l.split(':');
    let first_part = split
        .next()
        .ok_or_else(|| anyhow!("Failed to split input on :"))?;
    let policy = Policy::from_str(first_part)?;
    let s = split
        .next()
        .ok_or_else(|| anyhow!("No string part"))?
        .trim()
        .to_string();
    Ok((policy, s))
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<(Policy, String)>> {
    input.lines().map(|l| split_input(l)).collect()
}

fn get_char_count(s: &str) -> HashMap<char, u32> {
    s.chars().fold(HashMap::with_capacity(26), |mut acc, x| {
        acc.entry(x).and_modify(|i| *i += 1).or_insert(1);
        acc
    })
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(Policy, String)]) -> u32 {
    input.iter().fold(0, |acc, x| {
        let hm = get_char_count(&x.1);
        if let Some(y) = hm.get_key_value(&x.0.character) {
            if y.1 >= &x.0.min && y.1 <= &x.0.max {
                return acc + 1;
            }
        }
        acc
    })
}

fn part2_valid(s: &str, policy: &Policy) -> bool {
    let mut chars_iter = s.chars();
    let first_char = chars_iter.nth((policy.min - 1) as usize);
    let second_char = chars_iter.nth((policy.max - policy.min - 1) as usize);
    if let Some(fc) = first_char {
        if let Some(sc) = second_char {
            if (fc == policy.character && sc != policy.character)
                || (sc == policy.character && fc != policy.character)
            {
                // println!("{:?}, {:?}, {:?}, {:?}", s, policy, fc, sc);
                return true;
            }
        }
    }
    false
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(Policy, String)]) -> u32 {
    input.iter().fold(0, |acc, x| {
        if part2_valid(x.1.as_str(), &x.0) {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() -> Result<()> {
        let ex = "1-3 a: abcde";
        let (policy, s) = split_input(ex)?;
        assert!(part2_valid(&s, &policy));
        Ok(())
    }
    #[test]
    fn test2() -> Result<()> {
        let ex = "1-3 b: cdefg";
        let (policy, s) = split_input(ex)?;
        assert!(!part2_valid(&s, &policy));
        Ok(())
    }
    #[test]
    fn test3() -> Result<()> {
        let ex = "2-9 c: ccccccccc";
        let (policy, s) = split_input(ex)?;
        assert!(!part2_valid(&s, &policy));
        Ok(())
    }
}
