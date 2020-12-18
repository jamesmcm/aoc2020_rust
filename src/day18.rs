use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Sum,
    Multiply,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Op(Operator),
    OpenBracket,
    CloseBracket,
    Value(u64),
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<Token>>> {
    let mut out: Vec<Vec<Token>> = Vec::new();
    for line in input.lines() {
        let line = line.replace(" ", "");
        let mut tokens: Vec<Token> = Vec::with_capacity(line.len());
        let mut val: Option<u64> = None;

        for c in line.chars() {
            match c {
                '+' => {
                    if let Some(v) = val.take() {
                        tokens.push(Token::Value(v));
                    }
                    tokens.push(Token::Op(Operator::Sum));
                }
                '*' => {
                    if let Some(v) = val.take() {
                        tokens.push(Token::Value(v));
                    }
                    tokens.push(Token::Op(Operator::Multiply));
                }
                '(' => {
                    if let Some(v) = val.take() {
                        tokens.push(Token::Value(v));
                    }
                    tokens.push(Token::OpenBracket);
                }
                ')' => {
                    if let Some(v) = val.take() {
                        tokens.push(Token::Value(v));
                    }
                    tokens.push(Token::CloseBracket);
                }
                x => {
                    val = match val {
                        Some(v) => Some(v * 10 + (x as u8 - 48) as u64),
                        None => Some((x as u8 - 48) as u64),
                    };
                }
            }
        }
        if let Some(v) = val.take() {
            tokens.push(Token::Value(v));
        }
        out.push(tokens);
    }

    Ok(out)
}

pub fn evaluate(stream: &[Token]) -> Result<u64> {
    let mut stack: Vec<(Option<u64>, Option<Operator>)> = Vec::new();
    let mut lhs: Option<u64> = None;
    let mut op: Option<Operator> = None;

    use Token::*;
    for t in stream {
        match t {
            Value(v) => {
                if op.is_none() {
                    lhs = Some(*v);
                } else {
                    match op.take() {
                        Some(Operator::Multiply) => {
                            lhs = Some(lhs.unwrap() * v);
                        }
                        Some(Operator::Sum) => {
                            lhs = Some(lhs.unwrap() + v);
                        }
                        None => {
                            return Err(anyhow!(
                                "Operator was None when right bracket encountered"
                            ));
                        }
                    }
                }
            }
            Op(o) => {
                op = Some(*o);
            }
            OpenBracket => {
                stack.push((lhs.take(), op.take()));
            }
            CloseBracket => {
                let (prevlhs, prevop) = stack
                    .pop()
                    .expect("Stack empty when right bracket encountered");
                if let Some(plhs) = prevlhs {
                    match prevop.expect("Operator was None when right bracket encountered") {
                        Operator::Multiply => {
                            lhs = Some(plhs * lhs.unwrap());
                        }
                        Operator::Sum => {
                            lhs = Some(plhs + lhs.unwrap());
                        }
                    }
                } else {
                }
            }
        }
    }

    Ok(lhs.expect("Returned None lhs"))
}

// pub fn evaluate2(stream: &[Token]) -> Result<u64> {
//     let mut tokens = stream.clone();
//     let mut i = 0;
//     // Modify

//     evaluate(&tokens)
// }

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EvalMode {
    LHS,
    RHS,
}

pub fn evaluate2(stream: &[Token]) -> Result<u64> {
    use EvalMode::*;

    let mut stack: Vec<(
        Option<u64>,
        Option<u64>,
        Option<Operator>,
        Option<Operator>,
        EvalMode,
    )> = Vec::new();
    let mut lhs: Option<u64> = None;
    let mut rhs: Option<u64> = None;
    let mut lhs_op: Option<Operator> = None;
    let mut rhs_op: Option<Operator> = None;
    let mut eval: EvalMode = LHS;

    use Token::*;
    for (i, t) in stream.iter().enumerate() {
        match t {
            Value(v) => {
                if eval == LHS {
                    if lhs_op.is_none() {
                        lhs = Some(*v);
                    } else {
                        if stream.get(i + 1) != Some(&Token::Op(Operator::Sum)) {
                            match lhs_op.take() {
                                Some(Operator::Multiply) => {
                                    lhs = Some(lhs.unwrap() * v);
                                }
                                Some(Operator::Sum) => {
                                    lhs = Some(lhs.unwrap() + v);
                                }
                                None => {
                                    return Err(anyhow!(
                                        "LHS Operator was None when right bracket encountered"
                                    ));
                                }
                            }
                        } else {
                            rhs = Some(*v);
                            eval = RHS;
                        }
                    }
                } else {
                    if rhs_op.is_none() {
                        rhs = Some(*v);
                    } else {
                        match rhs_op.take() {
                            Some(Operator::Multiply) => {
                                rhs = Some(rhs.unwrap() * v);
                            }
                            Some(Operator::Sum) => {
                                rhs = Some(rhs.unwrap() + v);
                            }
                            None => {
                                return Err(anyhow!(
                                    "RHS Operator was None when right bracket encountered"
                                ));
                            }
                        }
                    }
                }
            }
            Op(o) => {
                if eval == LHS {
                    lhs_op = Some(*o);
                } else {
                    match o {
                        Operator::Sum => {
                            rhs_op = Some(*o);
                        }
                        Operator::Multiply => {
                            eval = LHS;
                            match lhs_op {
                                Some(Operator::Multiply) => {
                                    lhs = Some(lhs.unwrap() * rhs.unwrap());
                                }
                                Some(Operator::Sum) => {
                                    lhs = Some(lhs.unwrap() + rhs.unwrap());
                                }
                                None => {
                                    if lhs.is_none() {
                                        lhs = rhs;
                                    } else {
                                        return Err(anyhow!(
                                            "LHS Operator was None when + to end RHS encountered"
                                        ));
                                    }
                                }
                            }

                            lhs_op = Some(Operator::Multiply);
                            rhs = None;
                        }
                    }
                };
            }
            OpenBracket => {
                // Find pos closing bracket
                let mut count_bracket: i32 = 0;
                let mut j = i;
                let next_op = loop {
                    if j >= stream.len() {
                        break Err(anyhow!("No closing bracket found for opening bracket!"));
                    }
                    if stream[j] == Token::OpenBracket {
                        count_bracket += 1;
                    }
                    if stream[j] == Token::CloseBracket {
                        count_bracket -= 1;
                    }
                    if count_bracket == 0 {
                        break Ok(stream.get(j + 1));
                    }
                    j += 1;
                };

                // println!("j: {}, nextop: {:?}", j, next_op);

                if next_op? == Some(&Token::Op(Operator::Sum)) {
                    eval = RHS;
                }

                stack.push((lhs.take(), rhs.take(), lhs_op.take(), rhs_op.take(), eval));
                eval = LHS;
            }
            CloseBracket => {
                let (prevlhs, prevrhs, prevlhsop, prevrhsop, preveval) = stack
                    .pop()
                    .expect("Stack empty when right bracket encountered");

                eval = preveval;

                if let Some(r) = rhs {
                    match lhs_op {
                        Some(Operator::Multiply) => {
                            lhs = Some(lhs.unwrap() * r);
                        }
                        Some(Operator::Sum) => {
                            lhs = Some(lhs.unwrap() + r);
                        }
                        None => {
                            if lhs.is_none() {
                                lhs = Some(r);
                            } else {
                                return Err(anyhow!(
                                    "LHS Operator was None when + to end RHS encountered"
                                ));
                            }
                        }
                    }
                }

                if preveval == LHS {
                    if let Some(plhs) = prevlhs {
                        match prevlhsop.expect("Operator was None when right bracket encountered") {
                            Operator::Multiply => {
                                lhs = Some(plhs * lhs.unwrap());
                            }
                            Operator::Sum => {
                                lhs = Some(plhs + lhs.unwrap());
                            }
                        }
                        rhs_op = prevrhsop;
                        rhs = prevrhs;
                    } else {
                        lhs_op = prevlhsop;
                        rhs = prevrhs;
                        rhs_op = prevrhsop;
                    }
                } else {
                    if let Some(prhs) = prevrhs {
                        match prevrhsop.expect("Operator was None when right bracket encountered") {
                            Operator::Multiply => {
                                rhs = Some(prhs * lhs.unwrap());
                            }
                            Operator::Sum => {
                                rhs = Some(prhs + lhs.unwrap());
                            }
                        }
                        lhs_op = prevlhsop;
                        lhs = prevlhs;
                    } else {
                        rhs = lhs;
                        lhs_op = prevlhsop;
                        lhs = prevlhs;
                        rhs_op = prevrhsop;
                    }
                }
            }
        }
        // println!(
        //     "lhs: {:?}, rhs: {:?}, lhs_op: {:?}, rhs_op: {:?}, eval: {:?}\nstack: {:?}",
        //     lhs, rhs, lhs_op, rhs_op, eval, stack
        // );
    }

    if let Some(r) = rhs {
        match lhs_op {
            Some(Operator::Multiply) => {
                lhs = Some(lhs.unwrap() * r);
            }
            Some(Operator::Sum) => {
                lhs = Some(lhs.unwrap() + r);
            }
            None => {
                if lhs.is_none() {
                    lhs = Some(r);
                } else {
                    return Err(anyhow!(
                        "LHS Operator was None when + to end RHS encountered"
                    ));
                }
            }
        }
    }

    // println!(
    //     "lhs: {:?}, rhs: {:?}, lhs_op: {:?}, rhs_op: {:?}, eval: {:?}\nstack: {:?}",
    //     lhs, rhs, lhs_op, rhs_op, eval, stack
    // );

    Ok(lhs.expect("Returned None lhs"))
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[Vec<Token>]) -> u64 {
    input.iter().map(|v| evaluate(v).unwrap()).sum()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[Vec<Token>]) -> u64 {
    input.iter().map(|v| evaluate2(v).unwrap()).sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test1_0() -> Result<()> {
        let ex = "1 + 2 * 3 + 4 * 5 + 6";
        let inp = input_generator(ex)?;
        assert_eq!(evaluate(&inp[0]).unwrap(), 71);
        Ok(())
    }

    #[test]
    fn test1_1() -> Result<()> {
        let ex = "1 + 2 * 3 + 4 * 5 + 6";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 71);
        Ok(())
    }
    #[test]
    fn test1_2() -> Result<()> {
        let ex = "1 + (2 * 3) + (4 * (5 + 6))";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 51);
        Ok(())
    }
    #[test]
    fn test1_3() -> Result<()> {
        let ex = "2 * 3 + (4 * 5)";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 26);
        Ok(())
    }
    #[test]
    fn test1_4() -> Result<()> {
        let ex = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 437);
        Ok(())
    }
    #[test]
    fn test1_5() -> Result<()> {
        let ex = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 12240);
        Ok(())
    }
    #[test]
    fn test1_6() -> Result<()> {
        let ex = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part1(&inp), 13632);
        Ok(())
    }
    #[test]
    fn test2_0() -> Result<()> {
        let ex = "(1 + 2) * (3 + 4) * (5 + 6)";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 231);
        Ok(())
    }

    #[test]
    fn test2_00() -> Result<()> {
        let ex = "((((1 + 2) * 3) + 4) * 5) + 6";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 71);
        Ok(())
    }
    #[test]
    fn test2_1() -> Result<()> {
        let ex = "1 + 2 * 3 + 4 * 5 + 6";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 231);
        Ok(())
    }
    #[test]
    fn test2_2() -> Result<()> {
        let ex = "1 + (2 * 3) + (4 * (5 + 6))";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 51);
        Ok(())
    }
    #[test]
    fn test2_3() -> Result<()> {
        let ex = "2 * 3 + (4 * 5)";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 46);
        Ok(())
    }
    #[test]
    fn test2_4() -> Result<()> {
        let ex = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 1445);
        Ok(())
    }
    #[test]
    fn test2_5() -> Result<()> {
        let ex = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 669060);
        Ok(())
    }
    #[test]
    fn test2_6() -> Result<()> {
        let ex = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 23340);
        Ok(())
    }
    #[test]
    fn test2_6_1() -> Result<()> {
        let ex = "0 + ( 0 + (2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 23340);
        Ok(())
    }
    #[test]
    fn test2_6_2() -> Result<()> {
        let ex = "1 * ( 1 * (2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 23340);
        Ok(())
    }
    #[test]
    fn test2_bracket() -> Result<()> {
        let ex = "2 * (2 * 3) + 5";
        let inp = input_generator(ex)?;
        assert_eq!(solve_part2(&inp), 22);
        Ok(())
    }
}
