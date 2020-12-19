use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();
    let arrival_time = input[0].parse::<u64>().unwrap();
    let bus_tuples: Vec<(u64, u64)> = input[1]
        .split(",")
        .enumerate()
        .filter(|id| id.1 != "x")
        .map(|id| (id.0 as u64, id.1.parse::<u64>().unwrap()))
        .collect();

    println!("part 1: {}", part1(arrival_time, &bus_tuples));
    println!("part 2: {}", part2(&bus_tuples, false));
}

fn part1(arrival_time: u64, bus_tuples: &Vec<(u64, u64)>) -> u64 {
    let mut best_bus_id = 0;
    let mut best_departure_time = u64::MAX;
    for tuple in bus_tuples {
        let id = tuple.1;
        let tmp = id * (arrival_time / id);
        if tmp == arrival_time {
            return 0;
        }
        let departure_time = tmp + id;
        if departure_time < best_departure_time {
            best_departure_time = departure_time;
            best_bus_id = id;
        }
    }
    best_bus_id * (best_departure_time - arrival_time)
}

fn part2(bus_tuples: &Vec<(u64, u64)>, debug: bool) -> u64 {
    if debug {
        println!("{:?}", bus_tuples);
    }
    let mut start_a = mod_inv(bus_tuples[0].0, bus_tuples[0].1);
    let mut inc_a = bus_tuples[0].1;
    for i in 1..bus_tuples.len() {
        if debug {
            println!("{}", i);
        }
        let start_b = mod_inv(bus_tuples[i].0, bus_tuples[i].1);
        let inc_b = bus_tuples[i].1;
        let ecd = earliest_common_departure(start_a, start_b, inc_a, inc_b);
        if debug {
            println!(
                "ecd({}, {}, {}, {}) = {}",
                start_a, start_b, inc_a, inc_b, ecd
            );
        }
        let lcm = lcm(inc_a, inc_b);
        if debug {
            println!("lcm({}, {}) = {}", inc_a, inc_b, lcm);
        }
        start_a = ecd;
        inc_a = lcm;
        if debug {
            println!();
        }
    }
    start_a
}

fn earliest_common_departure(start_a: u64, start_b: u64, inc_a: u64, inc_b: u64) -> u64 {
    let mut a = start_a;
    let mut b = start_b;
    while a != b {
        if a < b {
            a += ((b - a) / inc_a) * inc_a;
            if a < b {
                a += inc_a;
            }
        } else {
            b += ((a - b) / inc_b) * inc_b;
            if b < a {
                b += inc_b;
            }
        }
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while a != b {
        if a > b {
            a -= ((a - b) / b) * b;
            if a > b {
                a -= b;
            }
        } else {
            b -= ((b - a) / a) * a;
            if b > a {
                b -= a;
            }
        }
    }
    a
}

fn mod_inv(a: u64, b: u64) -> u64 {
    let a = -(a as i64);
    let b = b as i64;
    (((a % b) + b) % b) as u64
}
