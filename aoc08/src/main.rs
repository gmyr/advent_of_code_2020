use std::collections::HashSet;

use std::fs;

struct Action {
    pc: i64,
    acc: i64,
}

fn main() {
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i64 {
    let result = evaluate_program(lines);
    result.1
}

fn part2(lines: &Vec<String>) -> i64 {
    for i in 0..lines.len() {
        let mut lines2 = lines.clone();
        if &lines2[i][..3] == "nop" {
            lines2[i] = String::from("jmp") + &lines2[i][3..];
        } else if &lines2[i][..3] == "jmp" {
            lines2[i] = String::from("nop") + &lines2[i][3..];
        }
        let result = evaluate_program(&lines2);
        if result.0 {
            return result.1;
        }
    }
    panic!("error");
}

fn evaluate_program(lines: &Vec<String>) -> (bool, i64) {
    let mut pc = 0;
    let mut acc = 0;
    let mut visited = HashSet::new();
    visited.insert(pc);

    loop {
        let a = decode_instr(&lines[pc as usize]);
        pc += a.pc;
        acc += a.acc;
        if pc as usize == lines.len() {
            return (true, acc);
        }
        if visited.contains(&pc) {
            return (false, acc);
        }
        visited.insert(pc);
    }
}

fn decode_instr(line: &str) -> Action {
    let op = &line[..3];
    let arg = line[4..].parse::<i64>().unwrap();
    match op {
        "nop" => Action { pc: 1, acc: 0 },
        "acc" => Action { pc: 1, acc: arg },
        "jmp" => Action { pc: arg, acc: 0 },
        _ => panic!("unknown operation"),
    }
}
