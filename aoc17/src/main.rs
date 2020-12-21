use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl Vec3<i64> {
    fn closed_neighborhood(&self) -> Vec<Vec3<i64>> {
        let mut neighbors = self.open_neighborhood();
        neighbors.push(Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        });
        neighbors
    }

    fn open_neighborhood(&self) -> Vec<Vec3<i64>> {
        let mut neighbors = Vec::new();
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                for z_offset in -1..=1 {
                    if x_offset != 0 || y_offset != 0 || z_offset != 0 {
                        neighbors.push(Vec3 {
                            x: self.x + x_offset,
                            y: self.y + y_offset,
                            z: self.z + z_offset,
                        });
                    }
                }
            }
        }
        neighbors
    }
}

fn main() {
    let mut active_cubes = parse_input("input.txt");
    for _ in 0..6 {
        active_cubes = simulate_step(&active_cubes);
    }
    println!("part 1: {}", active_cubes.len());
}

fn parse_input(path: &str) -> HashSet<Vec3<i64>> {
    let mut active_cubes = HashSet::new();
    for (y, line) in fs::read_to_string(path).unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    active_cubes.insert(Vec3 {
                        x: x as i64,
                        y: y as i64,
                        z: 0,
                    });
                }
                '.' => (),
                _ => panic!("invalid input"),
            }
        }
    }
    active_cubes
}

fn simulate_step(active_cubes: &HashSet<Vec3<i64>>) -> HashSet<Vec3<i64>> {
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
            .filter(|c| active_cubes.contains(c))
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
