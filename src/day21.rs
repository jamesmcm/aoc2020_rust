use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::str::FromStr;
use std::string::ToString;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split('(');
            let ingredients = split
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(|x| x.to_string())
                .collect();
            let allergens = split
                .next()
                .unwrap()
                .strip_prefix("contains ")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(",")
                .map(|x| x.trim().to_string())
                .collect();
            Food {
                ingredients,
                allergens,
            }
        })
        .collect()
}

pub fn get_allergen_map(input: &[Food]) -> HashMap<String, HashSet<String>> {
    let mut allergen_map: HashMap<String, HashSet<String>> = HashMap::new();
    for food in input {
        for allergen in food.allergens.iter().cloned() {
            allergen_map
                .entry(allergen)
                .and_modify(|x| {
                    *x = x
                        .intersection(&HashSet::from_iter(food.ingredients.iter().cloned()))
                        .cloned()
                        .collect()
                })
                .or_insert(HashSet::from_iter(food.ingredients.iter().cloned()));
        }
    }
    allergen_map
}

pub fn reduce_allergen_map(map: HashMap<String, HashSet<String>>) -> HashMap<String, String> {
    let mut map = map.clone();
    let mut out_map: HashMap<String, String> = HashMap::new();
    while out_map.len() < map.len() {
        map.iter()
            .filter(|x| x.1.len() == 1)
            .map(|x| (x.0, x.1.into_iter().next().unwrap()))
            .for_each(|x| {
                out_map.insert(x.0.clone(), x.1.clone());
            });
        map.iter_mut().for_each(|x| {
            for val in out_map.values() {
                if x.1.contains(val) {
                    x.1.remove(val);
                }
            }
        });
        // println!("{:?}", out_map);
        // println!("{:?}", map);
    }
    out_map
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &[Food]) -> u32 {
    let mut sum = 0;
    let allergen_map = get_allergen_map(input);
    println!("{:?}", allergen_map);

    for food in input {
        for ingredient in &food.ingredients {
            if !allergen_map.values().any(|m| m.contains(ingredient)) {
                sum += 1;
            }
        }
    }
    sum
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &[Food]) -> String {
    let allergen_map = get_allergen_map(input);
    let reduce = reduce_allergen_map(allergen_map);
    println!("{:?}", reduce);

    let mut recvec: Vec<(String, String)> = reduce.into_iter().collect();
    recvec.sort_by_key(|x| x.0.clone());
    recvec
        .into_iter()
        .map(|x| x.1)
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test1_0() {
        let ex = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let inp = input_generator(ex);
        assert_eq!(solve_part1(&inp), 5);
    }
    #[test]
    fn test2_0() {
        let ex = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let inp = input_generator(ex);
        assert_eq!(solve_part2(&inp), "mxmxvkd,sqjhc,fvjkl".to_string());
    }
}
