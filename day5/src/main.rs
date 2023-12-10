static EXAMPLE_INPUT: &str = r#"ugknbfddgicrmopn
jchzalrnumimnmhp"#;

fn main() {
    println!("\n-- Advent of Code 2015 - Day 5 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

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
            let mut has_pair = false;
            let mut has_repeat = false;

            let mut prev = ' ';
            let mut prev_prev = ' ';
            for (i, c) in line.chars().enumerate() {
                if i > 1 && !has_pair {
                    let pair = format!("{}{}", prev_prev, prev);
                    let rest = &line[i + 1..];
                    if rest.contains(&pair) {
                        has_pair = true;
                    }
                }

                if i > 0 && !has_repeat && (c == prev_prev && c != prev) && prev_prev != ' ' {
                    has_repeat = true;
                }

                prev_prev = prev;
                prev = c;
            }

            has_pair && has_repeat
        })
        .count();

    println!("Part 2: {}", n_nice);
}
