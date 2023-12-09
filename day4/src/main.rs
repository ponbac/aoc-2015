use std::sync::Mutex;

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

static EXAMPLE_INPUT: &str = r#"abcdef"#;

fn main() {
    println!("\n-- Advent of Code 2015 - Day 4 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut i = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, i));
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            println!("Part 1: {}", i);
            break;
        }
        i += 1;
    }
}

fn part2(input: &str) {
    let n_threads = 8;

    let found = Mutex::new(false);
    (0..n_threads)
        .into_par_iter()
        .find_any(|i| {
            let mut j = 0;
            loop {
                if *found.lock().unwrap() {
                    return false;
                }

                let hash = md5::compute(format!("{}{}", input, i + j * n_threads));
                if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
                    println!("Part 2: {}", i + j * n_threads);
                    *found.lock().unwrap() = true;
                    return true;
                }
                j += 1;
            }
        })
        .unwrap();
}
