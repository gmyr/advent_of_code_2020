use std::collections::HashMap;

#[derive(Clone)]
struct Cups {
    current: usize,
    next: HashMap<usize, usize>,
}

impl Cups {
    fn from(ids: &[usize]) -> Cups {
        assert!(ids.len() >= 1);
        let current = ids[0];
        let mut next = HashMap::new();
        for i in 0..ids.len() {
            next.insert(ids[i], ids[(i + 1) % ids.len()]);
        }
        Cups { current, next }
    }

    fn forward(&mut self) {
        self.current = *self.next.get(&self.current).unwrap();
    }

    fn remove_after(&mut self, after: usize) -> usize {
        let v0 = after;
        let v1 = *self.next.get(&after).unwrap();
        let v2 = *self.next.get(&v1).unwrap();
        self.next.insert(v0, v2);
        self.next.remove(&v1);
        v1
    }

    fn take_after(&mut self, after: usize, k: usize) -> Vec<usize> {
        (0..k).map(|_| self.remove_after(after)).collect()
    }

    fn insert_after(&mut self, after: usize, element: usize) {
        let v0 = after;
        let v1 = *self.next.get(&after).unwrap();
        self.next.insert(v0, element);
        self.next.insert(element, v1);
    }

    fn put_after(&mut self, after: usize, elements: Vec<usize>) {
        for element in elements.iter().rev() {
            self.insert_after(after, *element);
        }
    }

    fn to_vec(&self) -> Vec<usize> {
        let mut result = vec![self.current];
        loop {
            let element = *self.next.get(&result.last().unwrap()).unwrap();
            if element == self.current {
                break;
            }
            result.push(element);
        }
        result
    }
}

fn main() {
    let mut ids = vec![1, 5, 7, 6, 2, 3, 9, 8, 4];
    let mut cups = Cups::from(&ids);

    ids.append(&mut (10..=1000000).collect());
    let mut cups2 = Cups::from(&ids);

    simulate(&mut cups, 100);
    cups.current = 1;
    let part1_result = cups
        .to_vec()
        .iter()
        .skip(1)
        .fold(String::new(), |acc, u| acc + &u.to_string());
    println!("part 1: {}", part1_result);

    simulate(&mut cups2, 10000000);
    cups2.current = 1;
    let vec = cups2.to_vec();
    let part2_result = vec[1] * vec[2];
    println!("part 2: {}", part2_result);
}

fn simulate(cups: &mut Cups, steps: usize) {
    let modulus = *cups.next.keys().max().unwrap() + 1;
    for _ in 0..steps {
        step(cups, modulus);
    }
}

fn step(cups: &mut Cups, modulus: usize) {
    let next_3 = cups.take_after(cups.current, 3);
    let mut destination = (cups.current + modulus - 1) % modulus;
    while !cups.next.contains_key(&destination) {
        destination = (destination + modulus - 1) % modulus;
    }
    cups.put_after(destination, next_3);
    cups.forward();
}
