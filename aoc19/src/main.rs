use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let line: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();
    let mut line_iter = line.iter().peekable();

    // read rules
    let mut rules = HashMap::new();
    while line_iter.peek().unwrap().len() != 0 {
        let line = line_iter.next().unwrap();
        let result = parse_rule(line);
        rules.insert(result.0, result.1);
    }
    line_iter.next(); // skip empty line

    // generate all possible messages for part 1
    let valid_msgs_part1 = generate_msgs(0, &rules);

    // count valid messages for part 1
    let msgs: Vec<String> = line_iter.map(|s| s.to_string()).collect();
    let result_part1 = msgs
        .iter()
        .map(|msg| valid_msgs_part1.contains(msg))
        .filter(|b| *b)
        .count();
    println!("part 1: {}", result_part1);

    let from42 = generate_msgs(42, &rules);
    let from31 = generate_msgs(31, &rules);
    let result_part2 = msgs
        .iter()
        .map(|msg| check_part2(msg, &from42, &from31))
        .filter(|b| *b)
        .count();
    println!("part 2: {}", result_part2);
}

#[derive(Debug)]
enum Symbol {
    NonTerminal(u64),
    Terminal(char),
}

fn parse_rule(line: &str) -> (u64, Vec<Vec<Symbol>>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let id = parts[0].parse::<u64>().unwrap();
    let rhs = parts[1];
    let options: Vec<Vec<Symbol>> = if rhs.contains("|") {
        rhs.split(" | ")
            .map(|list| parse_non_terminal_list(list))
            .collect()
    } else if rhs.contains("\"") {
        vec![vec![Symbol::Terminal(
            rhs.chars().collect::<Vec<char>>()[1],
        )]]
    } else {
        vec![parse_non_terminal_list(rhs)]
    };
    (id, options)
}

fn parse_non_terminal_list(s: &str) -> Vec<Symbol> {
    s.split(" ")
        .map(|s| Symbol::NonTerminal(s.parse::<u64>().unwrap()))
        .collect()
}

fn generate_msgs(start_id: u64, rules: &HashMap<u64, Vec<Vec<Symbol>>>) -> HashSet<String> {
    let rhs = rules.get(&start_id).unwrap();
    if rhs.len() == 1 {
        match rhs[0][0] {
            Symbol::Terminal(c) => {
                let mut hs = HashSet::new();
                hs.insert(String::from(c));
                hs
            }
            Symbol::NonTerminal(_) => generate_from_non_terminals(&rhs[0], rules),
        }
    } else if rhs.len() == 2 {
        let result = generate_from_non_terminals(&rhs[0], rules);
        result
            .union(&generate_from_non_terminals(&rhs[1], rules))
            .map(|s| s.to_string())
            .collect()
    } else {
        panic!("invalid rhs");
    }
}

fn generate_from_non_terminals(
    non_terminals: &Vec<Symbol>,
    rules: &HashMap<u64, Vec<Vec<Symbol>>>,
) -> HashSet<String> {
    let results: Vec<HashSet<String>> = non_terminals
        .iter()
        .map(|sym| {
            if let Symbol::NonTerminal(id) = sym {
                generate_msgs(*id, rules)
            } else {
                panic!("invalid rhs")
            }
        })
        .collect();
    let mut hs = HashSet::new();
    hs.insert(String::from(""));
    for set in &results {
        let mut tmp = HashSet::new();
        for prefix in hs {
            for postfix in set {
                let mut clone = prefix.clone();
                clone.push_str(postfix);
                tmp.insert(clone);
            }
        }
        hs = tmp;
    }
    hs
}

fn check_part2(msg: &str, from42: &HashSet<String>, from31: &HashSet<String>) -> bool {
    let mut msg = msg.to_string();

    // count matching tails
    let mut num_tails = 0;
    loop {
        let mut done = true;
        for pattern in from31 {
            match msg.strip_suffix(pattern) {
                Some(prefix) => {
                    msg = prefix.to_string();
                    num_tails += 1;
                    done = false;
                    break;
                }
                None => (),
            }
        }
        if done {
            break;
        }
    }

    let mut num_heads = 0;
    while msg.len() != 0 {
        let mut found_match = false;
        for pattern in from42 {
            match msg.strip_prefix(pattern) {
                Some(suffix) => {
                    msg = suffix.to_string();
                    num_heads += 1;
                    found_match = true;
                    break;
                }
                None => (),
            }
        }
        if !found_match {
            return false;
        }
    }

    num_tails >= 1 && num_heads - num_tails >= 1
}
