use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

trait Contains {
    fn contains(&self, n: u64) -> bool;
}

struct Interval {
    lo: u64,
    hi: u64,
}

impl Interval {
    fn from(s: &str) -> Interval {
        let bounds: Vec<&str> = s.split("-").collect();
        let lo = bounds[0].parse::<u64>().unwrap();
        let hi = bounds[1].parse::<u64>().unwrap();
        Interval { lo, hi }
    }
}

impl Contains for Interval {
    fn contains(&self, n: u64) -> bool {
        self.lo <= n && n <= self.hi
    }
}

struct IntervalGroup {
    name: String,
    intervals: Vec<Interval>,
}

impl Contains for IntervalGroup {
    fn contains(&self, n: u64) -> bool {
        for interval in self.intervals.iter() {
            if interval.contains(n) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let input = parse_input("input.txt");
    let interval_groups = input.0;
    let my_ticket = input.1;
    let tickets = input.2;

    let mut valid_tickets = Vec::new();
    let mut part1_result = 0;
    for ticket in &tickets {
        let mut is_valid = true;
        for num in ticket {
            if !interval_groups.iter().any(|group| group.contains(*num)) {
                part1_result += num;
                is_valid = false;
            }
        }
        if is_valid {
            valid_tickets.push(ticket.clone());
        }
    }
    println!("part 1: {}", part1_result);

    let mut candidates = Vec::new();
    for _ in 0..interval_groups.len() {
        candidates.push((0..interval_groups.len()).collect::<HashSet<usize>>());
    }

    for ticket in &valid_tickets {
        for (num_idx, num) in ticket.iter().enumerate() {
            for (group_idx, group) in interval_groups.iter().enumerate() {
                if !group.contains(*num) {
                    candidates[num_idx].remove(&group_idx);
                }
            }
        }
    }

    let mut assignment = HashMap::new();
    while assignment.len() < interval_groups.len() {
        let element: usize;
        if let Some(hit) = candidates.iter().enumerate().find(|hit| hit.1.len() == 1) {
            let idx = hit.0;
            element = *hit.1.iter().next().unwrap();
            assignment.insert(idx, element);
        } else {
            panic!("error");
        }

        for c in &mut candidates {
            c.remove(&element);
        }
    }

    let mut part2_result = 1;
    for (k, v) in assignment.iter() {
        if interval_groups[*v].name.starts_with("departure") {
            part2_result *= my_ticket[*k];
        }
    }

    println!("part 2: {}", part2_result)
}

fn parse_input(path: &str) -> (Vec<IntervalGroup>, Vec<u64>, Vec<Vec<u64>>) {
    let lines: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();
    let mut line_iter = lines.iter().peekable();

    // parse rules
    let mut interval_groups = Vec::new();
    while line_iter.peek().unwrap().len() != 0 {
        let line = line_iter.next().unwrap();
        let name = line.split(": ").next().unwrap();
        let ranges: Vec<&str> = line
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(" or ")
            .collect();
        interval_groups.push(IntervalGroup {
            name: name.to_string(),
            intervals: vec![Interval::from(ranges[0]), Interval::from(ranges[1])],
        })
    }
    line_iter.next(); // skip empty line

    // parse own ticket
    line_iter.next(); // skip header
    let line = line_iter.next().unwrap();
    let my_ticket: Vec<u64> = line
        .split(",")
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    line_iter.next(); // skip empty line

    // parse other tickets
    let mut tickets = Vec::new();
    line_iter.next(); // skip header
    while line_iter.peek() != None {
        let line = line_iter.next().unwrap();
        let nums: Vec<u64> = line
            .split(",")
            .map(|num| num.parse::<u64>().unwrap())
            .collect();
        tickets.push(nums);
    }

    (interval_groups, my_ticket, tickets)
}
