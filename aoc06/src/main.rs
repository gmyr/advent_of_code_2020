use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let groups: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_owned())
        .collect();

    println!("{:?}", part1(&groups));
    println!("{:?}", part2(&groups));
}

fn part1(groups: &Vec<String>) -> usize {
    groups
        .iter()
        .map(|s| {
            let hs: HashSet<_> = s.chars().filter(|c| c != &'\n').collect();
            hs.len()
        })
        .sum()
}

fn part2(groups: &Vec<String>) -> usize {
    let mut count = 0;
    for group in groups {
        let lines: Vec<&str> = group.lines().collect();
        let group_size = lines.len();
        let mut map = HashMap::new();
        for line in lines {
            for c in line.chars() {
                *map.entry(c).or_insert(0usize) += 1;
            }
        }
        count += map.values().filter(|&&v| v == group_size).count();
    }

    count
}
