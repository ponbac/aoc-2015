static EXAMPLE_INPUT: &str = r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#;

fn main() {
    println!("\n-- Advent of Code 2015 - Day 8 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    solve(input.trim());
}

fn solve(input: &str) {
    let mut total_chars = 0;
    let mut total_chars_in_memory = 0;
    let mut total_chars_encoded = 0;

    for line in input.lines() {
        let chars = line.chars().count();
        let chars_in_memory = count_chars_in_memory(line);
        let chars_encoded = count_chars_encoded(line);

        total_chars += chars;
        total_chars_in_memory += chars_in_memory;
        total_chars_encoded += chars_encoded;

        println!(
            "Line: {} | Chars: {} | Chars in memory: {} | Chars encoded: {}",
            line, chars, chars_in_memory, chars_encoded
        );
    }

    println!(
        "Total Chars: {} | Total Chars in memory: {} | Total Chars encoded: {}",
        total_chars, total_chars_in_memory, total_chars_encoded
    );
    println!(
        "Difference: {} | Difference encoded: {}",
        total_chars - total_chars_in_memory,
        total_chars_encoded - total_chars
    );
}

fn count_chars_in_memory(line: &str) -> usize {
    let mut chars_in_memory = 0;
    let mut chars = line.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('x') => {
                    chars.next();
                    chars.next();
                }
                Some('\\') | Some('"') => {}
                _ => panic!("Invalid escape sequence"),
            }
        }

        chars_in_memory += 1;
    }

    chars_in_memory - 2
}

fn count_chars_encoded(line: &str) -> usize {
    let mut chars_encoded = 2;

    for c in line.chars() {
        match c {
            '\\' => chars_encoded += 2,
            '"' => chars_encoded += 2,
            _ => chars_encoded += 1,
        }
    }

    chars_encoded
}
