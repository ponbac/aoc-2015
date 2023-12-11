use itertools::Itertools;

static EXAMPLE_INPUT: &str = r#"
qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy
xxxyxx
"#;

fn main() {
    println!("\n-- Advent of Code 2015 - Day 5 --");

    let input = EXAMPLE_INPUT;
    // let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let forbidden = ["ab", "cd", "pq", "xy"];

    let n_nice = input
        .lines()
        .filter(|line| {
            let mut n_vowels = 0;
            let mut has_double = false;
            let mut has_forbidden = false;

            let mut prev = ' ';
            for c in line.chars() {
                if vowels.contains(&c) {
                    n_vowels += 1;
                }

                if c == prev {
                    has_double = true;
                }

                let window = format!("{}{}", prev, c);
                if forbidden.contains(&window.as_str()) {
                    has_forbidden = true;
                }

                prev = c;
            }

            n_vowels >= 3 && has_double && !has_forbidden
        })
        .count();

    println!("Part 1: {}", n_nice);
}

fn part2(input: &str) {
    let n_nice = input
        .lines()
        .filter(|line| {
            let pairs = format!("{}{}", line, if line.len() % 2 == 0 { "" } else { " " })
                .chars()
                .tuple_windows()
                // .inspect(|(a, b, c)| println!("{}{}{}", a, b, c))
                .filter(|(a, b, c)| !(a == b && b == c))
                .map(|(a, b, _)| format!("{}{}", a, b))
                // .inspect(|pair| println!("{}", pair))
                .collect::<Vec<_>>();

            let has_pairs = pairs
                .iter()
                .enumerate()
                .any(|(i, pair)| pairs.iter().skip(i + 1).any(|p| p == pair));

            let has_repeat = line.chars().tuple_windows().any(|(a, _, c)| a == c);

            println!("{} {} {}", line, has_pairs, has_repeat);

            has_pairs && has_repeat
        })
        .count();

    println!("Part 2: {}", n_nice);
}
