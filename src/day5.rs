use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryFrom;

pub enum RowDivision {
    Back,
    Front,
}

pub enum SeatDivision {
    Right,
    Left,
}

impl TryFrom<char> for RowDivision {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'F' => Ok(Self::Front),
            'B' => Ok(Self::Back),
            _ => Err(anyhow!("Bad row char: {}", c)),
        }
    }
}

impl TryFrom<char> for SeatDivision {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'R' => Ok(Self::Right),
            'L' => Ok(Self::Left),
            _ => Err(anyhow!("Bad seat char: {}", c)),
        }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Result<Vec<(Vec<RowDivision>, Vec<SeatDivision>)>> {
    input
        .lines()
        .map(|l| {
            let (rowpart, seatpart) = l.split_at(7);
            let rowvec = rowpart
                .chars()
                .map(|c| RowDivision::try_from(c))
                .collect::<Result<Vec<RowDivision>>>();
            let seatvec = seatpart
                .chars()
                .map(|c| SeatDivision::try_from(c))
                .collect::<Result<Vec<SeatDivision>>>();
            Ok((rowvec?, seatvec?))
        })
        .collect()
}

fn get_seat_id(inp: &(Vec<RowDivision>, Vec<SeatDivision>)) -> u32 {
    let mut start = 0;
    let mut end = 127;

    for r in inp.0.iter() {
        match r {
            RowDivision::Back => {
                start += ((end + 1) - start) / 2;
            }
            RowDivision::Front => {
                end -= (end - start) / 2;
            }
        }
    }

    let row = start;
    start = 0;
    end = 7;
    for r in inp.1.iter() {
        match r {
            SeatDivision::Right => {
                start += ((end + 1) - start) / 2;
            }
            SeatDivision::Left => {
                end -= (end - start) / 2;
            }
        }
        // println!("seat start: {}, end: {}", start, end);
    }
    let seat = start;

    // println!("Row: {}, Seat: {}", row, seat);
    (row * 8) + seat
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[(Vec<RowDivision>, Vec<SeatDivision>)]) -> Result<u32> {
    input
        .iter()
        .map(|x| get_seat_id(x))
        .max()
        .ok_or_else(|| anyhow!("No maximum?"))
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[(Vec<RowDivision>, Vec<SeatDivision>)]) -> Result<u32> {
    let mut seat_ids: Vec<u32> = input.iter().map(|x| get_seat_id(x)).collect();
    seat_ids.sort_unstable();
    let out = seat_ids.iter().try_fold(None, |acc, x| {
        if let Some(z) = acc {
            if x - z > 1 {
                Err(z + 1)
            } else {
                Ok(Some(x))
            }
        } else {
            Ok(Some(x))
        }
    });

    if let Err(x) = out {
        Ok(x)
    } else {
        Err(anyhow!("No missing val found!"))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() -> Result<()> {
        let ex = "FBFBBFFRLR";
        let input = input_generator(ex)?;
        assert_eq!(get_seat_id(&input[0]), 357);
        Ok(())
    }
    #[test]
    fn test2() -> Result<()> {
        let ex = "BFFFBBFRRR";
        let input = input_generator(ex)?;
        assert_eq!(get_seat_id(&input[0]), 567);
        Ok(())
    }
    #[test]
    fn test3() -> Result<()> {
        let ex = "FFFBBBFRRR";
        let input = input_generator(ex)?;
        assert_eq!(get_seat_id(&input[0]), 119);
        Ok(())
    }
    #[test]
    fn test4() -> Result<()> {
        let ex = "BBFFBBFRLL";
        let input = input_generator(ex)?;
        assert_eq!(get_seat_id(&input[0]), 820);
        Ok(())
    }
}
