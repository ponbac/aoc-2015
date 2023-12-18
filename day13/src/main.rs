fn main() {
    println!("\n-- Advent of Code 2015 - Day 13 --");

    let input = include_str!("input.txt");
    solve(input.trim());
}

fn solve(input: &str) {
    todo!()
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
        todo!()
    }
}
