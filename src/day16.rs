use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldRange {
    min: i64,
    max: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    name: String,
    ranges: Vec<FieldRange>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    values: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Input {
    fields: Vec<Field>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl FromStr for FieldRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fsplit = s.split('-');
        let min = fsplit.next().unwrap().parse()?;
        let max = fsplit.next().unwrap().parse()?;
        Ok(Self { min, max })
    }
}

impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fsplit = s.split(':');
        let name = fsplit.next().unwrap();
        let ranges = fsplit
            .next()
            .unwrap()
            .split(" or ")
            .map(|x| FieldRange::from_str(x.trim()).unwrap())
            .collect();
        Ok(Self {
            name: name.to_string(),
            ranges,
        })
    }
}

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Result<Vec<i64>> = s
            .split(',')
            .map(|x| x.parse().map_err(|e| anyhow!("{:?}", e)))
            .collect();
        Ok(Self { values: vals? })
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<Input> {
    let mut blocks = input.split("\n\n");
    let mut fields = Vec::new();
    for line in blocks.next().unwrap().lines() {
        fields.push(Field::from_str(line)?);
    }

    let mut your_ticket_block = blocks.next().unwrap().lines();
    your_ticket_block.next();
    let your_ticket = Ticket::from_str(your_ticket_block.next().unwrap())?;

    let mut nearby_ticket_block = blocks.next().unwrap().lines();
    let mut nearby_tickets = Vec::new();
    nearby_ticket_block.next();
    for t in nearby_ticket_block {
        nearby_tickets.push(Ticket::from_str(t)?);
    }

    Ok(Input {
        fields,
        your_ticket,
        nearby_tickets,
    })
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> i64 {
    input
        .nearby_tickets
        .iter()
        .map(|t| -> i64 {
            t.values
                .iter()
                .map(|v| {
                    (
                        v,
                        input
                            .fields
                            .iter()
                            .any(|f| f.ranges.iter().any(|r| *v >= r.min && *v <= r.max)),
                    )
                })
                .filter(|x| !x.1)
                .map(|x| x.0)
                .sum()
        })
        .sum()
}

pub fn filter_invalid_tickets(tickets: &[Ticket], fields: &[Field]) -> Vec<Ticket> {
    tickets
        .iter()
        .cloned()
        .map(|t| {
            (
                t.clone(),
                t.values
                    .iter()
                    .map(|v| {
                        fields
                            .iter()
                            .any(|f| f.ranges.iter().any(|r| *v >= r.min && *v <= r.max))
                    })
                    .all(|x| x),
            )
        })
        .filter(|x| x.1)
        .map(|x| x.0)
        .collect()
}

pub fn find_field_cols(tickets: &[Ticket], fields: &[Field]) -> HashMap<String, usize> {
    let mut field_set: Vec<Vec<usize>> = vec![(0..fields.len()).collect(); fields.len()]; // Vec<Vec<field_index>>
    let mut out_map: HashMap<String, usize> = HashMap::with_capacity(fields.len());

    for t in tickets {
        // Check each column against possible fields in field_set
        // Remove indices of impossible fields from Vec
        for (col, val) in t.values.iter().enumerate() {
            field_set[col] = field_set[col]
                .iter()
                .filter(|fi| {
                    fields[**fi]
                        .ranges
                        .iter()
                        .any(|r| *val >= r.min && *val <= r.max)
                })
                .map(|x| *x)
                .collect()
        }
    }

    // println!("Field set: {:?}", field_set);

    let mut max = field_set.iter().map(|x| x.len()).max().unwrap();
    while max > 0 {
        let mut to_remove = Vec::new();
        field_set
            .iter()
            .enumerate()
            .filter(|x| x.1.len() == 1)
            .for_each(|x| {
                to_remove.push(x.1[0]);
                out_map.insert(fields[x.1[0]].name.clone(), x.0);
            });
        to_remove.into_iter().for_each(|x| {
            field_set.iter_mut().for_each(|v| {
                if let Some(index) = v.iter().position(|z| *z == x) {
                    v.remove(index);
                }
            })
        });
        max = field_set.iter().map(|x| x.len()).max().unwrap();
    }
    // println!("Map: {:?}", out_map);

    out_map

    // field_set
    //     .iter()
    //     .enumerate()
    //     .map(|x| (fields[x.1[0]].name.clone(), x.0))
    //     .collect()
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Input) -> i64 {
    let valid_tickets = filter_invalid_tickets(&input.nearby_tickets, &input.fields);
    let map = find_field_cols(&valid_tickets, &input.fields);
    map.iter()
        .filter(|x| x.0.starts_with("departure"))
        .map(|x| input.your_ticket.values[*x.1])
        .fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test1_1() -> Result<()> {
        let ex = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 71);
        Ok(())
    }
    #[test]
    fn filter_invalid() -> Result<()> {
        let ex = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let inp = input_generator(ex)?;
        assert_eq!(
            filter_invalid_tickets(&inp.nearby_tickets, &inp.fields),
            vec![Ticket {
                values: vec![7, 3, 47]
            }]
        );
        Ok(())
    }
    #[test]
    fn filter_invalid2() -> Result<()> {
        let ex = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let inp = input_generator(ex)?;
        assert_eq!(
            filter_invalid_tickets(&inp.nearby_tickets, &inp.fields),
            vec![
                Ticket {
                    values: vec![3, 9, 18]
                },
                Ticket {
                    values: vec![15, 1, 5]
                },
                Ticket {
                    values: vec![5, 14, 9]
                }
            ]
        );
        Ok(())
    }
    #[test]
    fn test_fields() -> Result<()> {
        let ex = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let inp = input_generator(ex)?;
        let mut map = HashMap::new();
        map.insert("row".to_string(), 0);
        map.insert("class".to_string(), 1);
        map.insert("seat".to_string(), 2);
        assert_eq!(find_field_cols(&inp.nearby_tickets, &inp.fields), map);

        Ok(())
    }
}
