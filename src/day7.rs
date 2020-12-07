use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<(u32, String)>> {
    input.lines().map(|l| parse_line(l)).collect()
}

fn parse_line(s: &str) -> (String, Vec<(u32, String)>) {
    lazy_static! {
        static ref subject_re: Regex = Regex::new(
            // r"([a-z]+ [a-z]+) bags contain (([0-9]+) ([a-z]+ [a-z]+) bags(\.|,\s))+|(no other bags)"
            r"([a-z]+ [a-z]+) bags contain"
        )
        .expect("Bad regex");

        static ref edges_re: Regex = Regex::new(
            // r"([a-z]+ [a-z]+) bags contain (([0-9]+) ([a-z]+ [a-z]+) bags(\.|,\s))+|(no other bags)"
            r"([0-9]+) ([a-z]+ [a-z]+) bag"
        )
        .expect("Bad regex");
    }

    let subject_name = subject_re
        .captures(s)
        .expect("No subject phrase")
        .get(1)
        .expect("No subject name")
        .as_str()
        .to_string();

    let edges = edges_re
        .captures_iter(s)
        .map(|x| (x[1].parse().expect("Bad int"), x[2].to_string()))
        .collect();

    (subject_name, edges)
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &HashMap<String, Vec<(u32, String)>>) -> usize {
    let mut to_check = Vec::new();
    let mut seen = HashSet::new();
    let mut sum = 0;
    for (key, val) in input.iter() {
        if val.iter().any(|x| x.1 == "shiny gold") {
            if !to_check.contains(key) {
                to_check.push(key.clone());
                seen.insert(key.clone());
                sum += 1;
            }
        }
    }

    while to_check.len() > 0 {
        let target = to_check.pop().expect("empty vec");

        for (key, val) in input.iter() {
            // println!("{}: {}, {:?} - {:?}", target, key, val, seen);
            if val.iter().any(|x| x.1 == target) {
                if (!to_check.contains(key)) && (!seen.contains(key)) {
                    to_check.push(key.clone());
                    seen.insert(key.clone());
                    sum += 1;
                }
            }
        }
    }

    sum
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    // DFS from shiny gold
    let mut to_check = Vec::new();
    // let mut seen = HashSet::new();
    let mut sum = 0;

    let start = input.get("shiny gold").expect("No shiny gold");
    let i = 1;
    start.iter().for_each(|x| {
        sum += i * x.0;
        to_check.push((x.0, x.1.clone()));
    });
    // seen.insert("shiny gold".to_string());

    // Iterative DFS - avoid stack overflow
    while to_check.len() > 0 {
        let cur = to_check.pop().expect("Empty vec");
        input.get(&cur.1).expect("Bad Key").iter().for_each(|x| {
            sum += x.0 * cur.0;
            to_check.push((x.0 * cur.0, x.1.clone()));
        });
    }

    sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse() {
        let ex = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let input = parse_line(ex);
        assert_eq!(
            input,
            (
                "light red".to_string(),
                vec![
                    (1, "bright white".to_string()),
                    (2, "muted yellow".to_string())
                ]
            )
        );
    }
    #[test]
    fn test_parse_empty() {
        let ex = "faded blue bags contain no other bags.";
        let input = parse_line(ex);
        assert_eq!(input, ("faded blue".to_string(), vec![]));
    }
    #[test]
    fn test1() {
        let ex = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let input = input_generator(ex);
        let sol = solve_part1(&input);
        assert_eq!(sol, 4);
    }
    #[test]
    fn test2() {
        let ex = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let input = input_generator(ex);
        let sol = solve_part2(&input);
        assert_eq!(sol, 32);
    }
    #[test]
    fn test3() {
        let ex = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let input = input_generator(ex);
        let sol = solve_part2(&input);
        assert_eq!(sol, 126);
    }
}
