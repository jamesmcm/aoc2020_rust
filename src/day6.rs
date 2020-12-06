use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|s| s.split('\n').map(|x| x.to_string()).collect())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Vec<String>]) -> usize {
    let mut sum = 0;
    for group in input {
        let mut set = HashSet::new();
        for person in group {
            person.chars().for_each(|c| {
                set.insert(c);
            });
        }
        sum += set.len();
    }
    sum
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Vec<String>]) -> usize {
    let mut sum = 0;
    for group in input {
        let mut map: HashMap<char, usize> = HashMap::new();
        let group_size = group.len();
        for person in group {
            person.chars().for_each(|c| {
                map.entry(c).and_modify(|x| *x += 1).or_insert(1);
            });
        }
        sum += map.values().filter(|v| **v == group_size).count();
    }
    sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let ex = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let input = input_generator(ex);
        assert_eq!(solve_part1(&input), 11);
    }
    #[test]
    fn test2() {
        let ex = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let input = input_generator(ex);
        assert_eq!(solve_part2(&input), 6);
    }
}
