use std::fs;

fn main() {
    let mut id_min = u64::MAX;
    let mut id_max = 0;
    let mut id_sum = 0u64;
    for id in fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| seat_id(s))
    {
        id_min = u64::min(id_min, id);
        id_max = u64::max(id_max, id);
        id_sum += id;
    }

    let expected_sum = (id_max * (id_max + 1) - (id_min) * (id_min - 1)) / 2;
    let id_missing = expected_sum - id_sum;

    println!("part 1: {}", id_max);
    println!("part 2: {}", id_missing);
}

fn seat_id(s: &str) -> u64 {
    let row_s = &s[..7].replace("F", "0").replace("B", "1");
    let row = u64::from_str_radix(&row_s, 2).unwrap();
    let col_s = &s[7..].replace("L", "0").replace("R", "1");
    let col = u64::from_str_radix(&col_s, 2).unwrap();
    row * 8 + col
}
