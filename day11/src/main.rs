fn main() {
    println!("\n-- Advent of Code 2015 - Day 11 --");

    let input = include_str!("input.txt");
    solve(input.trim());
}

fn solve(input: &str) -> usize {
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
