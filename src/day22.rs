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

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Result<(Vec<i64>, Vec<i64>)> {
    let mut iter = input.split("\n\n").map(|p| {
        let mut lines = p.lines();
        lines.next();
        lines
            .map(|x| x.parse::<i64>().map_err(|e| anyhow!("{:?}", e)))
            .collect::<Result<Vec<i64>>>()
    });
    Ok((iter.next().unwrap()?, iter.next().unwrap()?))
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    P1,
    P2,
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let mut p1stack = VecDeque::from(input.0.clone());
    let mut p2stack = VecDeque::from(input.1.clone());

    println!("p1: {:?}, p2: {:?}", p1stack, p2stack);
    let win_stack = loop {
        let p1card = p1stack.pop_front().expect("p1 no cards");
        let p2card = p2stack.pop_front().expect("p2 no cards");

        if p1card >= p2card {
            p1stack.push_back(p1card);
            p1stack.push_back(p2card);
        } else {
            p2stack.push_back(p2card);
            p2stack.push_back(p1card);
        }
        if p2stack.is_empty() {
            break p1stack;
        }
        if p1stack.is_empty() {
            break p2stack;
        }
    };

    win_stack.iter().enumerate().fold(0, |acc, x| {
        acc + x.1 * (win_stack.len() as i64 - x.0 as i64)
    })
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let mut p1stack = VecDeque::from(input.0.clone());
    let mut p2stack = VecDeque::from(input.1.clone());

    let mut seen = HashSet::new();
    let win_stack = loop {
        let p1vec = p1stack.iter().cloned().collect::<Vec<i64>>();
        let p2vec = p2stack.iter().cloned().collect::<Vec<i64>>();

        if seen.contains(&(p1vec.clone(), p2vec.clone())) {
            break p1stack;
        }
        seen.insert((p1vec, p2vec));
        let p1card = p1stack.pop_front().expect("p1 no cards");
        let p2card = p2stack.pop_front().expect("p2 no cards");

        let round_winner = if p1stack.len() >= p1card as usize && p2stack.len() >= p2card as usize {
            recurse_game(
                p1stack
                    .iter()
                    .cloned()
                    .take(p1card as usize)
                    .collect::<VecDeque<i64>>(),
                p2stack
                    .iter()
                    .cloned()
                    .take(p2card as usize)
                    .collect::<VecDeque<i64>>(),
            )
        } else if p1card >= p2card {
            Player::P1
        } else {
            Player::P2
        };

        match round_winner {
            Player::P1 => {
                p1stack.push_back(p1card);
                p1stack.push_back(p2card);
            }
            Player::P2 => {
                p2stack.push_back(p2card);
                p2stack.push_back(p1card);
            }
        }

        if p2stack.is_empty() {
            break p1stack;
        }
        if p1stack.is_empty() {
            break p2stack;
        }
    };

    win_stack.iter().enumerate().fold(0, |acc, x| {
        acc + x.1 * (win_stack.len() as i64 - x.0 as i64)
    })
}

pub fn recurse_game(mut p1stack: VecDeque<i64>, mut p2stack: VecDeque<i64>) -> Player {
    let p1vec = p1stack.iter().cloned().collect::<Vec<i64>>();
    let p2vec = p2stack.iter().cloned().collect::<Vec<i64>>();
    // println!("p1r: {:?}, p2r: {:?}", p1vec, p2vec);
    let mut seen = HashSet::new();

    loop {
        let p1vec = p1stack.iter().cloned().collect::<Vec<i64>>();
        let p2vec = p2stack.iter().cloned().collect::<Vec<i64>>();

        if seen.contains(&(p1vec.clone(), p2vec.clone())) {
            return Player::P1;
        }
        seen.insert((p1vec, p2vec));
        let p1card = p1stack.pop_front().expect("p1 no cards");
        let p2card = p2stack.pop_front().expect("p2 no cards");

        let round_winner = if p1stack.len() >= p1card as usize && p2stack.len() >= p2card as usize {
            recurse_game(
                p1stack
                    .iter()
                    .cloned()
                    .take(p1card as usize)
                    .collect::<VecDeque<i64>>(),
                p2stack
                    .iter()
                    .cloned()
                    .take(p2card as usize)
                    .collect::<VecDeque<i64>>(),
            )
        } else if p1card >= p2card {
            Player::P1
        } else {
            Player::P2
        };

        match round_winner {
            Player::P1 => {
                p1stack.push_back(p1card);
                p1stack.push_back(p2card);
            }
            Player::P2 => {
                p2stack.push_back(p2card);
                p2stack.push_back(p1card);
            }
        }

        if p2stack.is_empty() {
            break Player::P1;
        }
        if p1stack.is_empty() {
            break Player::P2;
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test1_0() -> Result<()> {
        let ex = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 306);
        Ok(())
    }
    #[test]
    fn test2_0() -> Result<()> {
        let ex = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 291);
        Ok(())
    }
    #[test]
    fn test2_loop() -> Result<()> {
        let ex = "Player 1:
43
19

Player 2:
2
29
14";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 105);
        Ok(())
    }
}
