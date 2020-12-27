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

    // generate all possible messages
    let valid_msgs = generate_msgs(0, &rules);

    // count valid messages
    let result = line_iter
        .map(|line| valid_msgs.contains(line))
        .filter(|b| *b)
        .count();
    println!("{}", result);
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

// fn check_msgs(start_id: u64, rules: &HashMap<u64, Vec<Vec<Symbol>>>, msgs: &HashSet<String>) -> u64 {
//     0
// }
