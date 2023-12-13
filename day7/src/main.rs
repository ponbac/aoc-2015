use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space1},
    combinator::{map, map_res},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use rand::seq::SliceRandom;

static EXAMPLE_INPUT: &str = r#"
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
"#;

fn main() {
    println!("\n-- Advent of Code 2015 - Day 7 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    solve(input);
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Value {
    Wire(String),
    Number(u16),
}

impl Value {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            map(wire_name, Value::Wire),
            map(parse_number, Value::Number),
        ))(i)
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Operation {
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, Value),
    RShift(Value, Value),
    Not(Value),
    Assign(Value),
}

impl Operation {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            parse_and,
            parse_or,
            parse_lshift,
            parse_rshift,
            parse_not,
            parse_assign,
        ))(i)
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Instruction {
    operation: Operation,
    output: String,
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_pair(Operation::parse, tag(" -> "), wire_name),
            |(operation, output)| Self { operation, output },
        )(i)
    }
}

fn solve(input: &str) {
    let mut wire_map: HashMap<String, u16> = HashMap::from_iter(
        input
            .trim()
            .lines()
            .map(|l| l.split_once(" -> ").unwrap())
            .map(|(_, k)| (k.trim().to_string(), 0)),
    );

    // println!("Wire map: {:?}", wire_map);

    let mut instructions: Vec<Instruction> = input
        .trim()
        .lines()
        .map(|l| Instruction::parse(l).unwrap().1)
        .collect();

    for instruction in &instructions {
        println!("{:?}", instruction);
    }

    while !instructions.is_empty() {
        let mut new_instructions = Vec::new();

        instructions.shuffle(&mut rand::thread_rng());
        for instruction in instructions {
            if !process(&instruction, &mut wire_map) {
                // println!("Failed to process: {:?}", instruction);
                new_instructions.push(instruction);
            }
        }

        println!("Instructions left: {}", new_instructions.len());
        instructions = new_instructions;
    }

    println!("Wire map: {:?}", wire_map);
    println!("Wire a: {:?}", wire_map.get("a"));
}

fn process(instruction: &Instruction, wire_map: &mut HashMap<String, u16>) -> bool {
    match &instruction.operation {
        Operation::Assign(n) => {
            let n = match n {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };
            if n != 0 {
                wire_map.insert(instruction.output.clone(), n);
                return true;
            }
        }
        Operation::And(a, b) => {
            let a = match a {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };
            let b = match b {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };
            if a != 0 && b != 0 {
                wire_map.insert(instruction.output.clone(), a & b);
                return true;
            }
        }
        Operation::Or(a, b) => {
            let a = match a {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };
            let b = match b {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };
            if a != 0 && b != 0 {
                wire_map.insert(instruction.output.clone(), a | b);
                return true;
            }
        }
        Operation::LShift(a, n) => {
            let a = match a {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };
            let n = match n {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };
            if a != 0 && n != 0 {
                wire_map.insert(instruction.output.clone(), a << n);
                return true;
            }
        }
        Operation::RShift(a, n) => {
            let a = match a {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };
            let n = match n {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };
            if a != 0 && n != 0 {
                wire_map.insert(instruction.output.clone(), a >> n);
                return true;
            }
        }
        Operation::Not(a) => {
            let a = match a {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };
            if a != 0 {
                wire_map.insert(instruction.output.clone(), !a);
                return true;
            }
        }
    }

    false
}

fn wire_name(i: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(i)
}

fn parse_and(i: &str) -> IResult<&str, Operation> {
    map(
        tuple((Value::parse, tag(" AND "), Value::parse)),
        |(a, _, b)| Operation::And(a, b),
    )(i)
}

fn parse_or(i: &str) -> IResult<&str, Operation> {
    map(
        tuple((Value::parse, space1, tag("OR"), space1, Value::parse)),
        |(a, _, _, _, b)| Operation::Or(a, b),
    )(i)
}

fn parse_lshift(i: &str) -> IResult<&str, Operation> {
    map(
        tuple((Value::parse, space1, tag("LSHIFT"), space1, Value::parse)),
        |(a, _, _, _, n)| Operation::LShift(a, n),
    )(i)
}

fn parse_rshift(i: &str) -> IResult<&str, Operation> {
    map(
        tuple((Value::parse, space1, tag("RSHIFT"), space1, Value::parse)),
        |(a, _, _, _, n)| Operation::RShift(a, n),
    )(i)
}

fn parse_not(i: &str) -> IResult<&str, Operation> {
    map(preceded(tuple((tag("NOT"), space1)), Value::parse), |a| {
        Operation::Not(a)
    })(i)
}

fn parse_assign(i: &str) -> IResult<&str, Operation> {
    map(Value::parse, Operation::Assign)(i)
}

fn parse_number(i: &str) -> IResult<&str, u16> {
    map_res(digit1, |s: &str| s.parse::<u16>())(i)
}
