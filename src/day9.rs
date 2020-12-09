use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn two_sum(input: &[i64], target: i64) -> Result<(i64, i64)> {
    let mut hashset = HashSet::with_capacity(input.len());
    for &i in input {
        if hashset.contains(&i) {
            return Ok((i, target - i));
        }
        hashset.insert(target - i);
    }
    Err(anyhow!("No solution found!"))
}

fn solve_one(l: &[i64], preamble_len: usize) -> i64 {
    *l.windows(preamble_len + 1)
        .map(|w| {
            let last = w.last().expect("empty array");
            (last, two_sum(&w[0..w.len() - 1], *last))
        })
        .filter(|r| r.1.is_err())
        .map(|r| *r.0)
        .collect::<Vec<i64>>()
        .first()
        .expect("No candidate")
}

fn find_range(l: &[i64], target: i64) -> Result<&[i64]> {
    let mut start: usize = 0;
    let mut end: usize = 1;
    let mut sum: i64 = l[0] + l[1];

    loop {
        if sum < target {
            end += 1;
            if end >= l.len() {
                break Err(anyhow!("Target sum not found!"));
            }
            sum += l[end];
        } else if sum > target {
            sum -= l[start];
            start += 1;
        } else {
            break Ok(&l[start..=end]);
        }
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    solve_one(input, 25)
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[i64]) -> Result<i64> {
    let target = solve_one(input, 25);
    let range = find_range(input, target)?;
    Ok(range.iter().min().expect("no min") + range.iter().max().expect("no max"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let input = input_generator(input);
        assert_eq!(solve_one(&input, 5), 127);
    }
    #[test]
    fn test_two() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let input = input_generator(input);
        assert_eq!(solve_one(&input, 5), 127);
        assert_eq!(find_range(&input, 127).unwrap(), &[15, 25, 47, 40]);
    }
}
