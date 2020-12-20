use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::str::FromStr;
use std::string::ToString;
use std::sync::Mutex;

lazy_static! {
    static ref CACHE: Mutex<HashMap<(u32, Vec<Transform>), Tile>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Pixel {
    On,
    Off,
}

impl TryFrom<char> for Pixel {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Self::On),
            '.' => Ok(Self::Off),
            _ => Err(anyhow!("Invalid char: {}", c)),
        }
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::On => "#",
                Pixel::Off => ".",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub array: Vec<Vec<Pixel>>,
    pub length: usize,
    pub id: u32,
    pub transforms: Vec<Transform>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Border {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Transform {
    FlipHorizontal,
    // FlipVertical,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl Tile {
    pub fn new(array: Vec<Vec<Pixel>>, id: u32) -> Self {
        let length = array[0].len();
        Tile {
            array,
            length,
            id,
            transforms: Vec::new(),
        }
    }

    pub fn rotate_90(&self) -> Self {
        let mut trans = self.transforms.clone();
        trans.push(Transform::Rotate90);
        let mut lock = CACHE.lock().unwrap();
        if let Some(res) = (*lock).get(&(self.id, trans)) {
            return res.clone();
        }

        let mut newtile = self.clone();
        let len = newtile.array[0].len();
        for i in 0..len {
            for j in 0..len {
                newtile.array[i][j] = self.array[len - j - 1][i];
            }
        }
        newtile.transforms.push(Transform::Rotate90);
        (*lock).insert((self.id, newtile.transforms.clone()), newtile.clone());
        newtile
    }
    pub fn rotate_180(&self) -> Self {
        let mut trans = self.transforms.clone();
        trans.push(Transform::Rotate180);
        let mut lock = CACHE.lock().unwrap();
        if let Some(res) = (*lock).get(&(self.id, trans)) {
            return res.clone();
        }

        let mut newtile = self.clone();
        let len = newtile.array[0].len();
        for i in 0..len {
            for j in 0..len {
                newtile.array[i][j] = self.array[len - j - 1][len - i - 1];
            }
        }
        newtile.transforms.push(Transform::Rotate180);
        (*lock).insert((self.id, newtile.transforms.clone()), newtile.clone());
        newtile
    }
    pub fn rotate_270(&self) -> Self {
        let mut trans = self.transforms.clone();
        trans.push(Transform::Rotate270);
        let mut lock = CACHE.lock().unwrap();
        if let Some(res) = (*lock).get(&(self.id, trans)) {
            return res.clone();
        }

        let mut newtile = self.clone();
        let len = newtile.array[0].len();
        for i in 0..len {
            for j in 0..len {
                newtile.array[i][j] = self.array[j][len - i - 1];
            }
        }
        newtile.transforms.push(Transform::Rotate270);
        (*lock).insert((self.id, newtile.transforms.clone()), newtile.clone());
        newtile
    }
    pub fn flip_horizontal(&self) -> Self {
        let mut trans = self.transforms.clone();
        trans.push(Transform::FlipHorizontal);
        let mut lock = CACHE.lock().unwrap();
        if let Some(res) = (*lock).get(&(self.id, trans)) {
            return res.clone();
        }

        let mut newtile = self.clone();
        let len = newtile.array[0].len();
        for i in 0..len {
            for j in 0..len {
                newtile.array[i][j] = self.array[i][len - j - 1];
            }
        }
        newtile.transforms.push(Transform::FlipHorizontal);
        (*lock).insert((self.id, newtile.transforms.clone()), newtile.clone());
        newtile
    }
    // pub fn flip_vertical(&self) -> Self {
    //     let mut trans = self.transforms.clone();
    //     trans.push(Transform::FlipVertical);
    //     let mut lock = CACHE.lock().unwrap();
    //     if let Some(res) = (*lock).get(&(self.id, trans)) {
    //         return res.clone();
    //     }
    //     let mut newtile = self.clone();
    //     let len = newtile.array[0].len();
    //     for i in 0..len {
    //         newtile.array[i] = self.array[len - i - 1].clone();
    //     }
    //     newtile.transforms.push(Transform::FlipVertical);
    //     (*lock).insert((self.id, newtile.transforms.clone()), newtile.clone());
    //     newtile
    // }

    pub fn get_border(&self, side: Border) -> Vec<Pixel> {
        use Border::*;
        match side {
            Top => self.array[0].clone(),
            Bottom => self.array[self.length - 1].clone(),
            Left => self.array.iter().cloned().map(|r| r[0]).collect(),
            Right => self
                .array
                .iter()
                .cloned()
                .map(|r| r[self.length - 1])
                .collect(),
        }
    }
}
impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut linesplit = s.lines();
        let tilenum: u32 =
            linesplit.next().unwrap().trim().strip_suffix(':').unwrap()[5..].parse()?;
        let s = &linesplit.collect::<Vec<&str>>().join("\n");

        let array = s
            .lines()
            .map(|x| x.chars().map(|c| Pixel::try_from(c)).collect())
            .collect::<Result<Vec<Vec<Pixel>>>>()?;
        Ok(Tile::new(array, tilenum))
    }
}
#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Result<Vec<Tile>> {
    input.split("\n\n").map(|s| Tile::from_str(s)).collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AllTransforms {
    Identity,
    FH,
    R90,
    R180,
    R270,
    R90FH,
    R180FH,
    R270FH,
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &[Tile]) -> u64 {
    // TODO:
    // Generate all 8 Tile possibilities first identified by enum
    // Store in (id, transform_enum) HashMap
    // Also build (border, bordertype) -> (id, transform_enum) map - are they unique?
    // Can then get next candidate by checking if there is valid border to the right/below
    // Remaining Available Vec should be vec of Ids, then loop over transform enum to get all
    // possibilities
    // Candidate is (id, transform_enum) tuple
    // Placed Vec should be Vec of (id, transform_enum)s
    // Avoid clones
    let num_tiles = (input.len() as f64).sqrt() as usize;
    println!("Num tiles: {}, sqrt: {}", input.len(), num_tiles);
    let placed = vec![None; num_tiles * num_tiles];
    let pc = step_solve(placed, input.to_vec(), 0, None, num_tiles);
    let placed = pc.unwrap().0;
    let ids: Vec<Option<u32>> = placed.iter().map(|x| x.as_ref().map(|y| y.id)).collect();
    println!("{:?}", ids);

    let mut i = 0;
    while i < placed.len() {
        for r in 0..placed[0].as_ref().unwrap().array.len() {
            let mut v = Vec::new();
            for j in 0..num_tiles {
                v.push(
                    placed[i + j].as_ref().unwrap().array[r]
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(""),
                );
            }
            println!("{}", v.join(" "));
        }
        i += num_tiles;
        println!("\n");
    }

    placed[0].as_ref().unwrap().id as u64
        * placed[num_tiles - 1].as_ref().unwrap().id as u64
        * placed[num_tiles * num_tiles - num_tiles]
            .as_ref()
            .unwrap()
            .id as u64
        * placed[num_tiles * num_tiles - 1].as_ref().unwrap().id as u64
}

pub fn step_solve(
    mut placed: Vec<Option<Tile>>,
    available: Vec<Tile>,
    mut target_pos: usize,
    candidate: Option<Tile>,
    num_tiles: usize,
) -> Result<(Vec<Option<Tile>>, Vec<Tile>)> {
    if candidate.is_some() {
        if target_pos > ((num_tiles * num_tiles) - 1) {
            return Ok((placed, available));
        }
        placed[target_pos] = candidate;

        if let Err(e) = valid_state(&placed, num_tiles, target_pos) {
            return Err(e);
        }

        if available.is_empty() {
            println!(
                "Placed: {:?}, target_pos: {}",
                placed
                    .iter()
                    .map(|x| x.as_ref().map(|y| (y.id, y.transforms.clone())))
                    .collect::<Vec<Option<(u32, Vec<Transform>)>>>(),
                target_pos
            );
            return Ok((placed, available));
        }
        target_pos += 1;
    }

    for (i, candidate) in available.iter().enumerate() {
        let mut new_available = available.clone();
        new_available.remove(i);
        let pc = step_solve(
            placed.clone(),
            new_available.clone(),
            target_pos,
            Some(candidate.clone()),
            num_tiles,
        );
        if pc.is_ok() {
            return pc;
        }
        let pc = step_solve(
            placed.clone(),
            new_available.clone(),
            target_pos,
            Some(candidate.rotate_90()),
            num_tiles,
        );
        if pc.is_ok() {
            return pc;
        }
        let pc = step_solve(
            placed.clone(),
            new_available.clone(),
            target_pos,
            Some(candidate.rotate_180()),
            num_tiles,
        );
        if pc.is_ok() {
            return pc;
        }
        let pc = step_solve(
            placed.clone(),
            new_available.clone(),
            target_pos,
            Some(candidate.rotate_270()),
            num_tiles,
        );
        if pc.is_ok() {
            return pc;
        }
        let pc = step_solve(
            placed.clone(),
            new_available.clone(),
            target_pos,
            Some(candidate.flip_horizontal()),
            num_tiles,
        );
        if pc.is_ok() {
            return pc;
        }
        let pc = step_solve(
            placed.clone(),
            new_available.clone(),
            target_pos,
            Some(candidate.rotate_90().flip_horizontal()),
            num_tiles,
        );
        if pc.is_ok() {
            return pc;
        }
        let pc = step_solve(
            placed.clone(),
            new_available.clone(),
            target_pos,
            Some(candidate.rotate_180().flip_horizontal()),
            num_tiles,
        );
        if pc.is_ok() {
            return pc;
        }
        let pc = step_solve(
            placed.clone(),
            new_available.clone(),
            target_pos,
            Some(candidate.rotate_270().flip_horizontal()),
            num_tiles,
        );
        if pc.is_ok() {
            return pc;
        }
    }
    Err(anyhow!("No valid follow-up states"))
}

pub fn valid_state(placed: &[Option<Tile>], num_tiles: usize, target_pos: usize) -> Result<()> {
    use Border::*;
    let i = target_pos;
    let opt = &placed[i];
    if let Some(t) = opt {
        let index: i64 = (i as i64) - num_tiles as i64;
        if index >= 0 {
            if let Some(at) = placed.get(index as usize).map(|x| x.as_ref()).flatten() {
                if at.get_border(Bottom) != t.get_border(Top) {
                    return Err(anyhow!(
                        "Above check failed: {:?}, {:?}",
                        (t.id, i),
                        (index, at.id)
                    ));
                }
            }
        }
        // Below
        let index: i64 = (i as i64) + num_tiles as i64;
        if index >= 0 {
            if let Some(at) = placed.get(index as usize).map(|x| x.as_ref()).flatten() {
                if at.get_border(Top) != t.get_border(Bottom) {
                    return Err(anyhow!(
                        "Below check failed: {:?}, {:?}",
                        (t.id, i),
                        (index, at.id)
                    ));
                }
            }
        }
        // Left
        let index: i64 = (i as i64) - 1;
        if index >= 0 && (i % num_tiles) != 0 {
            if let Some(at) = placed.get(index as usize).map(|x| x.as_ref()).flatten() {
                if at.get_border(Right) != t.get_border(Left) {
                    return Err(anyhow!(
                        "Left check failed: {:?}, {:?}",
                        (t.id, i),
                        (index, at.id)
                    ));
                }
            }
        }
        // Right
        let index: i64 = (i as i64) + 1;
        if index >= 0 && (i % num_tiles) != (num_tiles - 1) {
            if let Some(at) = placed.get(index as usize).map(|x| x.as_ref()).flatten() {
                if at.get_border(Left) != t.get_border(Right) {
                    return Err(anyhow!(
                        "Right check failed: {:?}, {:?}",
                        (t.id, i),
                        (index, at.id)
                    ));
                }
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test1_0() -> Result<()> {
        let ex = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        let inp = input_generator(ex)?;
        // println!("{:?}", inp);
        assert_eq!(solve_part1(&inp), 20899048083289);
        Ok(())
    }
}
