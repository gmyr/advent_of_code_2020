use std::fs;

fn main() {
    println!("{}", parse_and_check_file(&check_fn_part1));
    println!("{}", parse_and_check_file(&check_fn_part2));
}

fn parse_and_check_file<F>(check_fn: &F) -> usize
where
    F: Fn(char, usize, usize, &str) -> bool,
{
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter(|line| parse_and_check_line(line, check_fn))
        .count()
}

fn parse_and_check_line<F>(line: &str, check_fn: &F) -> bool
where
    F: Fn(char, usize, usize, &str) -> bool,
{
    let line_parts: Vec<&str> = line.split(": ").collect();
    let rule = line_parts[0];
    let password = line_parts[1];
    let rule_parts: Vec<&str> = rule.split(" ").collect();
    let range = rule_parts[0];
    let ch = rule_parts[1].chars().next().unwrap();
    let range_parts: Vec<&str> = range.split("-").collect();
    let a = range_parts[0].parse::<usize>().unwrap();
    let b = range_parts[1].parse::<usize>().unwrap();
    check_fn(ch, a, b, password)
}

fn check_fn_part1(ch: char, min: usize, max: usize, password: &str) -> bool {
    let count = password.chars().filter(|&c| c == ch).count();
    min <= count && count <= max
}

fn check_fn_part2(ch: char, first: usize, second: usize, password: &str) -> bool {
    let chars: Vec<char> = password.chars().collect();
    (chars[first - 1] == ch) ^ (chars[second - 1] == ch)
}
