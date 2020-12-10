use std::collections::HashSet;
use std::collections::LinkedList;
use std::fs;

fn main() {
    let numbers: Vec<i64> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let result1 = part1(&numbers);
    let result2 = part2(&numbers, result1);
    println!("{}", result1);
    println!("{}", result2);
}

fn part1(numbers: &Vec<i64>) -> i64 {
    let mut preds = LinkedList::new();
    let mut hs = HashSet::new();

    let mut it = numbers.iter();
    for _ in 0..25 {
        match it.next() {
            Some(v) => {
                hs.insert(v);
                preds.push_back(v);
            }
            None => panic!("nope!"),
        }
    }

    for num in it {
        let mut valid = false;
        for a in &hs {
            let b = num - *a;
            if hs.contains(&b) {
                valid = true;
                break;
            }
        }
        if !valid {
            return *num;
        }
        hs.remove(preds.pop_front().unwrap());
        hs.insert(num);
        preds.push_back(num);
    }

    panic!("nope!")
}

fn part2(numbers: &Vec<i64>, desired_sum: i64) -> i64 {
    let mut low = 0;
    let mut high = 1;
    let mut sum = numbers[low] + numbers[high];
    loop {
        if sum == desired_sum {
            break;
        } else if sum < desired_sum {
            high += 1;
            sum += numbers[high];
        } else {
            // sum > desired_sum
            sum -= numbers[low];
            low += 1;
        }
    }

    let min = numbers[low..high + 1].iter().min().unwrap();
    let max = numbers[low..high + 1].iter().max().unwrap();
    min + max
}
