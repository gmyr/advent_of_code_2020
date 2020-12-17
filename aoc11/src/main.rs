use std::fs;

fn main() {
    let seats: Vec<Vec<Option<bool>>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| decode(c)).collect())
        .collect();
    println!("part 1: {}", solve(&seats, false, &next_state_part1));
    println!("part 2: {}", solve(&seats, false, &next_state_part2));
}

fn decode(c: char) -> Option<bool> {
    match c {
        '.' => None,
        'L' => Some(false),
        '#' => Some(true),
        _ => panic!("unknown char"),
    }
}

fn encode(pos: Option<bool>) -> char {
    match pos {
        None => '.',
        Some(false) => 'L',
        Some(true) => '#',
    }
}

fn is_occupied(pos: Option<bool>) -> bool {
    match pos {
        None => false,
        Some(b) => b,
    }
}

fn solve<F>(seats: &Vec<Vec<Option<bool>>>, print_maps: bool, trans_fn: &F) -> u64
where
    F: Fn(usize, usize, &Vec<Vec<Option<bool>>>) -> Option<bool>,
{
    let mut seats = seats.clone();
    if print_maps {
        print_seats(&seats);
    }
    loop {
        let last = seats;
        seats = step(&last, trans_fn);
        if print_maps {
            print_seats(&seats);
        }
        if last == seats {
            return count_occupied_seast(&seats);
        }
    }
}

fn print_seats(seats: &Vec<Vec<Option<bool>>>) {
    for row in seats {
        for seat in row {
            print!("{}", encode(*seat));
        }
        println!();
    }
    println!();
}

fn count_occupied_seast(seats: &Vec<Vec<Option<bool>>>) -> u64 {
    seats
        .iter()
        .flatten()
        .map(|pos| is_occupied(*pos) as u64)
        .sum()
}

fn step<F>(seats: &Vec<Vec<Option<bool>>>, trans_fn: &F) -> Vec<Vec<Option<bool>>>
where
    F: Fn(usize, usize, &Vec<Vec<Option<bool>>>) -> Option<bool>,
{
    let mut result = Vec::new();
    for y in 0..seats.len() {
        let mut row = Vec::new();
        for x in 0..seats[0].len() {
            row.push(trans_fn(x, y, seats));
        }
        result.push(row);
    }
    result
}

fn next_state_part1(x: usize, y: usize, seats: &Vec<Vec<Option<bool>>>) -> Option<bool> {
    if seats[y][x] == None {
        return None;
    }
    let mut num_occupied = 0;
    for y_offset in -1..=1 {
        let y_tmp = y as i64 + y_offset;
        if 0 <= y_tmp && y_tmp < seats.len() as i64 {
            for x_offset in -1..=1 {
                let x_tmp = x as i64 + x_offset;
                if (y_offset != 0 || x_offset != 0) && 0 <= x_tmp && x_tmp < seats[0].len() as i64 {
                    num_occupied += is_occupied(seats[y_tmp as usize][x_tmp as usize]) as u64;
                }
            }
        }
    }
    if is_occupied(seats[y][x]) {
        if num_occupied >= 4 {
            return Some(false);
        }
    } else {
        if num_occupied == 0 {
            return Some(true);
        }
    }
    seats[y][x]
}

fn next_state_part2(x: usize, y: usize, seats: &Vec<Vec<Option<bool>>>) -> Option<bool> {
    if seats[y][x] == None {
        return None;
    }
    let mut num_occupied = 0;
    for y_offset in -1..=1 {
        for x_offset in -1..=1 {
            if y_offset != 0 || x_offset != 0 {
                num_occupied += occupied_seat_visible(x, y, x_offset, y_offset, seats) as u64;
            }
        }
    }
    if is_occupied(seats[y][x]) {
        if num_occupied >= 5 {
            return Some(false);
        }
    } else {
        if num_occupied == 0 {
            return Some(true);
        }
    }
    seats[y][x]
}

fn occupied_seat_visible(
    x: usize,
    y: usize,
    x_offset: i64,
    y_offset: i64,
    seats: &Vec<Vec<Option<bool>>>,
) -> bool {
    let mut x_tmp = x as i64 + x_offset;
    let mut y_tmp = y as i64 + y_offset;
    loop {
        if x_tmp < 0 || x_tmp >= seats[0].len() as i64 || y_tmp < 0 || y_tmp >= seats.len() as i64 {
            return false;
        }
        match seats[y_tmp as usize][x_tmp as usize] {
            None => (),
            Some(false) => return false,
            Some(true) => return true,
        }
        x_tmp += x_offset;
        y_tmp += y_offset;
    }
}
