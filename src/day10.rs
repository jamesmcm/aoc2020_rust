use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[i32]) -> usize {
    let mut sorted = input.to_vec();
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(4);
    sorted.push(0);
    sorted.push(sorted.iter().max().unwrap() + 3);
    sorted.sort();

    sorted.windows(2).for_each(|x| {
        let diff = x[1] - x[0];
        map.entry(diff).and_modify(|x| *x += 1).or_insert(1);
    });

    map.get(&1).unwrap() * map.get(&3).unwrap()
}

#[derive(Debug, PartialEq)]
pub struct Node {
    val: i32,
    links: Vec<usize>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            links: Vec::new(),
        }
    }

    pub fn add_link(&mut self, index: usize) {
        self.links.push(index);
    }
}

pub fn generate_nodes(sorted: &[i32]) -> Vec<Node> {
    let mut node_vec: Vec<Node> = Vec::with_capacity(sorted.len());

    for (i, x) in sorted.iter().enumerate() {
        let mut node = Node::new(*x);
        for l in 1..=3 {
            if let Some(z) = sorted.get(i + l) {
                if *z <= sorted[i] + 3 {
                    node.add_link(i + l);
                }
            }
        }
        node_vec.push(node)
    }
    node_vec
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[i32]) -> usize {
    let mut sorted = input.to_vec();
    sorted.push(0);
    sorted.push(sorted.iter().max().unwrap() + 3);
    sorted.sort();

    let nodes = generate_nodes(&sorted);
    let mut scores = vec![0; sorted.len()];
    *scores.last_mut().unwrap() = 1;
    for i in (0..nodes.len() - 1).rev() {
        let mut s = 0;
        for link in &nodes[i].links {
            s += scores[*link];
        }
        scores[i] = s;
    }
    // println!("{:?}", scores);

    *scores.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = "16
10
15
5
1
11
7
19
6
12
4
";
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), 35);
    }
    #[test]
    fn test_two() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), 220);
    }
    #[test]
    fn test_nodes() {
        let input = "1
2
3";
        let input = input_generator(input);
        let mut sorted = input.to_vec();
        sorted.push(0);
        sorted.push(sorted.iter().max().unwrap() + 3);
        sorted.sort();
        assert_eq!(
            generate_nodes(&sorted),
            vec![
                Node {
                    val: 0,
                    links: vec![1, 2, 3]
                },
                Node {
                    val: 1,
                    links: vec![2, 3]
                },
                Node {
                    val: 2,
                    links: vec![3]
                },
                Node {
                    val: 3,
                    links: vec![4]
                },
                Node {
                    val: 6,
                    links: vec![]
                }
            ]
        );
    }
    #[test]
    fn test2_1() {
        let input = "16
10
15
5
1
11
7
19
6
12
4
";
        let input = input_generator(input);
        assert_eq!(solve_part2(&input), 8);
    }
    #[test]
    fn test2_two() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let input = input_generator(input);
        assert_eq!(solve_part2(&input), 19208);
    }
}
