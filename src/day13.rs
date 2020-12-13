use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use num::integer::lcm;
use std::collections::HashMap;

pub struct Input {
    pub time: u64,
    pub schedules: Vec<(usize, u64)>,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Result<Input> {
    let mut lines = input.lines();
    let time: u64 = lines.next().ok_or(anyhow!("No line"))?.parse()?;
    let schedules: Vec<(usize, u64)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|x| x.1 != "x")
        .map(|x| (x.0, x.1.parse().unwrap()))
        .collect();

    Ok(Input { time, schedules })
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &Input) -> u64 {
    let wait = input
        .schedules
        .iter()
        .map(|x| {
            let rem = input.time.rem_euclid(x.1);
            let wait = if rem == 0 { 0 } else { x.1 - rem };

            (x.1, wait)
        })
        .min_by_key(|x| x.1)
        .unwrap();

    wait.0 * wait.1
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Input) -> u64 {
    let base_cycle = input.schedules[0].1;
    let maxmod = input.schedules.iter().fold(1, |acc, x| acc * x.1) as usize;
    let mut cycles = Vec::new();
    for s in input.schedules[1..].iter() {
        cycles.push((
            s.1,
            (modulo_inverse(base_cycle, s.1) * (s.1 - s.0 as u64)).rem_euclid(s.1),
        ));
    }
    println!("{:?}", cycles);
    let hms: Vec<std::collections::HashSet<_>> = cycles
        .iter()
        .map(|c| {
            let mut hm = std::collections::HashSet::with_capacity(1000);
            for i in 1..100000.min(maxmod) {
                hm.insert(base_cycle * (c.1 + (i as u64 * c.0 as u64)));
            }
            hm
        })
        .collect();
    let mut set = hms[0].clone();

    for hs in hms.into_iter() {
        set = set.intersection(&hs).map(|x| *x).collect();
    }

    *set.iter().min().expect("Set fail")
}

fn brute_force(input: &Input) -> u64 {
    let mut i: u64 = 0;
    loop {
        if input
            .schedules
            .iter()
            .all(|x| (i + x.0 as u64).rem_euclid(x.1 as u64) == 0)
        {
            break i;
        }
        i += input.schedules[0].1 as u64;
        if i == u64::MAX {
            break 0;
        }
    }
}

fn modulo_inverse(x: u64, p: u64) -> u64 {
    let mut out: Option<u64> = None;
    for i in 1..p {
        if (x * i).rem_euclid(p) == 1 {
            out = Some(i);
        }
    }
    out.expect("No inverse!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() -> Result<()> {
        let input = "939
7,13,x,x,59,x,31,19";
        let input = input_generator(input)?;
        assert_eq!(solve_part1(&input), 295);
        Ok(())
    }
    #[test]
    fn test_two_1() -> Result<()> {
        let input = "939
17,x,13,19";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 3417);
        Ok(())
    }
    #[test]
    fn test_two_2() -> Result<()> {
        let input = "939
67,7,59,61";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 754018);
        Ok(())
    }
    #[test]
    fn test_two_3() -> Result<()> {
        let input = "939
67,x,7,59,61";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 779210);
        Ok(())
    }
    #[test]
    fn test_two_4() -> Result<()> {
        let input = "939
67,7,x,59,61";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 1261476);
        Ok(())
    }
    #[test]
    fn test_two_5() -> Result<()> {
        let input = "939
1789,37,47,1889";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 1202161486);
        Ok(())
    }
    #[test]
    fn test_mine() -> Result<()> {
        let input = "939
17,x,13";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 102);
        Ok(())
    }
    #[test]
    fn test_small_full() -> Result<()> {
        let input = "939
3,5,7";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 54);
        Ok(())
    }
    #[test]
    fn test_small_end() -> Result<()> {
        let input = "939
3,x,7";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 12);
        Ok(())
    }
    #[test]
    fn test_small_start() -> Result<()> {
        let input = "939
3,5";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 9);
        Ok(())
    }
    #[test]
    fn test_tiny_full() -> Result<()> {
        let input = "939
2,3,7";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 26);
        Ok(())
    }
    #[test]
    fn test_tiny_end() -> Result<()> {
        let input = "939
2,x,7";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 12);
        Ok(())
    }
    #[test]
    fn test_tiny_start() -> Result<()> {
        let input = "939
2,3";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 2);
        Ok(())
    }

    #[test]
    fn test_inv() {
        assert_eq!(modulo_inverse(4, 13), 10);
    }
    #[test]
    fn test_mine3() -> Result<()> {
        let input = "939
17,x,x,19";
        let input = input_generator(input)?;
        assert_eq!(solve_part2(&input), 187);
        Ok(())
    }
}
