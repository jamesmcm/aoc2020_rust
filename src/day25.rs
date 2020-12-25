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

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> (u64, u64) {
    let mut l = input.lines();
    let first: u64 = l.next().unwrap().parse().unwrap();
    let second: u64 = l.next().unwrap().parse().unwrap();
    (first, second)
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &(u64, u64)) -> u64 {
    let l = find_loop_length(*input);
    println!("{:?}, {:?}", input, l);
    let key1 = calculate_key(l.0, input.1);
    let key2 = calculate_key(l.1, input.0);
    println!("{}, {}", key1, key2);
    key2
}

pub fn find_loop_length(pub_keys: (u64, u64)) -> (u64, u64) {
    // let mut seen: HashSet<u64> = HashSet::new();
    let mut l = 1;
    let mut val = 1;
    let subject = 7;

    let mut first_loop: Option<u64> = None;
    let mut second_loop: Option<u64> = None;

    loop {
        val *= subject;
        val %= 20201227;

        if val == pub_keys.0 {
            first_loop = Some(l);
        }
        if val == pub_keys.1 {
            second_loop = Some(l);
        }

        if first_loop.is_some() && second_loop.is_some() {
            break;
        }

        // if seen.contains(&val) {
        //     subject += 1;
        //     l = 1;
        //     val = 1;
        //     seen.clear();
        // }
        l += 1;
    }

    (first_loop.unwrap(), second_loop.unwrap())
}

pub fn calculate_key(l: u64, pub_key: u64) -> u64 {
    let mut val = 1;
    for _i in 0..l {
        val *= pub_key;
        val %= 20201227;
    }

    val
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test1_0() {
        let ex = "5764801
17807724";
        let inp = input_generator(ex);
        assert_eq!(solve_part1(&inp), 14897079);
    }
}
