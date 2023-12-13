use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space1},
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

#[derive(Debug)]
enum Operation {
    And(String, String),
    Or(String, String),
    LShift(String, usize),
    RShift(String, usize),
    Not(String),
    Assign(u16),
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

#[derive(Debug)]
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
    }

    for instruction in instructions {
        match instruction.operation {
            Operation::Assign(n) => {
                wire_map.insert(instruction.output, n);
            }
            Operation::And(a, b) => {
                let a = wire_map.get(&a).unwrap();
                let b = wire_map.get(&b).unwrap();
                wire_map.insert(instruction.output, a & b);
            }
            Operation::Or(a, b) => {
                let a = wire_map.get(&a).unwrap();
                let b = wire_map.get(&b).unwrap();
                wire_map.insert(instruction.output, a | b);
            }
            Operation::LShift(a, n) => {
                let a = wire_map.get(&a).unwrap();
                wire_map.insert(instruction.output, a << n);
            }
            Operation::RShift(a, n) => {
                let a = wire_map.get(&a).unwrap();
                wire_map.insert(instruction.output, a >> n);
            }
            Operation::Not(a) => {
                let a = wire_map.get(&a).unwrap();
                wire_map.insert(instruction.output, !a);
            }
        }
    }

    println!("Wire map: {:?}", wire_map);
}

fn wire_name(i: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(i)
}

fn number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse::<usize>)(i)
}

fn parse_and(i: &str) -> IResult<&str, Operation> {
    map(
        tuple((wire_name, space1, tag("AND"), space1, wire_name)),
        |(a, _, _, _, b)| Operation::And(a, b),
    )(i)
}

fn parse_or(i: &str) -> IResult<&str, Operation> {
    map(
        tuple((wire_name, space1, tag("OR"), space1, wire_name)),
        |(a, _, _, _, b)| Operation::Or(a, b),
    )(i)
}

fn parse_lshift(i: &str) -> IResult<&str, Operation> {
    map(
        tuple((wire_name, space1, tag("LSHIFT"), space1, number)),
        |(a, _, _, _, n)| Operation::LShift(a, n),
    )(i)
}

fn parse_rshift(i: &str) -> IResult<&str, Operation> {
    map(
        tuple((wire_name, space1, tag("RSHIFT"), space1, number)),
        |(a, _, _, _, n)| Operation::RShift(a, n),
    )(i)
}

fn parse_not(i: &str) -> IResult<&str, Operation> {
    map(preceded(tuple((tag("NOT"), space1)), wire_name), |a| {
        Operation::Not(a)
    })(i)
}

fn parse_assign(i: &str) -> IResult<&str, Operation> {
    map(parse_number, Operation::Assign)(i)
}

fn parse_number(i: &str) -> IResult<&str, u16> {
    map_res(digit1, |s: &str| s.parse::<u16>())(i)
}