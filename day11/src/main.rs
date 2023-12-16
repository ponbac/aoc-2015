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
}

fn next_password(password: &str) -> String {
    let mut chars = password.chars().rev();
    let mut next = String::new();

    let mut carry = true;
    while let Some(c) = chars.next() {
        let c = if carry {
            carry = false;
            match c {
                'z' => 'a',
                _ => (c as u8 + 1) as char,
            }
        } else {
            c
        };

        if c == 'i' || c == 'o' || c == 'l' {
            next.push((c as u8 + 1) as char);
            next.push_str(&"a".repeat(chars.count()));
            break;
        }

        if c == 'z' {
            carry = true;
        }

        next.push(c);
    }

    todo!()
}

fn is_valid(password: &str) -> bool {
    todo!()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use rstest::rstest;

//     #[rstest(
//         case("1", "11"),
//         case("11", "21"),
//         case("21", "1211"),
//         case("1211", "111221"),
//         case("111221", "312211")
//     )]
//     fn test_look_and_say(#[case] number: &str, #[case] expected: bool) {
//         assert_eq!(result, expected);
//     }
// }
