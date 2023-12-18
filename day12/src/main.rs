use serde_json::Value;

fn main() {
    println!("\n-- Advent of Code 2015 - Day 12 --");

    let input = include_str!("input.txt");
    solve(input.trim());
}

fn solve(input: &str) {
    let json = serde_json::from_str::<Value>(input).unwrap();
    println!("Part 1: {}", sum_numbers(&json, false));
    println!("Part 2: {}", sum_numbers(&json, true));
}

fn sum_numbers(json: &Value, ignore_red: bool) -> i64 {
    match json {
        Value::Number(num) => num.as_i64().unwrap_or(0),
        Value::Array(arr) => arr.iter().map(|v| sum_numbers(v, ignore_red)).sum(),
        Value::Object(obj) => {
            if ignore_red && obj.values().any(|v| v == "red") {
                0
            } else {
                obj.values().map(|v| sum_numbers(v, ignore_red)).sum()
            }
        }
        _ => 0,
    }
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
        let json = serde_json::from_str::<Value>(json).unwrap();
        let result = sum_numbers(&json, false);
        assert_eq!(result, expected);
    }

    #[rstest(
        case(r#"[1,2,3]"#, 6),
        case(r#"[1,{"c":"red","b":2},3]"#, 4),
        case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0),
        case(r#"[1,"red",5]"#, 6)
    )]
    fn test_p2(#[case] json: &str, #[case] expected: i64) {
        let json = serde_json::from_str::<Value>(json).unwrap();
        let result = sum_numbers(&json, true);
        assert_eq!(result, expected);
    }
}
