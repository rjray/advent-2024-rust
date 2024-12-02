// Day 1

use std::collections::HashMap;

use itertools::Itertools;

pub fn get_column_numbers(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

pub fn part1(input: String) {
    let (mut left, mut right) = get_column_numbers(&input);
    left.sort_unstable();
    right.sort_unstable();

    let value: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| (l - r).abs())
        .sum();

    println!("{}", value);
}

pub fn part2(input: String) {
    let (left, right) = get_column_numbers(&input);

    let mut freqs = HashMap::new();
    for val in right {
        *freqs.entry(val).or_insert(0) += 1;
    }

    let value: i32 = left.iter().map(|v| freqs.get(v).unwrap_or(&0) * v).sum();

    println!("{}", value);
}
