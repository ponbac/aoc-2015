fn main() {
    println!("\n-- Advent of Code 2015 - Day 10 --");

    let input = include_str!("input.txt");
    solve(input.trim());
}

fn solve(input: &str) -> usize {
    let mut result = input.trim().to_string();
    for _ in 0..40 {
        result = look_and_say(&result);
    }
    println!("Part 1: {}", result.len());

    for _ in 0..10 {
        result = look_and_say(&result);
    }
    println!("Part 2: {}", result.len());

    result.len()
}

fn look_and_say(input: &str) -> String {
    let mut result = String::new();

    let mut chars = input.chars();
    let mut current_char = chars.next().unwrap();
    let mut current_count = 1;
    for c in chars {
        if c == current_char {
            current_count += 1;
        } else {
            result.push_str(&format!("{}{}", current_count, current_char));
            current_char = c;
            current_count = 1;
        }
    }
    result.push_str(&format!("{}{}", current_count, current_char));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest(
        case("1", "11"),
        case("11", "21"),
        case("21", "1211"),
        case("1211", "111221"),
        case("111221", "312211")
    )]
    fn test_look_and_say(#[case] number: &str, #[case] expected: &str) {
        let result = look_and_say(number);
        assert_eq!(result, expected);
    }
}
