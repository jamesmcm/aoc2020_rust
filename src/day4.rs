use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug)]
enum HeightMeasure {
    Cm,
    Inch,
}

impl FromStr for HeightMeasure {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(Self::Cm),
            "in" => Ok(Self::Inch),
            _ => Err(anyhow!("Unknown height measure: {}", s)),
        }
    }
}

#[derive(Debug)]
enum EyeColour {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColour {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(Self::Amber),
            "blu" => Ok(Self::Blue),
            "brn" => Ok(Self::Brown),
            "gry" => Ok(Self::Grey),
            "grn" => Ok(Self::Green),
            "hzl" => Ok(Self::Hazel),
            "oth" => Ok(Self::Other),
            _ => Err(anyhow!("Unknown eye colour: {}", s)),
        }
    }
}

#[derive(Debug)]
struct ValidPassport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: u32,
    hgt_type: HeightMeasure,
    hcl: String,
    ecl: EyeColour,
    pid: u32,
}

impl TryFrom<&HashMap<String, String>> for ValidPassport {
    type Error = anyhow::Error;
    fn try_from(hm: &HashMap<String, String>) -> Result<Self> {
        let byr_raw = hm.get("byr").ok_or(anyhow!("No byr"))?;
        let byr: u32 = byr_raw.parse()?;
        if byr < 1920 || byr > 2002 {
            return Err(anyhow!("Bad byr: {}", byr));
        }

        let iyr_raw = hm.get("iyr").ok_or(anyhow!("No iyr"))?;
        let iyr: u32 = iyr_raw.parse()?;
        if iyr < 2010 || iyr > 2020 {
            return Err(anyhow!("Bad iyr: {}", iyr));
        }

        let eyr_raw = hm.get("eyr").ok_or(anyhow!("No eyr"))?;
        let eyr: u32 = eyr_raw.parse()?;
        if eyr < 2020 || eyr > 2030 {
            return Err(anyhow!("Bad eyr: {}", eyr));
        }

        let hgt_raw = hm.get("hgt").ok_or(anyhow!("No hgt"))?;
        let hgt_regex = Regex::new(r"(?P<h>[0-9]{2,3})(?P<t>in|cm)").expect("Bad regex");
        let caps = hgt_regex
            .captures(hgt_raw)
            .ok_or(anyhow!("No hgt captures"))?;
        let hgt: u32 = caps
            .get(1)
            .ok_or(anyhow!("No hgt capture"))?
            .as_str()
            .parse()?;
        let hgt_type =
            HeightMeasure::from_str(caps.get(2).ok_or(anyhow!("No hgt type capture"))?.as_str())?;
        match hgt_type {
            HeightMeasure::Cm => {
                if hgt < 150 || hgt > 193 {
                    return Err(anyhow!("Bad height number: {}", hgt));
                }
            }
            HeightMeasure::Inch => {
                if hgt < 59 || hgt > 76 {
                    return Err(anyhow!("Bad height number: {}", hgt));
                }
            }
        };

        let hcl_raw = hm.get("hcl").ok_or(anyhow!("No hcl"))?;
        let hcl_regex = Regex::new(r"#(?P<hex>[0-9a-f]+)").expect("Bad regex");
        let caps = hcl_regex
            .captures(hcl_raw)
            .ok_or(anyhow!("No hcl captures"))?;
        let hcl: String = caps
            .get(1)
            .ok_or(anyhow!("No hcl capture"))?
            .as_str()
            .to_string();
        if hcl.len() != 6 {
            return Err(anyhow!("Bad hcl length: {}", hcl));
        }

        let ecl_raw = hm.get("ecl").ok_or(anyhow!("No ecl"))?;
        let ecl = EyeColour::from_str(ecl_raw)?;

        let pid_raw = hm.get("pid").ok_or(anyhow!("No pid"))?;
        let pid_regex = Regex::new(r"(?P<id>[0-9]+)").expect("Bad regex");
        let caps = pid_regex
            .captures(pid_raw)
            .ok_or(anyhow!("No pid captures"))?;
        let pid = caps.get(1).ok_or(anyhow!("No pid capture"))?.as_str();

        if pid.len() != 9 {
            return Err(anyhow!("Bad pid length: {}", pid));
        }

        let pid: u32 = pid.parse()?;

        Ok(Self {
            byr,
            iyr,
            eyr,
            hgt,
            hgt_type,
            hcl,
            ecl,
            pid,
        })
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<HashMap<String, String>> {
    let re = Regex::new(r"\n(?P<c>[^\n])").expect("Bad regex");
    let input = re.replace_all(input, " ${c}");
    input
        .lines()
        .map(|l| {
            let mut hm = HashMap::with_capacity(8);
            let pairs = l.trim().split(' ');
            pairs.for_each(|p| {
                let mut splitp = p.split(':');
                let key = splitp.next().expect("No key");
                let val = splitp.next().expect("No val");
                // (key.to_string(), val.to_string())
                hm.insert(key.to_string(), val.to_string());
            });
            hm
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(map: &[HashMap<String, String>]) -> i32 {
    let needed_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    map.iter().fold(0, |acc, x| {
        if needed_keys.iter().all(|k| x.contains_key(*k)) {
            acc + 1
        } else {
            acc
        }
    })
}

#[aoc(day4, part2)]
pub fn solve_part2(map: &[HashMap<String, String>]) -> usize {
    let needed_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    map.iter()
        .filter(|x| needed_keys.iter().all(|k| x.contains_key(*k)))
        .map(|x| ValidPassport::try_from(x))
        // .for_each(|x| {
        //     if let Ok(z) = x {
        //         println!("{:?}", z);
        //     }
        // });
        .filter(|x| x.is_ok())
        .count()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1() {
        let ex = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let input = input_generator(ex);
        assert_eq!(solve_part1(&input), 2);
    }
    #[test]
    fn test2() {
        let ex = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let input = input_generator(ex);
        assert_eq!(solve_part2(&input), 0);
    }
    #[test]
    fn test3() {
        let ex = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let input = input_generator(ex);
        assert_eq!(solve_part2(&input), 4);
    }
}
