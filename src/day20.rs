use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    pub transform: Transform,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Border {
    Top,
    Bottom,
    Left,
    Right,
}

pub fn rotate_90(input: &[Vec<Pixel>]) -> Vec<Vec<Pixel>> {
    let mut newtile = input.to_vec();
    let len = newtile[0].len();
    for i in 0..len {
        for j in 0..len {
            newtile[i][j] = input[len - j - 1][i];
        }
    }
    newtile
}

pub fn flip_horizontal(input: &[Vec<Pixel>]) -> Vec<Vec<Pixel>> {
    let mut newtile = input.to_vec();
    let len = newtile[0].len();
    for i in 0..len {
        for j in 0..len {
            newtile[i][j] = input[i][len - j - 1];
        }
    }
    newtile
}

impl Tile {
    pub fn new(array: Vec<Vec<Pixel>>, id: u32) -> Self {
        let length = array[0].len();
        Tile {
            array,
            length,
            id,
            transform: Transform::Identity,
        }
    }

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
pub enum Transform {
    Identity,
    FH,
    R90,
    R180,
    R270,
    R90FH,
    R180FH,
    R270FH,
}

pub fn get_map(
    input: &[Tile],
) -> (
    Vec<Option<(u32, Transform)>>,
    HashMap<(u32, Transform), Tile>,
) {
    // Generate all 8 Tile possibilities first identified by enum
    // Store in (id, transform_enum) HashMap
    let mut tile_map: HashMap<(u32, Transform), Tile> = HashMap::with_capacity(8 * input.len());
    // println!("{:?}", input);
    for tile in input {
        tile_map.insert((tile.id, Transform::Identity), tile.clone());
        tile_map.insert(
            (tile.id, Transform::FH),
            Tile {
                id: tile.id,
                transform: Transform::FH,
                length: tile.length,
                array: flip_horizontal(&tile.array),
            },
        );
        tile_map.insert(
            (tile.id, Transform::R90),
            Tile {
                id: tile.id,
                transform: Transform::R90,
                length: tile.length,
                array: rotate_90(&tile.array),
            },
        );
        tile_map.insert(
            (tile.id, Transform::R180),
            Tile {
                id: tile.id,
                transform: Transform::R180,
                length: tile.length,
                array: rotate_90(&rotate_90(&tile.array)),
            },
        );
        tile_map.insert(
            (tile.id, Transform::R270),
            Tile {
                id: tile.id,
                transform: Transform::R270,
                length: tile.length,
                array: rotate_90(&rotate_90(&rotate_90(&tile.array))),
            },
        );
        tile_map.insert(
            (tile.id, Transform::R90FH),
            Tile {
                id: tile.id,
                transform: Transform::R90FH,
                length: tile.length,
                array: flip_horizontal(&rotate_90(&tile.array)),
            },
        );
        tile_map.insert(
            (tile.id, Transform::R180FH),
            Tile {
                id: tile.id,
                transform: Transform::R180FH,
                length: tile.length,
                array: flip_horizontal(&rotate_90(&rotate_90(&tile.array))),
            },
        );
        tile_map.insert(
            (tile.id, Transform::R270FH),
            Tile {
                id: tile.id,
                transform: Transform::R270FH,
                length: tile.length,
                array: flip_horizontal(&rotate_90(&rotate_90(&rotate_90(&tile.array)))),
            },
        );
    }
    // tile_map.iter().for_each(|x| {
    //     println!("{:?}", x.0);
    //     for r in &x.1.array {
    //         println!(
    //             "{}",
    //             r.iter()
    //                 .map(|c| c.to_string())
    //                 .collect::<Vec<String>>()
    //                 .join("")
    //         );
    //     }
    // });
    let num_tiles = (input.len() as f64).sqrt() as usize;
    println!("Num tiles: {}, sqrt: {}", input.len(), num_tiles);

    // Also build (border, bordertype) -> (id, transform_enum) map - are they unique?
    let mut border_map: HashMap<(Vec<Pixel>, Border), Vec<(u32, Transform)>> =
        HashMap::with_capacity(tile_map.len());
    tile_map.iter().for_each(|x| {
        border_map
            .entry((x.1.get_border(Border::Top), Border::Top))
            .and_modify(|v| v.push(*x.0))
            .or_insert(vec![x.0.clone()]);

        border_map
            .entry((x.1.get_border(Border::Bottom), Border::Bottom))
            .and_modify(|v| v.push(*x.0))
            .or_insert(vec![x.0.clone()]);

        border_map
            .entry((x.1.get_border(Border::Left), Border::Left))
            .and_modify(|v| v.push(*x.0))
            .or_insert(vec![x.0.clone()]);
        border_map
            .entry((x.1.get_border(Border::Right), Border::Right))
            .and_modify(|v| v.push(*x.0))
            .or_insert(vec![x.0.clone()]);
    });

    // Can then get next candidate by checking if there is valid border to the right/below
    // Remaining Available Vec should be vec of Ids, then loop over transform enum to get all
    // possibilities
    // Candidate is (id, transform_enum) tuple
    // Placed Vec should be Vec of (id, transform_enum)s
    let placed: Vec<Option<(u32, Transform)>> = vec![None; num_tiles * num_tiles];
    let pc = step_solve(
        &tile_map,
        &border_map,
        placed,
        input.iter().map(|x| x.id).collect(),
        0,
        None,
        num_tiles,
    );
    let placed = pc.unwrap().0;
    (placed, tile_map)
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &[Tile]) -> u64 {
    let num_tiles = (input.len() as f64).sqrt() as usize;
    let (placed, tile_map) = get_map(input);
    println!("{:?}", placed);

    let mut i = 0;
    while i < placed.len() {
        for r in 0..tile_map
            .get(placed[0].as_ref().unwrap())
            .unwrap()
            .array
            .len()
        {
            let mut v = Vec::new();
            for j in 0..num_tiles {
                v.push(
                    tile_map.get(placed[i + j].as_ref().unwrap()).unwrap().array[r]
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

    placed[0].as_ref().unwrap().0 as u64
        * placed[num_tiles - 1].as_ref().unwrap().0 as u64
        * placed[num_tiles * num_tiles - num_tiles]
            .as_ref()
            .unwrap()
            .0 as u64
        * placed[num_tiles * num_tiles - 1].as_ref().unwrap().0 as u64
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &[Tile]) -> u64 {
    let num_tiles = (input.len() as f64).sqrt() as usize;
    let (placed, tile_map) = get_map(input);

    // Build large map

    // Slide across image
    // Get indices of sea monster pixels
    // Count unique indices (could overlap)
}

pub fn step_solve(
    tile_map: &HashMap<(u32, Transform), Tile>,
    border_map: &HashMap<(Vec<Pixel>, Border), Vec<(u32, Transform)>>,
    mut placed: Vec<Option<(u32, Transform)>>,
    available: Vec<u32>,
    mut target_pos: usize,
    candidate: Option<(u32, Transform)>,
    num_tiles: usize,
) -> Result<(Vec<Option<(u32, Transform)>>, Vec<u32>)> {
    if candidate.is_some() {
        if target_pos > ((num_tiles * num_tiles) - 1) {
            return Ok((placed, available));
        }
        placed[target_pos] = candidate;

        if let Err(e) = valid_state(tile_map, &placed, num_tiles, target_pos) {
            return Err(e);
        }

        if available.is_empty() {
            return Ok((placed, available));
        }
        target_pos += 1;
    }

    let candidates: Vec<(u32, Transform)> = {
        if target_pos == 0 {
            tile_map.keys().cloned().collect()
        } else if target_pos % num_tiles == 0 {
            border_map
                .get(&(
                    tile_map
                        .get(&placed[target_pos - num_tiles].unwrap())
                        .unwrap()
                        .get_border(Border::Bottom),
                    Border::Top,
                ))
                .unwrap_or(&vec![])
                .clone()
        } else {
            border_map
                .get(&(
                    tile_map
                        .get(&placed[target_pos - 1].unwrap())
                        .unwrap()
                        .get_border(Border::Right),
                    Border::Left,
                ))
                .unwrap_or(&vec![])
                .clone()
        }
    };

    if available.len() < 5 {
        println!(
            "Placed: {:?}\nCandidates: {:?}\nAvailable: {:?}\n-------------\n",
            placed, candidates, available
        );
    }

    let candidates: Vec<(u32, Transform)> = candidates
        .into_iter()
        .filter(|x| available.contains(&x.0))
        .collect();

    for candidate in candidates {
        let mut new_available = available.clone();
        new_available.remove(
            available
                .iter()
                .enumerate()
                .find(|x| *x.1 == candidate.0)
                .map(|x| x.0)
                .unwrap(),
        );
        let pc = step_solve(
            tile_map,
            border_map,
            placed.clone(),
            new_available.clone(),
            target_pos,
            Some(candidate),
            num_tiles,
        );
        if pc.is_ok() {
            return pc;
        }
    }
    Err(anyhow!("No valid follow-up states"))
}

pub fn valid_state(
    tile_map: &HashMap<(u32, Transform), Tile>,
    placed: &[Option<(u32, Transform)>],
    num_tiles: usize,
    target_pos: usize,
) -> Result<()> {
    use Border::*;
    let i = target_pos;
    let opt = &placed[i];
    if let Some(t) = opt {
        let index: i64 = (i as i64) - num_tiles as i64;
        if index >= 0 {
            if let Some(at) = placed.get(index as usize).map(|x| x.as_ref()).flatten() {
                let at = tile_map.get(at).unwrap();
                let t = tile_map.get(t).unwrap();
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
                let at = tile_map.get(at).unwrap();
                let t = tile_map.get(t).unwrap();
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
                let at = tile_map.get(at).unwrap();
                let t = tile_map.get(t).unwrap();
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
                let at = tile_map.get(at).unwrap();
                let t = tile_map.get(t).unwrap();
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
