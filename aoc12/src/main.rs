use std::fs;

struct Ship {
    x: i64,
    y: i64,
    bearing: i64,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            bearing: 90,
        }
    }

    fn follow_instruction(&mut self, letter: char, number: i64) {
        match letter {
            'N' => self.y += number,
            'S' => self.y -= number,
            'E' => self.x += number,
            'W' => self.x -= number,
            'L' => self.bearing = (self.bearing + 360 - number) % 360,
            'R' => self.bearing = (self.bearing + number) % 360,
            'F' => match self.bearing {
                0 => self.y += number,
                180 => self.y -= number,
                90 => self.x += number,
                270 => self.x -= number,
                _ => panic!("invalid bearing"),
            },
            _ => panic!("unknown instruction"),
        }
    }

    fn move_to(&mut self, waypoint: &Waypoint, times: i64) {
        self.x += times * waypoint.x;
        self.y += times * waypoint.y;
    }

    fn manhatten_distance_from_origin(&self) -> i64 {
        i64::abs(self.x) + i64::abs(self.y)
    }
}

struct Waypoint {
    x: i64,
    y: i64,
}

impl Waypoint {
    fn new() -> Waypoint {
        Waypoint { x: 10, y: 1 }
    }

    fn follow_instruction(&mut self, letter: char, number: i64) {
        let mut number = number;
        match letter {
            'N' => self.y += number,
            'S' => self.y -= number,
            'E' => self.x += number,
            'W' => self.x -= number,
            'L' => {
                while number > 0 {
                    number -= 90;
                    let tmp = self.x;
                    self.x = -self.y;
                    self.y = tmp;
                }
            }
            'R' => {
                while number > 0 {
                    number -= 90;
                    let tmp = self.x;
                    self.x = self.y;
                    self.y = -tmp;
                }
            }
            _ => panic!("unknown instruction"),
        }
    }
}

fn main() {
    let instructions: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();
    println!("part 1: {}", part1(&instructions));
    println!("part 2: {}", part2(&instructions));
}

fn part1(instructions: &Vec<String>) -> i64 {
    let mut ship = Ship::new();
    for instruction in instructions {
        let letter = instruction.chars().next().unwrap();
        let number = instruction[1..].parse::<i64>().unwrap();
        ship.follow_instruction(letter, number);
    }
    ship.manhatten_distance_from_origin()
}

fn part2(instructions: &Vec<String>) -> i64 {
    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new();
    for instruction in instructions {
        let letter = instruction.chars().next().unwrap();
        let number = instruction[1..].parse::<i64>().unwrap();
        match letter {
            'F' => ship.move_to(&waypoint, number),
            _ => waypoint.follow_instruction(letter, number),
        }
    }
    ship.manhatten_distance_from_origin()
}
