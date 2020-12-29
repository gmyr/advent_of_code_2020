static MODULUS: u64 = 20201227;

fn main() {
    let card_key = 15335876;
    let door_key = 15086442;
    println!("part 1: {}", transform(door_key, loop_size(7, card_key)));
    println!("part 2: Merry Christmas!");
}

fn loop_size(subject_number: u64, key: u64) -> u64 {
    let mut value = 1;
    let mut loop_size = 0;
    while value != key {
        value = (value * subject_number) % MODULUS;
        loop_size += 1;
    }
    loop_size
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % MODULUS;
    }
    return value;
}
