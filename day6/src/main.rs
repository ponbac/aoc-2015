use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::{separated_pair, tuple},
    IResult,
};

static EXAMPLE_INPUT: &str = r#"
turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
"#;

enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(
            alt((tag("turn on"), tag("turn off"), tag("toggle"))),
            |s: &str| match s {
                "turn on" => Ok(Self::TurnOn),
                "turn off" => Ok(Self::TurnOff),
                "toggle" => Ok(Self::Toggle),
                _ => Err(()),
            },
        )(input)
    }
}

struct Command {
    instruction: Instruction,
    start: (usize, usize),
    end: (usize, usize),
}

impl Command {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                Instruction::parse,
                tag(" "),
                parse_pos,
                tag(" through "),
                parse_pos,
            )),
            |(instruction, _, start, _, end)| Self {
                instruction,
                start,
                end,
            },
        )(input)
    }
}

fn main() {
    println!("\n-- Advent of Code 2015 - Day 6 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    solve(input);
}

fn solve(input: &str) {
    let commands = input
        .trim()
        .lines()
        .map(|line| Command::parse(line).unwrap().1)
        .collect::<Vec<_>>();

    let mut grid = vec![vec![false; 1000]; 1000];
    let mut grid_2: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for command in commands {
        (command.start.0..=command.end.0).for_each(|x| {
            for y in command.start.1..=command.end.1 {
                match command.instruction {
                    Instruction::TurnOn => {
                        grid[x][y] = true;
                        grid_2[x][y] += 1;
                    }
                    Instruction::TurnOff => {
                        grid[x][y] = false;
                        grid_2[x][y] = grid_2[x][y].saturating_sub(1);
                    }
                    Instruction::Toggle => {
                        grid[x][y] = !grid[x][y];
                        grid_2[x][y] += 2;
                    }
                }
            }
        });
    }

    let count = grid.iter().flatten().filter(|&&b| b).count();
    let count_2 = grid_2.iter().flatten().sum::<usize>();

    println!("Part 1: {}", count);
    println!("Part 2: {}", count_2);
}

fn parse_pos(i: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(parse_number, tag(","), parse_number)(i)
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}
