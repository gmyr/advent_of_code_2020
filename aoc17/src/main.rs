use std::collections::HashSet;
use std::fs;

fn main() {
    println!("part 1: {}", simulate(3, 6).len());
    println!("part 2: {}", simulate(4, 6).len());
}

trait HasNeighbors: Sized {
    fn closed_neighborhood(&self) -> Vec<Self>;
    fn open_neighborhood(&self) -> Vec<Self>;
}

impl HasNeighbors for Vec<i64> {
    fn closed_neighborhood(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        neighbors.push(Vec::new());
        for element in self.iter() {
            let mut new_neighbors = Vec::new();
            for neighbor in &neighbors {
                for offset in -1..=1 {
                    let mut clone = neighbor.clone();
                    clone.push(element + offset);
                    new_neighbors.push(clone);
                }
            }
            neighbors = new_neighbors;
        }
        neighbors
    }

    fn open_neighborhood(&self) -> Vec<Self> {
        let mut neighbors = self.closed_neighborhood();
        neighbors.swap_remove(neighbors.len() / 2);
        neighbors
    }
}

fn simulate(rank: u64, steps: u64) -> HashSet<Vec<i64>> {
    let mut active_cubes = parse_input("input.txt", rank);
    for _ in 0..steps {
        active_cubes = simulate_step(&active_cubes);
    }
    active_cubes
}

fn parse_input(path: &str, rank: u64) -> HashSet<Vec<i64>> {
    let mut active_cubes = HashSet::new();
    for (y, line) in fs::read_to_string(path).unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    let mut cube = vec![x as i64, y as i64];
                    for _ in 2..rank {
                        cube.push(0);
                    }
                    active_cubes.insert(cube);
                }
                '.' => (),
                _ => panic!("invalid input"),
            }
        }
    }
    active_cubes
}

fn simulate_step(active_cubes: &HashSet<Vec<i64>>) -> HashSet<Vec<i64>> {
    let mut candidates = HashSet::new();
    for cube in active_cubes {
        for neighbor in cube.closed_neighborhood() {
            candidates.insert(neighbor);
        }
    }

    let mut new_active_cubes = HashSet::new();
    for cube in candidates {
        let num_active_neighbors = cube
            .open_neighborhood()
            .iter()
            .filter(|c| active_cubes.contains(*c))
            .count();
        if active_cubes.contains(&cube) {
            if num_active_neighbors == 2 || num_active_neighbors == 3 {
                new_active_cubes.insert(cube);
            }
        } else {
            if num_active_neighbors == 3 {
                new_active_cubes.insert(cube);
            }
        }
    }
    new_active_cubes
}
