use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Result<Vec<i64>> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse())
        .collect::<std::result::Result<Vec<i64>, core::num::ParseIntError>>()
        .map_err(|e| anyhow!("{:?}", e))
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let mut map: HashMap<i64, (usize, Option<usize>, usize)> = HashMap::with_capacity(256);
    let mut index: usize = 0;
    let mut prevnum = 0;
    for i in input {
        map.insert(*i, (1, None, index));
        prevnum = *i;
        index += 1;
        // println!("{}: {}", index, prevnum);
    }

    for j in input.len()..2020 {
        let prevnum_record = map.get(&prevnum).unwrap();
        let newnum = if prevnum_record.0 == 1 {
            0
        } else {
            (prevnum_record.2 - prevnum_record.1.unwrap()) as i64
        };

        map.entry(newnum)
            .and_modify(|v| *v = (v.0 + 1, Some(v.2), j))
            .or_insert((1, None, j));
        prevnum = newnum;
        // println!("{}: {}", j + 1, prevnum);
    }
    prevnum
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let mut map: HashMap<i64, (usize, Option<usize>, usize)> = HashMap::with_capacity(256);
    let mut index: usize = 0;
    let mut prevnum = 0;
    for i in input {
        map.insert(*i, (1, None, index));
        prevnum = *i;
        index += 1;
        // println!("{}: {}", index, prevnum);
    }

    for j in input.len()..30000000 {
        let prevnum_record = map.get(&prevnum).unwrap();
        let newnum = if prevnum_record.0 == 1 {
            0
        } else {
            (prevnum_record.2 - prevnum_record.1.unwrap()) as i64
        };

        map.entry(newnum)
            .and_modify(|v| *v = (v.0 + 1, Some(v.2), j))
            .or_insert((1, None, j));
        prevnum = newnum;
        // println!("{}: {}", j + 1, prevnum);
    }
    prevnum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1_1() -> Result<()> {
        let ex = "0,3,6";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 436);
        Ok(())
    }
    #[test]
    fn test1_2() -> Result<()> {
        let ex = "1,3,2";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 1);
        Ok(())
    }
    #[test]
    fn test1_3() -> Result<()> {
        let ex = "2,1,3";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 10);
        Ok(())
    }
    #[test]
    fn test1_4() -> Result<()> {
        let ex = "1,2,3";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 27);
        Ok(())
    }
    #[test]
    fn test1_5() -> Result<()> {
        let ex = "2,3,1";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 78);
        Ok(())
    }
    #[test]
    fn test1_6() -> Result<()> {
        let ex = "3,2,1";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 438);
        Ok(())
    }
    #[test]
    fn test1_7() -> Result<()> {
        let ex = "3,1,2";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 1836);
        Ok(())
    }
    #[test]
    fn test2_1() -> Result<()> {
        let ex = "0,3,6";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 175594);
        Ok(())
    }
    #[test]
    fn test2_2() -> Result<()> {
        let ex = "1,3,2";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 2578);
        Ok(())
    }
    #[test]
    fn test2_3() -> Result<()> {
        let ex = "2,1,3";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 3544142);
        Ok(())
    }
    #[test]
    fn test2_4() -> Result<()> {
        let ex = "1,2,3";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 261214);
        Ok(())
    }
    #[test]
    fn test2_5() -> Result<()> {
        let ex = "2,3,1";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 6895259);
        Ok(())
    }
    #[test]
    fn test2_6() -> Result<()> {
        let ex = "3,2,1";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 18);
        Ok(())
    }
    #[test]
    fn test2_7() -> Result<()> {
        let ex = "3,1,2";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 362);
        Ok(())
    }
}
