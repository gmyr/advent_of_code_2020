use std::fs;

fn main() {
    let data = parse_input("input.txt");

    let mut result = 1;
    result *= trees_hit(&data, 1, |y| 1 * y);
    result *= trees_hit(&data, 1, |y| 3 * y);
    result *= trees_hit(&data, 1, |y| 5 * y);
    result *= trees_hit(&data, 1, |y| 7 * y);
    result *= trees_hit(&data, 2, |y| y / 2);

    println!("{}", result);
}

fn parse_input(filename: &str) -> Vec<Vec<u32>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| if c == '.' { 0 } else { 1 }).collect())
        .collect()
}

fn trees_hit<F>(data: &Vec<Vec<u32>>, y_step: usize, x_fn: F) -> u32
where
    F: Fn(usize) -> usize,
{
    (y_step..data.len())
        .step_by(y_step)
        .map(|y| data[y][x_fn(y) % data[0].len()])
        .sum()
}
