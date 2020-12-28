use std::collections::HashSet;
use std::fs;

fn main() {
    // create master tiles
    let master_tiles: Vec<MasterTile> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|tile_s| MasterTile::from(&tile_s.lines().collect()))
        .collect();

    // determine side length of arrangement
    let side_length = f64::sqrt(master_tiles.len() as f64) as usize;
    assert!(side_length * side_length == master_tiles.len());

    // print_middle(&master_tiles[0].get_middle());

    // let variant = TileVariant {
    //     master_tile: &master_tiles[0],
    //     flipped: true,
    //     rotation: 3,
    // };
    // print_middle(&variant.get_middle());

    // find arrangment
    let arr: Vec<Vec<TileVariant>> = (0..side_length).map(|_| Vec::new()).collect();
    let result = find_arrangment(0, 0, side_length, arr, master_tiles.iter().collect());
    let arr = if let Some(arr) = result {
        arr
    } else {
        panic!("part 1: no result");
    };

    let num = arr[0][0].master_tile.id
        * arr[0][side_length - 1].master_tile.id
        * arr[side_length - 1][0].master_tile.id
        * arr[side_length - 1][side_length - 1].master_tile.id;
    println!("part 1: {}", num);

    // stitch image together
    let image = stitch_image(&arr);
    let num_pounds = image
        .iter()
        .map(|line| line.chars())
        .flatten()
        .filter(|c| *c == '#')
        .count();
    for flipped in 0..2 {
        for rotation in 0..4 {
            let image = transform_image(&image, flipped == 1, rotation);
            let num_dragons = num_dragons(&image);
            if num_dragons > 0 {
                println!("part 2: {}", num_pounds - 15 * num_dragons);
                return;
            }
        }
    }
    panic!("part 2: no solution found");
}

fn find_arrangment<'a>(
    x: usize,
    y: usize,
    side_length: usize,
    arr: Vec<Vec<TileVariant<'a>>>,
    remaining: HashSet<&'a MasterTile>,
) -> Option<Vec<Vec<TileVariant<'a>>>> {
    if remaining.len() == 0 {
        return Some(arr);
    }
    for master_tile in &remaining {
        for variant in master_tile.get_all_variants() {
            if tile_fits(x, y, &arr, &variant) {
                let next_x = (x + 1) % side_length;
                let next_y = y + (next_x == 0) as usize;
                let mut next_arr = arr.clone();
                next_arr[y].push(variant);
                let mut next_remaining = remaining.clone();
                next_remaining.remove(*master_tile);

                match find_arrangment(next_x, next_y, side_length, next_arr, next_remaining) {
                    Some(arr) => return Some(arr),
                    None => (),
                }
            }
        }
    }
    None
}

fn tile_fits<'a>(x: usize, y: usize, arr: &Vec<Vec<TileVariant<'a>>>, tile: &dyn Tile) -> bool {
    // check top
    if y > 0 && arr[y - 1][x].bottom().chars().rev().collect::<String>() != tile.top() {
        return false;
    }

    // check left
    if x > 0 && arr[y][x - 1].right().chars().rev().collect::<String>() != tile.left() {
        return false;
    }

    true
}

fn stitch_image(arr: &Vec<Vec<TileVariant>>) -> Vec<String> {
    let side_length = arr.len();
    let middle_side_length = arr[0][0].get_middle()[0].len();
    let mut image = Vec::new();
    for y in 0..side_length {
        for x in 0..side_length {
            for t in 0..middle_side_length {
                if x == 0 {
                    image.push(String::from(""));
                }
                image[y * middle_side_length + t].push_str(&arr[y][x].get_middle()[t]);
            }
        }
    }
    image
}

fn num_dragons(image: &Vec<String>) -> usize {
    let dragon = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    let dragon: Vec<Vec<char>> = dragon.iter().map(|line| line.chars().collect()).collect();
    let image: Vec<Vec<char>> = image.iter().map(|line| line.chars().collect()).collect();
    let mut num_dragons = 0;
    for y in 0..image.len() {
        for x in 0..image.len() {
            if sub_image_at(x, y, &image, &dragon) {
                num_dragons += 1;
            }
        }
    }
    num_dragons
}

fn sub_image_at(x: usize, y: usize, image: &Vec<Vec<char>>, sub_image: &Vec<Vec<char>>) -> bool {
    for offset_y in 0..sub_image.len() {
        for offset_x in 0..sub_image[0].len() {
            if y + offset_y >= image.len() || x + offset_x >= image[0].len() {
                return false;
            }

            if sub_image[offset_y][offset_x] == '#' && image[y + offset_y][x + offset_x] != '#' {
                return false;
            }
        }
    }
    true
}

fn transform_image(image: &Vec<String>, flipped: bool, rotation: usize) -> Vec<String> {
    let side_length = image[0].len();
    let mut result = Vec::new();
    for y in 0..side_length {
        result.push(String::new());
        for x in 0..side_length {
            let mut read_pos = match rotation {
                0 => (x, y),
                1 => (y, side_length - 1 - x),
                2 => (side_length - 1 - x, side_length - 1 - y),
                3 => (side_length - 1 - y, x),
                _ => panic!("invalid rotation"),
            };
            if flipped {
                read_pos = (side_length - 1 - read_pos.0, read_pos.1);
            }

            result[y].push(image[read_pos.1].chars().collect::<Vec<char>>()[read_pos.0]);
        }
    }
    result
}

// fn print_image(image: &Vec<String>) {
//     for line in image {
//         println!("{}", line);
//     }
//     println!();
// }

trait Tile {
    fn get_side(&self, i: usize) -> String;
    fn top(&self) -> String {
        self.get_side(0)
    }
    fn right(&self) -> String {
        self.get_side(1)
    }
    fn bottom(&self) -> String {
        self.get_side(2)
    }
    fn left(&self) -> String {
        self.get_side(3)
    }
    fn get_middle(&self) -> Vec<String>;
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct MasterTile {
    id: u64,
    sides: Vec<String>, // top, right, bottom, left
    middle: Vec<String>,
}

impl MasterTile {
    fn from(lines: &Vec<&str>) -> MasterTile {
        let id = lines[0]
            .replace("Tile ", "")
            .replace(":", "")
            .parse()
            .unwrap();
        let lines = &lines[1..];
        let top = lines.first().unwrap().to_string();
        let right = lines
            .iter()
            .map(|line| line.chars().last().unwrap())
            .collect();
        let bottom = lines.last().unwrap().chars().rev().collect();
        let left = lines
            .iter()
            .map(|line| line.chars().next().unwrap())
            .rev()
            .collect();
        let middle = lines[1..lines.len() - 1]
            .iter()
            .map(|line| line[1..line.len() - 1].chars().collect())
            .collect();
        MasterTile {
            id,
            sides: vec![top, right, bottom, left],
            middle,
        }
    }

    fn get_all_variants(&self) -> Vec<TileVariant> {
        let mut variants = Vec::new();
        for flipped in 0..2 {
            for rotation in 0..4 {
                variants.push(TileVariant {
                    master_tile: self,
                    flipped: flipped == 1,
                    rotation,
                });
            }
        }
        assert!(variants.len() == 8);
        variants
    }
}

impl Tile for MasterTile {
    fn get_side(&self, i: usize) -> String {
        self.sides[i].clone()
    }
    fn get_middle(&self) -> Vec<String> {
        self.middle.clone()
    }
}

#[derive(Clone)]
struct TileVariant<'a> {
    master_tile: &'a MasterTile,
    flipped: bool,
    rotation: usize,
}

impl<'a> Tile for TileVariant<'a> {
    fn get_side(&self, i: usize) -> String {
        if self.flipped {
            self.master_tile.sides[(4 - i + self.rotation) % 4]
                .chars()
                .rev()
                .collect()
        } else {
            self.master_tile.sides[(i + 4 - self.rotation) % 4].clone()
        }
    }
    fn get_middle(&self) -> Vec<String> {
        transform_image(&self.master_tile.middle, self.flipped, self.rotation)
    }
}
