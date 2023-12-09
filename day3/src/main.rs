use std::collections::HashSet;

static EXAMPLE_INPUT: &str = r#"^>v<"#;

fn main() {
    println!("\n-- Advent of Code 2015 - Day 3 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut visited_houses = HashSet::from([(0, 0)]);

    let mut pos = (0, 0);
    for char in input.chars() {
        match char {
            '^' => pos.1 += 1,
            '>' => pos.0 += 1,
            'v' => pos.1 -= 1,
            '<' => pos.0 -= 1,
            _ => panic!("Invalid character: {}", char),
        }

        visited_houses.insert(pos);
    }

    println!("Part 1: {}", visited_houses.len());
}

fn part2(input: &str) {
    let mut visited_houses = HashSet::from([(0, 0)]);

    let mut santa_pos = (0, 0);
    let mut robo_pos = (0, 0);
    for (i, char) in input.chars().enumerate() {
        let pos = if i % 2 == 0 {
            &mut santa_pos
        } else {
            &mut robo_pos
        };

        match char {
            '^' => pos.1 += 1,
            '>' => pos.0 += 1,
            'v' => pos.1 -= 1,
            '<' => pos.0 -= 1,
            _ => panic!("Invalid character: {}", char),
        }

        visited_houses.insert(*pos);
    }

    println!("Part 2: {}", visited_houses.len());
}
