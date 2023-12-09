static EXAMPLE_INPUT: &str = r#"2x3x4"#;

fn main() {
    println!("\n-- Advent of Code 2015 - Day 2 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let needed_paper = input
        .lines()
        .map(|line| {
            let mut dims = line.split('x').map(|s| s.parse::<u32>().unwrap());
            let l = dims.next().unwrap();
            let w = dims.next().unwrap();
            let h = dims.next().unwrap();

            let a = l * w;
            let b = w * h;
            let c = h * l;

            let slack = a.min(b).min(c);

            2 * a + 2 * b + 2 * c + slack
        })
        .sum::<u32>();

    println!("Part 1: {}", needed_paper);
}

fn part2(input: &str) {
    let needed_ribbon = input
        .lines()
        .map(|line| {
            let mut dims = line.split('x').map(|s| s.parse::<u32>().unwrap());
            let l = dims.next().unwrap();
            let w = dims.next().unwrap();
            let h = dims.next().unwrap();

            let a = l + l + w + w;
            let b = w + w + h + h;
            let c = h + h + l + l;

            let bow = l * w * h;

            a.min(b).min(c) + bow
        })
        .sum::<u32>();

    println!("Part 2: {}", needed_ribbon);
}
