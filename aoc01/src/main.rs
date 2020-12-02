use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let expected_value = 2020;

    let input: Vec<i32> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("part1_iter");
    match part1_iter(expected_value, &input) {
        Some((a, b)) => {
            println!("{} + {} = {}", a, b, a + b);
            println!("{} * {} = {}", a, b, a * b);
        }
        _ => println!("no solution"),
    }
    println!();

    println!("part1_indices");
    match part1_indices(expected_value, &input) {
        Some((a, b)) => {
            println!("{} + {} = {}", a, b, a + b);
            println!("{} * {} = {}", a, b, a * b);
        }
        _ => println!("no solution"),
    }
    println!();

    println!("part2");
    match part2(expected_value, &input) {
        Some((a, b, c)) => {
            println!("{} + {} + {} = {}", a, b, c, a + b + c);
            println!("{} * {} * {} = {}", a, b, c, a * b * c);
        }
        _ => println!("no solution"),
    }
}

// inspired by solution by Jason Miller (@imjasonmiller)
fn part1_iter(expected_value: i32, input: &[i32]) -> Option<(i32, i32)> {
    let entries = HashSet::<i32>::from_iter(input.iter().copied());
    entries.iter().find_map(|x| {
        entries
            .get(&(expected_value - x))
            .map(|y| (x.clone(), y.clone()))
    })
}

fn part1_indices(expected_value: i32, input: &[i32]) -> Option<(i32, i32)> {
    let mut entries: Vec<i32> = input.iter().copied().collect();
    entries.sort();
    let mut idx_lo = 0;
    let mut idx_hi = entries.len() - 1;
    while entries[idx_lo] + entries[idx_hi] != expected_value && idx_lo < idx_hi {
        if entries[idx_lo] + entries[idx_hi] < expected_value {
            idx_lo += 1;
        } else {
            idx_hi -= 1;
        }
    }

    if idx_lo < idx_hi {
        Some((entries[idx_lo], entries[idx_hi]))
    } else {
        None
    }
}

fn part2(expected_value: i32, input: &[i32]) -> Option<(i32, i32, i32)> {
    let entries = HashSet::<i32>::from_iter(input.iter().copied());
    entries.iter().find_map(|x| {
        entries.iter().filter(|y| *y != x).find_map(|y| {
            entries
                .get(&(expected_value - x - y))
                .map(|z| (x.clone(), y.clone(), z.clone()))
        })
    })
}
