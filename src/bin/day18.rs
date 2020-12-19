use std::error::Error;

use aoc2020_niels_overkamp::common::{self, AOCResult};

use crate::Expression::{Number, Operation};
use crate::Op::{Add, Mul};

const DAY: &str = "day18";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for line in input {
        let (exp, left) = parse_expression_1(line.to_owned())?;
        if left.len() > 0 {
            return Err(format!("Could not parse {}, unexpected {}", line, left).into());
        }
        sum_1 += exp.eval();

        let (exp, left) = parse_expression_2(line.to_owned())?;
        if left.len() > 0 {
            return Err(format!("Could not parse {}, unexpected {}", line, left).into());
        }
        sum_2 += exp.eval();
    }

    Ok([Some(sum_1.to_string()), Some(sum_2.to_string())])
}

fn parse_expression_1(mut input: String) -> Result<(Expression, String), Box<dyn Error>> {
    let mut prev_expression = None;
    let mut prev_op = None;
    loop {
        let res = next_char(input);
        let c = res.0;
        input = res.1;
        if let Some(c) = c {
            if prev_op.is_none() && prev_expression.is_some() {
                if c == ')' {
                    break;
                }
                prev_op = Some(Op::from_char(c).ok_or(format!("Expected op, got {}", c))?);
            } else {
                if c.is_digit(10) {
                    let num = c.to_digit(10).ok_or(format!("Expected digit got {}", c))?;
                    if prev_expression.is_none() {
                        prev_expression = Some(Expression::Number(num));
                    } else {
                        prev_expression = Some(Expression::Operation(
                            Box::new(Expression::Number(num)),
                            prev_op.take().unwrap(),
                            Box::new(prev_expression.take().unwrap()),
                        ))
                    }
                } else {
                    if c == '(' {
                        let res = parse_expression_1(input)?;
                        input = res.1;
                        if prev_expression.is_none() {
                            prev_expression = Some(res.0);
                        } else {
                            prev_expression = Some(Expression::Operation(
                                Box::new(res.0),
                                prev_op.take().unwrap(),
                                Box::new(prev_expression.take().unwrap()),
                            ))
                        };
                    } else {
                        return Err(format!("Expected digit or ) got {}", c).into());
                    };
                }
            }
        } else {
            break;
        }
    }
    return Ok((prev_expression.ok_or("Expected an expression, got ( or empty string")?, input));
}

fn parse_expression_2(mut input: String) -> Result<(Expression, String), Box<dyn Error>> {
    let res = parse_summation(input)?;
    let l = res.0;
    input = res.1;

    let res = next_char(input);
    let c = res.0;
    input = res.1;

    if let Some('*') = c {
        let res = parse_expression_2(input)?;
        Ok((
            Operation(Box::new(l), Mul, Box::new(res.0)),
            res.1
        ))
    } else {
        if let Some(c) = c {
            input.insert(0, c);
        }
        Ok((l, input))
    }
}

fn parse_summation(mut input: String) -> Result<(Expression, String), Box<dyn Error>> {
    let res = next_char(input);
    let c = res.0.ok_or("Expected a sum, got empty string")?;
    input = res.1;
    let exp = if c.is_digit(10) {
        Ok(Number(parse_digit(c)?))
    } else if c == '(' {
        let res = parse_expression_2(input)?;
        let exp = res.0;
        let res = next_char(res.1);
        input = res.1;
        if res.0 == Some(')') {
            Ok(exp)
        } else {
            Err(format!("Expected ), got {}", res.0.map(|c| c.to_string()).unwrap_or("empty string".to_owned())))
        }
    } else {
        Err(format!("Expected digit or (, got {}", c))
    }?;

    let res = next_char(input);
    let c = res.0;
    input = res.1;
    if let Some('+') = c {
        let res = parse_summation(input)?;
        Ok((
            Operation(Box::new(exp), Add, Box::new(res.0)),
            res.1
        ))
    } else {
        if let Some(c) = c {
            input.insert(0, c);
        }
        Ok((exp, input))
    }
}

fn parse_digit(c: char) -> Result<u32, String> {
    c.to_digit(10).ok_or(format!("Expected digit got {}", c))
}


fn next_char(mut input: String) -> (Option<char>, String) {
    let mut res = None;
    while input.len() > 0 {
        let c = input.remove(0);
        if !c.is_whitespace() {
            res = Some(c);
            break;
        }
    }
    (res, input)
}

enum Expression {
    Number(u32),
    Operation(Box<Expression>, Op, Box<Expression>),
}

impl Expression {
    fn eval(&self) -> i64 {
        match self {
            Expression::Number(num) => *num as i64,
            Expression::Operation(l, op, r) =>
                op.apply(l.eval(), r.eval())
        }
    }
}

enum Op {
    Add,
    Mul,
}

impl Op {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '*' => Some(Self::Mul),
            _ => None
        }
    }

    fn apply(&self, l: i64, r: i64) -> i64 {
        match self {
            Op::Add => l + r,
            Op::Mul => l * r,
        }
    }
}

#[test]
pub fn test_day18() {
    assert!(common::run_test(DAY, &run))
}