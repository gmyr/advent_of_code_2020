use std::fs;

fn main() {
    let mut numbers: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    numbers.sort();
    println!("part1: {}", part1(&numbers));
    println!("part1: {}", part2(&numbers));
}

fn part1(numbers: &Vec<i64>) -> i64 {
    let mut last = 0;
    let mut count_1_jolt = 0;
    let mut count_3_jolt = 1;
    for num in numbers {
        match num - last {
            1 => count_1_jolt += 1,
            2 => (),
            3 => count_3_jolt += 1,
            i => panic!("invalid step {}, sequence of adaptors cannot be used", i),
        }
        last = *num;
    }
    count_1_jolt * count_3_jolt
}

fn part2(numbers: &Vec<i64>) -> i64 {
    let mut tmp = vec![0, 0, 0];
    tmp.extend(numbers.clone());
    let numbers = tmp;

    let mut arr = [0, 0, 1];
    for i in 3..numbers.len() {
        let c = arr[2]
            + (numbers[i] - numbers[i - 2] <= 3) as i64 * arr[1]
            + (numbers[i] - numbers[i - 3] <= 3) as i64 * arr[0];
        arr = [arr[1], arr[2], c];
    }
    arr[2]
}
