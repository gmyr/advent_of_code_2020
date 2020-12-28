use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let decks = parse_input("input.txt");
    println!("part 1: {}", score_decks(&combat(decks.clone())));
    println!("part 2: {}", score_decks(&recursive_combat(decks.clone())));
}

fn parse_input(path: &str) -> (VecDeque<u64>, VecDeque<u64>) {
    let parts: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();
    (
        parse_player(&parts[0].lines().collect()),
        parse_player(&parts[1].lines().collect()),
    )
}

fn parse_player(lines: &Vec<&str>) -> VecDeque<u64> {
    lines[1..]
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn score_decks(decks: &(VecDeque<u64>, VecDeque<u64>)) -> u64 {
    if decks.0.is_empty() {
        score_deck(&decks.1)
    } else {
        score_deck(&decks.0)
    }
}

fn score_deck(deck: &VecDeque<u64>) -> u64 {
    deck.iter()
        .enumerate()
        .map(|(i, card)| (deck.len() - i) as u64 * card)
        .sum()
}

fn combat(mut decks: (VecDeque<u64>, VecDeque<u64>)) -> (VecDeque<u64>, VecDeque<u64>) {
    while !decks.0.is_empty() && !decks.1.is_empty() {
        let cards = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());
        if cards.0 > cards.1 {
            decks.0.push_back(cards.0);
            decks.0.push_back(cards.1);
        } else {
            decks.1.push_back(cards.1);
            decks.1.push_back(cards.0);
        }
    }
    decks
}

fn recursive_combat(mut decks: (VecDeque<u64>, VecDeque<u64>)) -> (VecDeque<u64>, VecDeque<u64>) {
    let mut mem = HashSet::new();
    while !decks.0.is_empty() && !decks.1.is_empty() {
        if mem.contains(&decks) {
            // decks occured in previous round, player 0 wins
            decks.0.append(&mut decks.1);
            return decks;
        }
        mem.insert(decks.clone());
        let cards = (decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap());
        if cards.0 as usize > decks.0.len() || cards.1 as usize > decks.1.len() {
            // at least one player doesn't have enough cards in their deck
            if cards.0 > cards.1 {
                decks.0.push_back(cards.0);
                decks.0.push_back(cards.1);
            } else {
                decks.1.push_back(cards.1);
                decks.1.push_back(cards.0);
            }
        } else {
            // both players have enough cards in their decks
            let new_decks: (VecDeque<u64>, VecDeque<u64>) = (
                decks.0.iter().take(cards.0 as usize).cloned().collect(),
                decks.1.iter().take(cards.1 as usize).cloned().collect(),
            );
            let new_decks = recursive_combat(new_decks);
            if new_decks.1.is_empty() {
                decks.0.push_back(cards.0);
                decks.0.push_back(cards.1);
            } else {
                decks.1.push_back(cards.1);
                decks.1.push_back(cards.0);
            }
        }
    }
    decks
}
