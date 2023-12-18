fn main() {
    println!("\n-- Advent of Code 2015 - Day 12 --");

    let input = include_str!("input.txt");
    solve(input.trim());
}

fn solve(input: &str) {
    let sum_1 = sum_numbers(input);
    println!("Part 1: {}", sum_1);
}

fn sum_numbers(input: &str) -> i64 {
    let mut sum = 0;
    let mut number_parts = String::new();
    let mut is_negative = false;

    for c in input.chars() {
        match c {
            '-' => is_negative = true,
            '0'..='9' => number_parts.push(c),
            _ => {
                if !number_parts.is_empty() {
                    let number = number_parts.parse::<i64>().unwrap();
                    if is_negative {
                        sum -= number;
                    } else {
                        sum += number;
                    }

                    number_parts = String::new();
                    is_negative = false;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest(
        case(r#"[1,2,3]"#, 6),
        case(r#"{"a":2,"b":4}"#, 6),
        case(r#"[[[3]]]"#, 3),
        case(r#"{"a":{"b":4},"c":-1}"#, 3),
        case(r#"{"a":[-1,1]}"#, 0),
        case(r#"[-1,{"a":1}]"#, 0),
        case(r#"[]"#, 0)
    )]
    fn test_p1(#[case] json: &str, #[case] expected: i64) {
        let result = sum_numbers(json);
        assert_eq!(result, expected);
    }
}
