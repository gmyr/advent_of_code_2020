use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}

fn part1() -> i64 {
    let graph = create_graph_backwards();
    let mut count = -1;
    let mut visited = HashSet::new();
    let mut queued = vec!["shiny gold"];
    while queued.len() != 0 {
        let node = queued.pop().unwrap();
        if !visited.contains(&node) {
            count += 1;
            match graph.get(node) {
                Some(targets) => {
                    for target in targets {
                        queued.push(target);
                    }
                }
                None => (),
            }
            visited.insert(node);
        }
    }
    count
}

fn create_graph_backwards() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let splits: Vec<&str> = line.split(" contain ").collect();
        let target = splits[0].replace(" bags", "");
        let right = splits[1];
        let sources: Vec<String> = right
            .split(", ")
            .map(|s| {
                s[2..]
                    .replace(" bags", "")
                    .replace(" bag", "")
                    .replace(".", "")
            })
            .collect();
        for source in sources {
            map.entry(source).or_insert(Vec::new()).push(target.clone());
        }
    }
    map
}

fn part2() -> i64 {
    let graph = create_graph_forwards();
    num_bags_rec(&graph, &"shiny gold")
}

fn num_bags_rec(graph: &HashMap<String, Vec<(i64, String)>>, node: &str) -> i64 {
    match graph.get(node) {
        None => 0,
        Some(targets) => targets
            .iter()
            .map(|target| target.0 * num_bags_rec(graph, &target.1) + target.0)
            .sum(),
    }
}

fn create_graph_forwards() -> HashMap<String, Vec<(i64, String)>> {
    let mut map = HashMap::new();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let splits: Vec<&str> = line.split(" contain ").collect();
        let source = splits[0].replace(" bags", "");
        let right = splits[1];
        if right == "no other bags." {
            continue;
        }

        let targets: Vec<(i64, String)> = right
            .split(", ")
            .map(|s| {
                (
                    s[..1].parse::<i64>().unwrap(),
                    s[2..]
                        .replace(" bags", "")
                        .replace(" bag", "")
                        .replace(".", ""),
                )
            })
            .collect();
        for target in targets {
            map.entry(source.clone())
                .or_insert(Vec::new())
                .push(target.clone());
        }
    }
    map
}
