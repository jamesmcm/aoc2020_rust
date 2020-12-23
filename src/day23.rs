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
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect()
}

pub fn inner_solve(input: &[usize], num_iter: usize) -> Vec<usize> {
    let mut input = input.to_vec();
    let mut seen = HashSet::with_capacity(10000);
    input.reserve(1_000_000);
    let mut current_i: usize = 0;
    let maxval = *input.iter().max().unwrap();
    // println!("input: {:?}", input,);
    for i in 0..num_iter {
        if seen.contains(&input) {
            println!("Looped at index: {}", i);
        } else {
            seen.insert(input.clone());
        }
        let current = input[current_i];
        let mut dest = current.checked_sub(1).unwrap_or(maxval);
        let mut to_move: Vec<usize> = Vec::with_capacity(3);
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
        // println!("input: {:?}, to_move: {:?}, dest: {}", input, to_move, dest);
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
        // println!("end input: {:?}, current: {}", input, input[current_i]);
    }

    input
}

pub fn inner_solve2(start_current: usize, input: Vec<usize>, num_iter: usize) -> Vec<usize> {
    let mut input = input.to_vec();
    let mut current: usize = start_current;
    let maxval = *input.iter().max().unwrap();

    for _i in 0..num_iter {
        let n1 = input[current as usize];
        let n2 = input[n1 as usize];
        let n3 = input[n2 as usize];

        let mut dest = current;
        let dest = loop {
            dest = dest.checked_sub(1).unwrap_or(maxval);
            if dest != n1 && dest != n2 && dest != n3 {
                break dest;
            }
        };

        // println!(
        //     "Before turn {}: input: {:?}, current: {}, to_move: {:?}, dest: {}",
        //     _i,
        //     &input[0..20],
        //     current,
        //     (n1, n2, n3),
        //     dest
        // );

        input[current as usize] = input[n3 as usize];
        input[n3 as usize] = input[dest as usize];
        input[dest as usize] = n1;

        current = input[current];
        // println!(
        //     "After turn {}: input: {:?}, current: {}",
        //     _i,
        //     &input[0..20],
        //     current,
        // );
        // return vec![0];
    }
    input
}

pub fn part1(input: &[usize], num_iter: usize) -> String {
    let input = inner_solve(input, num_iter);
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

pub fn part1_new(input: &[usize], num_iter: usize) -> String {
    let mut new_input = vec![0; input.len()];
    let start_current = input[0] - 1;
    for (i, v) in input.iter().enumerate() {
        new_input[(v - 1) as usize] = input[(i + 1) % input.len()] - 1;
    }
    let input = inner_solve2(start_current, new_input, num_iter);

    let mut final_vec = Vec::with_capacity(input.len());
    let mut v = input[0];
    loop {
        final_vec.push(v + 1);
        v = input[v];
        if v == 0 {
            break;
        }
    }

    final_vec
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &[usize]) -> String {
    part1(input, 100)
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mut new_input = vec![0; 1_000_000];
    let start_current = input[0] - 1;
    for (i, v) in input.iter().enumerate() {
        new_input[(v - 1) as usize] = input[(i + 1) % input.len()] - 1;
    }
    // Unloop end to add extra
    new_input[input[input.len() - 1] - 1] = input.len();

    // 1_000_000
    for i in 9..1_000_000 {
        new_input[i] = ((i + 1) % 1_000_000) as usize;
    }

    new_input[1_000_000 - 1] = start_current;
    println!("{:?}", &new_input[0..20]);
    println!("{:?}", &new_input[999990..]);
    // 10_000_000
    let input = inner_solve2(start_current, new_input, 10_000_000);
    let first = input[0];
    let second = input[first];
    println!("{}, {}", first, second);

    (first + 1) * (second + 1)
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
    #[test]
    fn test2_0() {
        let ex = "389125467";
        let inp = input_generator(ex);
        assert_eq!(solve_part2(&inp), 149245887792);
    }
    #[test]
    fn test2_1() {
        let ex = "389125467";
        let inp = input_generator(ex);
        assert_eq!(part1_new(&inp, 1), "54673289");
    }
    #[test]
    fn test2_2() {
        let ex = "389125467";
        let inp = input_generator(ex);
        assert_eq!(part1_new(&inp, 10), "92658374");
    }
    #[test]
    fn test2_3() {
        let ex = "389125467";
        let inp = input_generator(ex);
        assert_eq!(part1_new(&inp, 100), "67384529");
    }
    #[test]
    fn test2_4() {
        let ex = "389125467";
        let inp = input_generator(ex);
        assert_eq!(part1_new(&inp, 0), "25467389");
    }
}
