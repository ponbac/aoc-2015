use itertools::Itertools;

static EXAMPLE_INPUT: &str = r#"
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
"#;

fn main() {
    println!("\n-- Advent of Code 2015 - Day 9 --");

    // let input = EXAMPLE_INPUT;
    let input = include_str!("input.txt");

    solve(input.trim());
}

fn solve(input: &str) {
    let paths: Vec<(&str, &str, usize)> = input
        .lines()
        .map(|line| {
            let (cities, distance) = line.split_once(" = ").unwrap();
            let (city1, city2) = cities.split_once(" to ").unwrap();
            (city1, city2, distance.parse::<usize>().unwrap())
        })
        .collect();

    let cities: Vec<&str> = paths
        .iter()
        .flat_map(|(city1, city2, _)| vec![*city1, *city2])
        .unique()
        .collect();

    let mut distances: Vec<usize> = Vec::new();
    for permutation in cities.iter().permutations(cities.len()) {
        let mut distance = 0;
        for i in 0..permutation.len() - 1 {
            let city1 = permutation[i];
            let city2 = permutation[i + 1];
            let path = paths
                .iter()
                .find(|(p_city1, p_city2, _)| {
                    (p_city1 == city1 && p_city2 == city2) || (p_city1 == city2 && p_city2 == city1)
                })
                .unwrap();
            distance += path.2;
        }
        distances.push(distance);
    }

    let shortest_distance = distances.iter().min().unwrap();
    println!("Shortest distance: {}", shortest_distance);
    let longest_distance = distances.iter().max().unwrap();
    println!("Longest distance: {}", longest_distance);
}
