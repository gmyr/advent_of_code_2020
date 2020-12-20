use std::collections::HashMap;

fn main() {
    let initial_seq = vec![2, 20, 0, 4, 1, 17];
    println!("part 1: {}", game(&initial_seq, 2020));
    println!("part 2: {}", game(&initial_seq, 30000000));
}

fn game(initial_seq: &Vec<usize>, n: usize) -> usize {
    let mut last_occ = HashMap::new();
    for (i, n) in initial_seq.iter().enumerate() {
        if i == initial_seq.len() - 1 {
            break;
        }
        last_occ.insert(*n, i);
    }
    let mut last_num = *initial_seq.last().unwrap();
    for i in initial_seq.len() - 1..n - 1 {
        let new_num = if let Some(last_t) = last_occ.get(&last_num) {
            i - *last_t
        } else {
            0
        };
        last_occ.insert(last_num, i);
        last_num = new_num;
    }
    last_num
}
