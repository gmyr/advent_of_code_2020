use std::collections::HashMap;
use std::fs;

trait GetSetBit {
    fn get_bit(&self, pos: u64) -> bool;
    fn set_bit(&mut self, pos: u64, val: bool);
}

impl GetSetBit for u64 {
    fn get_bit(&self, pos: u64) -> bool {
        let mask = 1 << pos;
        (*self & mask) > 0
    }

    fn set_bit(&mut self, pos: u64, val: bool) {
        let mask = 1 << pos;
        if val {
            *self = *self | mask;
        } else {
            *self = *self - (*self & mask)
        }
    }
}

struct DockingSystem {
    mem: HashMap<u64, u64>,
    mask: Vec<Option<bool>>,
    v2: bool,
}

impl DockingSystem {
    fn new() -> DockingSystem {
        DockingSystem {
            mem: HashMap::new(),
            mask: Vec::new(),
            v2: false,
        }
    }

    fn new_v2() -> DockingSystem {
        DockingSystem {
            mem: HashMap::new(),
            mask: Vec::new(),
            v2: true,
        }
    }

    fn process_line(&mut self, line: &str) {
        if let Some(postfix) = line.strip_prefix("mask = ") {
            self.mask.clear();
            for c in postfix.chars() {
                let pos = match c {
                    '0' => Some(false),
                    '1' => Some(true),
                    'X' => None,
                    _ => panic!("invalid input: invalid mask"),
                };
                self.mask.push(pos);
            }
            return;
        }

        if let Some(postfix) = line.strip_prefix("mem[") {
            let addr = postfix.split("]").next().unwrap().parse::<u64>().unwrap();
            let val = postfix.split(" = ").last().unwrap().parse::<u64>().unwrap();
            if !self.v2 {
                self.store(addr, val);
            } else {
                self.store_v2(0, 0, addr, val);
            }
            return;
        }

        panic!("invalid input: invalid line")
    }

    fn store(&mut self, addr: u64, val: u64) {
        let mut val = val;
        for (bit_pos, bit_val) in self.mask.iter().enumerate() {
            if let Some(b) = bit_val {
                val.set_bit(35 - bit_pos as u64, *b);
            }
        }
        self.mem.insert(addr, val);
    }

    fn store_v2(&mut self, bit_pos: u64, addr_prefix: u64, addr: u64, val: u64) {
        if bit_pos == 36 {
            self.mem.insert(addr_prefix, val);
            return;
        }

        match self.mask[bit_pos as usize] {
            Some(false) => self.store_v2(
                bit_pos + 1,
                (addr_prefix << 1) + addr.get_bit(35 - bit_pos) as u64,
                addr,
                val,
            ),
            Some(true) => self.store_v2(bit_pos + 1, (addr_prefix << 1) + 1, addr, val),
            None => {
                self.store_v2(bit_pos + 1, (addr_prefix << 1) + 0, addr, val);
                self.store_v2(bit_pos + 1, (addr_prefix << 1) + 1, addr, val);
            }
        };
    }

    fn get_result(&self) -> u64 {
        self.mem.values().sum()
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> u64 {
    let mut ds = DockingSystem::new();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        ds.process_line(&line);
    }
    ds.get_result()
}

fn part2() -> u64 {
    let mut ds = DockingSystem::new_v2();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        ds.process_line(&line);
    }
    ds.get_result()
}
