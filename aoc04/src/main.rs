#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| String::from(s.replace("\n", " ").trim()) + " ")
        .collect();
    println!("part 1: {}", input.iter().filter(|s| is_valid1(s)).count());
    println!("part 2: {}", input.iter().filter(|s| is_valid2(s)).count());
}

fn is_valid1(s: &str) -> bool {
    lazy_static! {
        static ref PATTERNS: Vec<&'static str> =
            vec![r"byr:", r"iyr:", r"eyr:", r"hgt:", r"hcl:", r"ecl:", r"pid:",];
        static ref RES: Vec<Regex> = PATTERNS.iter().map(|re| Regex::new(re).unwrap()).collect();
    }

    for re in RES.iter() {
        match re.find(s) {
            None => return false,
            _ => (),
        }
    }
    true
}

fn is_valid2(s: &str) -> bool {
    lazy_static! {
        static ref RE_BYR: Regex = Regex::new(r"byr:\d{4} ").unwrap();
        static ref RE_IYR: Regex = Regex::new(r"iyr:\d{4} ").unwrap();
        static ref RE_EYR: Regex = Regex::new(r"eyr:\d{4} ").unwrap();
        static ref RE_HGH_CM: Regex = Regex::new(r"hgt:\d{3}cm ").unwrap();
        static ref RE_HGH_IN: Regex = Regex::new(r"hgt:\d{2}in ").unwrap();
        static ref RE_HCL: Regex = Regex::new(r"hcl:#[\da-f]{6} ").unwrap();
        static ref RE_ECL: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth) ").unwrap();
        static ref RE_PID: Regex = Regex::new(r"pid:\d{9} ").unwrap();
    }

    if !check_date_in_range(s, &RE_BYR, 1920, 2002) {
        return false;
    }

    if !check_date_in_range(s, &RE_IYR, 2010, 2020) {
        return false;
    }

    if !check_date_in_range(s, &RE_EYR, 2020, 2030) {
        return false;
    }

    let ok_cm = match RE_HGH_CM.find(s) {
        Some(m) => match m.as_str()[4..7].parse::<i64>() {
            Ok(i) => 150 <= i && i <= 193,
            _ => false,
        },
        None => false,
    };

    let ok_in = match RE_HGH_IN.find(s) {
        Some(m) => match m.as_str()[4..6].parse::<i64>() {
            Ok(i) => 59 <= i && i <= 76,
            _ => false,
        },
        None => false,
    };

    if !ok_cm && !ok_in {
        return false;
    }

    match RE_HCL.find(s) {
        None => return false,
        _ => (),
    }

    match RE_ECL.find(s) {
        None => return false,
        _ => (),
    }

    match RE_PID.find(s) {
        None => return false,
        _ => (),
    }
    true
}

fn check_date_in_range(s: &str, re: &Regex, min: i64, max: i64) -> bool {
    match re.find(s) {
        Some(m) => match m.as_str()[4..8].parse::<i64>() {
            Ok(i) => min <= i && i <= max,
            _ => false,
        },
        None => false,
    }
}
