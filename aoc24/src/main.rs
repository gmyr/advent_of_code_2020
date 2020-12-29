use std::collections::HashSet;
use std::fs;

fn main() {
    let tiles = part1();
    println!("part 1: {}", tiles.len());

    let tiles = part2(tiles);
    println!("part 2: {}", tiles.len());
}

fn part1() -> HashSet<(i64, i64)> {
    let mut tiles = HashSet::new();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let mut instr_iter = line.chars().peekable();
        let mut pos = (0, 0);
        while instr_iter.peek() != None {
            match instr_iter.next().unwrap() {
                'n' => match instr_iter.next().unwrap() {
                    'e' => pos = (pos.0 + 0, pos.1 + 1),
                    'w' => pos = (pos.0 - 1, pos.1 + 1),
                    _ => panic!("invalid input"),
                },
                'e' => pos = (pos.0 + 1, pos.1 + 0),
                's' => match instr_iter.next().unwrap() {
                    'e' => pos = (pos.0 + 1, pos.1 - 1),
                    'w' => pos = (pos.0 + 0, pos.1 - 1),
                    _ => panic!("invalid input"),
                },
                'w' => pos = (pos.0 - 1, pos.1 + 0),
                _ => panic!("invalid input"),
            }
        }
        if tiles.contains(&pos) {
            tiles.remove(&pos);
        } else {
            tiles.insert(pos);
        }
    }
    tiles
}

fn part2(mut tiles: HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    for _ in 0..100 {
        tiles = simulate(tiles);
    }
    tiles
}

fn simulate(tiles: HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut new_tiles = HashSet::new();
    let candidates: Vec<(i64, i64)> = tiles.iter().flat_map(|pos| get_neighbors(pos)).collect();
    for candidate in candidates {
        let num_black_neighbors = num_black_neighbors(&candidate, &tiles);
        if tiles.contains(&candidate) {
            // black tile
            if num_black_neighbors == 1 || num_black_neighbors == 2 {
                new_tiles.insert(candidate);
            }
        } else {
            // white tile
            if num_black_neighbors == 2 {
                new_tiles.insert(candidate);
            }
        }
    }
    new_tiles
}

fn get_neighbors(pos: &(i64, i64)) -> Vec<(i64, i64)> {
    vec![
        (pos.0 + 1, pos.1 + 0), // e
        (pos.0 + 1, pos.1 - 1), // se
        (pos.0 + 0, pos.1 - 1), // sw
        (pos.0 - 1, pos.1 + 0), // w
        (pos.0 - 1, pos.1 + 1), // nw
        (pos.0 + 0, pos.1 + 1), // ne
    ]
}

fn num_black_neighbors(pos: &(i64, i64), tiles: &HashSet<(i64, i64)>) -> usize {
    get_neighbors(pos)
        .iter()
        .filter(|pos| tiles.contains(pos))
        .count()
}
