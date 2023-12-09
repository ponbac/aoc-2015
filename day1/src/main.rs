static EXAMPLE_INPUT: &str = r#"()())"#;

fn main() {
    println!("\n-- Advent of Code 2015 - Day 1 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let floor = input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });

    println!("Part 1: {}", floor);
}

fn part2(input: &str) {
    let mut floor = 0;
    let mut basement = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor == -1 && basement == 0 {
            basement = i + 1;
        }
    }

    println!("Part 2: {}", basement);
}
