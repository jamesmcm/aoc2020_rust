use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::string::ToString;

#[derive(Clone, PartialEq, Debug)]
pub enum Rule {
    SingleRule(Vec<usize>),
    Fixed(char),
    MultiRule(Vec<Vec<usize>>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Input {
    pub rules: HashMap<usize, Rule>,
    pub messages: Vec<String>,
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Result<Input> {
    let mut split = input.split("\n\n");
    let rulestr = split.next().unwrap();
    let mut rules = HashMap::new();

    for r in rulestr.lines() {
        let mut rawsplit = r.split(':');
        let rule_id: usize = rawsplit.next().unwrap().trim().parse()?;

        let raw = rawsplit.next().unwrap();
        if raw.contains('"') {
            let c = raw.replace('"', "").trim().chars().next().unwrap();
            rules.insert(rule_id, Rule::Fixed(c));
            continue;
        }
        if raw.contains('|') {
            let mut multiruleset: Vec<Vec<usize>> = Vec::new();
            for rr in raw.split('|') {
                multiruleset.push(
                    rr.trim()
                        .split_whitespace()
                        .map(|x| x.parse().map_err(|e| anyhow!("{:?}", e)))
                        .collect::<Result<Vec<usize>>>()?,
                );
            }
            rules.insert(rule_id, Rule::MultiRule(multiruleset));
        } else {
            rules.insert(
                rule_id,
                Rule::SingleRule(
                    raw.trim()
                        .split_whitespace()
                        .map(|x| x.parse().map_err(|e| anyhow!("{:?}", e)))
                        .collect::<Result<Vec<usize>>>()?,
                ),
            );
        }
    }
    let messagestr = split.next().unwrap();
    let messages = messagestr.lines().map(|x| x.trim().to_string()).collect();

    Ok(Input { rules, messages })
}

fn rule_match<'a>(rules: &HashMap<usize, Rule>, s: &'a str, target_rule: usize) -> (bool, &'a str) {
    match rules.get(&target_rule).unwrap() {
        Rule::Fixed(c) => {
            if s.is_empty() {
                return (false, s);
            }
            if s.chars().next().unwrap() == *c {
                return (true, &s[1..]);
            } else {
                return (false, s);
            }
        }
        Rule::SingleRule(rvec) => {
            let mut news = s;
            for r in rvec {
                let res = rule_match(rules, news, *r);
                news = res.1;
                if !res.0 {
                    return (false, news);
                }
            }
            return (true, news);
        }
        Rule::MultiRule(multivec) => {
            let mm = multivec
                .iter()
                .map(|rvec| {
                    let mut news = s;
                    for r in rvec {
                        let res = rule_match(rules, news, *r);
                        news = res.1;
                        if !res.0 {
                            return (false, news);
                        }
                    }
                    return (true, news);
                })
                .filter(|x| x.0)
                .next();

            if let Some(m) = mm {
                return (true, m.1);
            } else {
                return (false, s);
            }
        }
    }
}

fn rule_match_loop<'a>(
    rules: &HashMap<usize, Rule>,
    s: &'a str,
    target_rule: usize,
) -> Vec<(bool, &'a str)> {
    match rules.get(&target_rule).unwrap() {
        Rule::Fixed(c) => {
            if s.is_empty() {
                return vec![];
            }
            if s.chars().next().unwrap() == *c {
                return vec![(true, &s[1..])];
            } else {
                return vec![];
            }
        }
        Rule::SingleRule(rvec) => {
            let mut candidates: Vec<&str> = vec![s];
            let mut new_candidates: Vec<&str> = vec![];
            for r in rvec {
                for c in candidates {
                    let res: Vec<(bool, &str)> = rule_match_loop(rules, c, *r)
                        .into_iter()
                        .filter(|x| x.0)
                        .collect();
                    if res.is_empty() {
                        break;
                    }
                    for nc in res {
                        new_candidates.push(nc.1);
                    }
                }
                candidates = new_candidates.clone();
                new_candidates.clear();
            }
            candidates.into_iter().map(|x| (true, x)).collect()
        }
        Rule::MultiRule(multivec) => {
            let mm: Vec<Vec<(bool, &str)>> = multivec
                .iter()
                .map(|rvec| {
                    let mut candidates: Vec<&str> = vec![s];
                    let mut new_candidates: Vec<&str> = vec![];
                    for r in rvec {
                        for c in candidates {
                            let res: Vec<(bool, &str)> = rule_match_loop(rules, c, *r)
                                .into_iter()
                                .filter(|x| x.0)
                                .collect();
                            if res.is_empty() {
                                break;
                            }
                            for nc in res {
                                new_candidates.push(nc.1);
                            }
                        }
                        candidates = new_candidates.clone();
                        new_candidates.clear();
                    }
                    candidates.into_iter().map(|x| (true, x)).collect()
                })
                .collect();

            let mm: Vec<(bool, &str)> = mm.into_iter().flatten().filter(|x| x.0).collect();

            if mm.is_empty() {
                return vec![(false, s)];
            }
            mm
        }
    }
}

fn rule_to_regex(rules: &HashMap<usize, Rule>, target_rule: usize) -> String {
    match rules.get(&target_rule).unwrap() {
        Rule::Fixed(c) => c.to_string(),
        Rule::SingleRule(rvec) => rvec
            .iter()
            .map(|ir| rule_to_regex(rules, *ir))
            .collect::<Vec<String>>()
            .join(""),
        Rule::MultiRule(multivec) => {
            let mm = multivec
                .iter()
                .map(|rvec| {
                    rvec.iter()
                        .map(|ir| rule_to_regex(rules, *ir))
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join("|");
            format!("({})", mm)
        }
    }
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let all_regex = input
        .rules
        .iter()
        .map(|(k, _v)| (*k, rule_to_regex(&input.rules, *k)))
        .collect::<HashMap<usize, String>>();

    let final_regex = format!("^{}$", all_regex.get(&0).unwrap());
    // println!("regex 8: {}", all_regex.get(&8).unwrap());
    // println!("regex 11: {}", all_regex.get(&11).unwrap());
    // println!("Final regex: {}", final_regex);
    let regex = Regex::new(&final_regex).unwrap();

    let mut sum = 0;
    for message in input.messages.iter() {
        // let m = rule_match(&input.rules, message, 0);
        // if m.0 && m.1.is_empty() {
        //     sum += 1
        // }
        if regex.is_match(message) {
            sum += 1;
        }
    }
    sum
}

// #[aoc(day19, part2)]
// pub fn solve_part2(input: &Input) -> usize {
//     let mut myinput = input.clone();
//     myinput
//         .rules
//         .entry(8)
//         .and_modify(|x| *x = Rule::MultiRule(vec![vec![42], vec![42, 8]]));
//     myinput
//         .rules
//         .entry(11)
//         .and_modify(|x| *x = Rule::MultiRule(vec![vec![42, 31], vec![42, 11, 31]]));

//     let mut sum = 0;
//     for message in input.messages.iter() {
//         let m = rule_match_loop(&myinput.rules, message, 0);
//         for n in m {
//             // println!("{}: {:?}", message, n);
//             // && n.1.is_empty()
//             if n.0 {
//                 sum += 1
//             }
//         }
//     }
//     sum
// }

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> usize {
    let mut valid_set: HashSet<String> = HashSet::new();
    let all_regex = input
        .rules
        .iter()
        .map(|(k, _v)| (*k, rule_to_regex(&input.rules, *k)))
        .collect::<HashMap<usize, String>>();

    for n in 1..10 {
        let new8 = format!("({})+", all_regex.get(&42).unwrap());
        let new11 = format!(
            "({}{}|({}){{{}}}({}){{{}}})",
            all_regex.get(&42).unwrap(),
            all_regex.get(&31).unwrap(),
            all_regex.get(&42).unwrap(),
            n,
            all_regex.get(&31).unwrap(),
            n
        );

        let final_regex = format!("^{}{}$", new8, new11);
        let regex = Regex::new(&final_regex).unwrap();
        for message in input.messages.iter() {
            // let m = rule_match(&input.rules, message, 0);
            // if m.0 && m.1.is_empty() {
            //     sum += 1
            // }
            if regex.is_match(message) {
                valid_set.insert(message.clone());
            }
        }
    }
    valid_set.len()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test1_0() -> Result<()> {
        let ex = "0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"

aab
aba
bbb
aaa";
        let inp = input_generator(ex)?;
        // println!("{:?}", inp);
        let m = rule_match(&inp.rules, &inp.messages[0], 0);
        assert_eq!(m.0, true);
        assert_eq!(m.1.is_empty(), true);
        assert_eq!(solve_part1(&inp), 2);
        Ok(())
    }

    #[test]
    fn test1_1() -> Result<()> {
        let ex = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

aa
ababbb
bababa
abbbab
aaabbb
aaaabbb";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 2);
        Ok(())
    }
    #[test]
    fn test1_2() -> Result<()> {
        let ex = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 3);
        Ok(())
    }
    #[ignore]
    #[test]
    fn test2_1() -> Result<()> {
        let ex = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 12);
        Ok(())
    }
    #[ignore]
    #[test]
    fn test2_0() -> Result<()> {
        let ex = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

aa
ababbb
bababa
abbbab
aaabbb
aaaabbb";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 2);
        Ok(())
    }
}
