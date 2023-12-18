use std::collections::HashSet;

fn main() {
    println!("\n-- Advent of Code 2015 - Day 11 --");

    let input = include_str!("input.txt");
    solve(input.trim());
}

fn solve(input: &str) {
    let mut password = input.to_string();
    loop {
        password = next_password(&password);
        if is_valid(&password) {
            break;
        }
    }
    println!("Part 1: {}", password);

    loop {
        password = next_password(&password);
        if is_valid(&password) {
            break;
        }
    }
    println!("Part 2: {}", password);
}

fn next_password(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    for c in chars.iter_mut().rev() {
        match c {
            'z' => *c = 'a',
            _ => {
                *c = ((*c as u8) + 1) as char;
                break;
            }
        }
    }
    chars.into_iter().collect()
}

fn is_valid(password: &str) -> bool {
    if password.contains('i') || password.contains('o') || password.contains('l') {
        return false;
    }

    let mut increasing_straight = false;
    let mut pairs = HashSet::new();

    let chars: Vec<char> = password.chars().collect();
    for i in 0..password.len() - 2 {
        let a = chars[i] as u8;
        let b = chars[i + 1] as u8;
        let c = chars[i + 2] as u8;

        if a + 1 == b && b + 1 == c {
            increasing_straight = true;
        }

        if a == b {
            pairs.insert((a, b));
        } else if b == c {
            pairs.insert((b, c));
        }
    }

    increasing_straight && pairs.len() >= 2
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest(
        case("hijklmmn", false),
        case("abbceffg", false),
        case("abbcegjk", false),
        case("abcdffaa", true),
        case("ghjaabcc", true)
    )]
    fn test_look_and_say(#[case] password: &str, #[case] expected: bool) {
        let result = is_valid(password);
        assert_eq!(result, expected);
    }
}
