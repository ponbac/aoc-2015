use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

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

    let instructions: Vec<Instruction> = input
        .trim()
        .lines()
        .map(|l| Instruction::parse(l).unwrap().1)
        .collect();

    for instruction in &instructions {
        println!("{:?}", instruction);
        process(instruction, &mut wire_map);
    }

    println!("Wire map: {:?}", wire_map);
    println!("Wire a: {:?}", wire_map.get("a"));
}

fn process(instruction: &Instruction, wire_map: &mut HashMap<String, u16>) {
    match &instruction.operation {
        Operation::Assign(n) => {
            let n = match n {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };

            wire_map.insert(instruction.output.clone(), n);
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

            wire_map.insert(instruction.output.clone(), a & b);
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

            wire_map.insert(instruction.output.clone(), a | b);
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

            wire_map.insert(instruction.output.clone(), a << n);
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

            wire_map.insert(instruction.output.clone(), a >> n);
        }
        Operation::Not(a) => {
            let a = match a {
                Value::Wire(w) => *wire_map.get(w).unwrap(),
                Value::Number(n) => *n,
            };

            wire_map.insert(instruction.output.clone(), !a);
        }
    }
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
        tuple((Value::parse, tag(" OR "), Value::parse)),
        |(a, _, b)| Operation::Or(a, b),
    )(i)
}

fn parse_lshift(i: &str) -> IResult<&str, Operation> {
    map(
        tuple((Value::parse, tag(" LSHIFT "), Value::parse)),
        |(a, _, n)| Operation::LShift(a, n),
    )(i)
}

fn parse_rshift(i: &str) -> IResult<&str, Operation> {
    map(
        tuple((Value::parse, tag(" RSHIFT "), Value::parse)),
        |(a, _, n)| Operation::RShift(a, n),
    )(i)
}

fn parse_not(i: &str) -> IResult<&str, Operation> {
    map(preceded(tag("NOT "), Value::parse), Operation::Not)(i)
}

fn parse_assign(i: &str) -> IResult<&str, Operation> {
    map(Value::parse, Operation::Assign)(i)
}

fn parse_number(i: &str) -> IResult<&str, u16> {
    map_res(digit1, |s: &str| s.parse::<u16>())(i)
}
