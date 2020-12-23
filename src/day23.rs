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

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect()
}

pub fn part1(input: &[u64], num_iter: usize) -> String {
    let mut input = input.to_vec();
    let mut current_i: usize = 0;
    let maxval = *input.iter().max().unwrap();
    println!("input: {:?}", input,);
    for _i in 0..num_iter {
        let current = input[current_i];
        let mut dest = current.checked_sub(1).unwrap_or(maxval);
        let mut to_move: Vec<u64> = Vec::with_capacity(3);
        for _j in 1..=3 {
            current_i = input
                .iter()
                .enumerate()
                .find(|x| *x.1 == current)
                .map(|x| x.0)
                .expect("currrent not found");

            to_move.push(input.remove((current_i + 1) % input.len()));
        }
        dest = loop {
            if dest < 1 {
                dest = maxval;
            }
            if !input.contains(&dest) {
                dest -= 1;
            } else {
                break dest;
            }
        };
        println!("input: {:?}, to_move: {:?}, dest: {}", input, to_move, dest);
        let dest_index = input
            .iter()
            .enumerate()
            .find(|x| *x.1 == dest)
            .map(|x| x.0)
            .expect("dest not found");
        for _j in 1..=3 {
            input.insert(dest_index + 1, to_move.pop().unwrap());
        }

        let new_current_index = input
            .iter()
            .enumerate()
            .find(|x| *x.1 == current)
            .map(|x| x.0)
            .expect("currrent not found");

        current_i = (new_current_index + 1) % input.len();
        println!("end input: {:?}, current: {}", input, input[current_i]);
    }

    let ind_1 = input
        .iter()
        .enumerate()
        .find(|x| *x.1 == 1)
        .map(|x| x.0)
        .expect("1 not found");

    let mut outvec: Vec<String> = input[ind_1 + 1..].iter().map(|c| c.to_string()).collect();

    input[0..ind_1]
        .iter()
        .for_each(|c| outvec.push(c.to_string()));
    outvec.join("")
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &[u64]) -> String {
    part1(input, 100)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test1_0() {
        let ex = "389125467";
        let inp = input_generator(ex);
        assert_eq!(solve_part1(&inp), "67384529");
    }
    #[test]
    fn test1_1() {
        let ex = "389125467";
        let inp = input_generator(ex);
        assert_eq!(part1(&inp, 1), "54673289");
    }
    #[test]
    fn test1_2() {
        let ex = "389125467";
        let inp = input_generator(ex);
        assert_eq!(part1(&inp, 10), "92658374");
    }
}
