use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::error::Error;
use std::collections::{HashMap, HashSet};
use crate::Direction::{North, East, West, South};

const DAY: &str = "day20";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut tiles = parse(input)?;

    let mut map = HashMap::new();

    let [corner_prod, monster_count] = if build_map(&mut map, &mut tiles, [0,0], Direction::North) {
        let min_x = map.keys().min().unwrap()[0];
        let min_y = map.keys().map(|k| k[1]).min().unwrap();
        let max_x = map.keys().max().unwrap()[0];
        let max_y = map.keys().map(|k| k[1]).max().unwrap();

        let corners = [[min_x, min_y], [min_x, max_y], [max_x, min_y], [max_x, max_x]];

        let corners = corners.iter().map(|k| map.get(k));

        let corner_prod = corners.map(|val| val.unwrap().0.id as u128).product::<u128>().to_string();

        let mut map = combine_map(map);

        print_map(&map);

        let mut delta_monster: Vec<[u32; 2]> = Vec::new();
        let mut monster_width = 0;
        let mut monster_height = 0;

        for (line, y) in SEA_MONSTER_MONSTER.lines().skip(1).zip(0..) {
            for (c, x) in line.chars().zip(0..) {
                if c == '#' {
                    delta_monster.push([x, y]);
                    monster_height = monster_height.max(y+1);
                    monster_width = monster_width.max(x+1);
                }
            }
        }

        let mut monster_count = 0;
        'outer: for _ in 0..2 {
            for _ in 0..4 {
                let mut monster_indices = Vec::new();
                for [x, y] in map.iter() {
                    let mut is_monster = true;
                    for [dx, dy] in delta_monster.iter() {
                        if !map.contains(&[*x + *dx, *y + *dy]) {
                            is_monster = false;
                            break;
                        }
                    };
                    if is_monster {
                        monster_indices.push([*x,*y]);
                        monster_count += 1;
                    };
                };

                delta_monster = rotate_monster(delta_monster, monster_width);
                let temp = [monster_width, monster_height];
                monster_height = temp[0];
                monster_width = temp[1];
            }
            delta_monster = flip_monster(delta_monster, monster_width);
        }

        [corner_prod, map.len().to_string()]
    } else {
        ["-".to_owned(), "-".to_owned()]
    };

    Ok([Some(corner_prod), Some(monster_count)])
}

fn flip_vert([x,y]: [u32;2], width: u32) -> [u32; 2] {
    [width - 1 - x, y]
}

fn rotate_flip([mut x, mut y]: [u32;2], height: u32, width: u32, mutation: Orientation) -> [u32; 2] {
    if mutation.flipped {
        let temp = flip_vert([x,y], width);
        x = temp[0];
        y = temp[1];
    }
    match mutation.direction {
        North => [x,y],
        East  => [height - 1 - y, x],
        South => [width - 1 - x, height - 1 - y],
        West  => [y, width - 1 - x],
    }
}

fn rotate_monster(monster: Vec<[u32; 2]>, width: u32) -> Vec<[u32;2]> {
    monster.into_iter().map(|[x,y]| [y, width - x - 1]).collect()
}

fn flip_monster(monster: Vec<[u32; 2]>, width: u32) -> Vec<[u32;2]> {
    monster.into_iter().map(|[x,y]| [width - x, y]).collect()
}

fn build_map(map: &mut HashMap<[i32;2], (Tile, Orientation)>, tiles: &mut Vec<Tile>, [x, y]: [i32;2], dir: Direction) -> bool {
    let left_dir = dir.rotate_left();
    let back_dir = left_dir.rotate_left();
    let left = left_dir.to_delta();
    let back = back_dir.to_delta();
    let left_constraint = map.get(&[x + left[0], y+ left[1]])
        .map(|(tile, orientation)| {
            let side = orientation.interface(left_dir).to_index();
            tile.keys[side[0]][side[1]]
        });
    let back_constraint = map.get(&[x + back[0], y+ back[1]])
        .map(|(tile, orientation)| {
            let side = orientation.interface(back_dir).to_index();
            tile.keys[side[0]][side[1]]
        });

    let mut checked = HashSet::new();
    loop {
        let mut index: Option<_> = None;
        'tile_for: for (tile, i) in tiles.iter().zip(0..) {
            for orientation in ORIENTATIONS.iter() {
                if !checked.contains(&(tile.id, *orientation)) {
                    checked.insert((tile.id, *orientation));
                    let left = orientation.add(left_dir);
                    let back = left.left().to_index();
                    let left = left.to_index();

                    if (back_constraint.is_none() || back_constraint.unwrap() == tile.keys[back[0]][back[1]])
                        && (left_constraint.is_none() || left_constraint.unwrap() == tile.keys[left[0]][left[1]]) {
                        index = Some((i, *orientation));
                        break 'tile_for;
                    }
                }
            }
        }
        if let Some((index, orientation)) = index {
            let tile = tiles.remove(index);
            map.insert([x,y], (tile, orientation));
            if tiles.len() == 0 {
                return true;
            } else {
                let mut dir = dir;
                if left_constraint.is_none() {
                    dir = dir.rotate_left();
                }
                let [dx, dy] = dir.to_delta();

                if build_map(map, tiles, [x + dx, y + dy], dir) {
                    return true;
                } else {
                    tiles.push(map.remove(&[x, y]).unwrap().0)
                }
            }
        } else {
            return false;
        }
    }
}

fn combine_map(map: HashMap<[i32;2], (Tile, Orientation)>) -> HashSet<[u32; 2]>{
    let [x, y] = map.keys().min().unwrap();
    let (center_tile, _) = map.get(&[0, 0]).unwrap();
    let data = &center_tile.data;
    let tile_width = data[0].len() as u32;
    let tile_height = data.len() as u32;

    let [x_offset, y_offset] = [(-x) * (tile_width as i32), (-y) * (tile_height as i32)];

    let mut char_map = HashSet::new();
    for ([x, y], (tile, orientation)) in map.into_iter() {
        let [x, y] = [(x * (tile_width as i32) + x_offset) as u32, (y * (tile_height as i32) + y_offset) as u32];
        for (line, dy) in tile.data.into_iter().zip(0..) {
            for (c, dx) in line.chars().zip(0..) {
                if c == '#' {
                    let [dx, dy] = rotate_flip([dx, dy], tile_height, tile_width, orientation);
                    char_map.insert([x + dx, y + dy]);
                }
            }
        }
    }
    char_map
}

fn print_map(map: &HashSet<[u32;2]>) {
    let max_x = *map.iter().map(|[x,_]| x).max().unwrap();
    let max_y = *map.iter().map(|[_,y]| y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if map.contains(&[x,y]) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

const ORIENTATIONS: [Orientation; 8] = [
    Orientation {direction: North, flipped: false},
    Orientation {direction: North, flipped: true},
    Orientation {direction: East, flipped: false},
    Orientation {direction: West, flipped: true},
    Orientation {direction: West, flipped: false},
    Orientation {direction: East, flipped: true},
    Orientation {direction: South, flipped: false},
    Orientation {direction: South, flipped: true},
];

const SEA_MONSTER_MONSTER: &str = "
                  #
#    ##    ##    ###
 #  #  #  #  #  #   ";

fn parse(input: &Vec<String>) -> Result<Vec<Tile>,Box<dyn Error>> {
    let mut tiles = Vec::new();
    let mut tile_data: Option<(Tile, u32, String, String)> = None;
    for line in input {
        if let Some((mut tile, row, mut left, mut right)) = tile_data {
            if line == "" {
                return Err(format!("Expected tile line got empty string").into())
            }
            //
            // let mut chars: Vec<char> = line.chars().collect();

            left.push_str(&line[0..1]);
            right.push_str(&line[line.len()-1..line.len()]);

            if row != 0 && row != (line.len() - 1) as u32 {
                tile.data.push(line[1..line.len() - 1].to_owned());
            }

            if row == 0 {
                tile.keys[2] = parse_row_as_binary_numbers(line, true)?;
            }

            if row == (line.len() - 1) as u32 {
                tile.keys[1] = parse_row_as_binary_numbers(&right, true)?;
                tile.keys[0] = parse_row_as_binary_numbers(line, false)?;
                tile.keys[3] = parse_row_as_binary_numbers(&left, false)?;
                tiles.push(tile);
                tile_data = None;
            } else {
                tile_data = Some((tile, row + 1, left, right));
            }
        } else if line != "" {
            let id: u32 = line.strip_prefix("Tile ").and_then(|s| s.strip_suffix(":")).ok_or(format!("Expected Tile <num>:, got {}", line))?
                .parse()?;
            tile_data = Some((Tile {
                id,
                data: Vec::new(),
                keys: [[0,0],[0,0],[0,0],[0,0]],
            },
            0,
            String::new(),
            String::new()));
        }
    }
    Ok(tiles)
}

fn parse_row_as_binary_numbers(row: &str, reverse: bool) -> Result<[u32;2], String> {
    let mut nums = [0, 0];
    let len = (row.len() - 1) as u32;
    for (c, col) in row.chars().zip(0..) {
        if c == '#' {
            nums[0] |= 1 << (len - col);
            nums[1] |= 1 << col;
        } else if c != '.' {
            return Err(format!("Expected '#' or '.' got '{}' at pos {} in string {}", c, col, row))
        }
    }
    if reverse {
        Ok([nums[1], nums[0]])
    } else {
        Ok(nums)
    }
}

#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    data: Vec<String>,
    keys: [[u32; 2]; 4],
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Orientation {
    direction: Direction,
    flipped: bool,
}

impl Orientation {
    fn to_index(self) -> [usize; 2] {
        [self.direction.to_index(), self.flipped as usize]
    }

    fn flip(self) -> Self {
        Orientation {
            direction: self.direction.flip_vert(),
            flipped: !self.flipped,
        }
    }

    fn right(self) -> Self {
        self.add(Direction::East)
    }

    fn left(self) -> Self {
        self.add(Direction::West)
    }

    fn add(self, side: Direction) -> Self {
        if self.flipped {
            Orientation {
                direction: self.direction.subtract(side),
                flipped: self.flipped,
            }
        } else {
            Orientation {
                direction: self.direction.add(side),
                flipped: self.flipped,
            }
        }
    }

    fn interface(self, side: Direction) -> Self {
        let mut orientation = self.add(side.rotate_half());
        orientation.flipped = !orientation.flipped;
        orientation
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    North, East, South, West
}

impl Direction {
    fn to_index(&self) -> usize {
        *self as usize
    }
    
    fn to_delta(&self) -> [i32; 2] {
        match self {
            North => [0,1],
            East => [1,0],
            South => [0,-1],
            West => [-1,0],
        }
    }

    fn from_byte(byte: i8) -> Self {
        match (byte + 4) % 4 {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => panic!("Direction byte has to be between 0 and 4 (excl) got {}", byte)
        }
    }

    fn rotate_left(self) -> Self {
        Self::from_byte((self as i8 - 1))
    }

    fn rotate_right(self) -> Self {
        Self::from_byte((self as i8) + 1)
    }

    fn rotate_half(self) -> Self {
        Self::from_byte((self as i8)+ 2)
    }

    fn add(self, other: Self) -> Self {
        Self::from_byte((self as i8) + (other as i8))
    }
    
    fn subtract(self, other: Self) -> Self {
        Self::from_byte((self as i8) - (other as i8))
    }

    fn flip_vert(self) -> Self {
        Self::from_byte(-(self as i8))
    }
}

#[test]
pub fn test_day20() {
    assert!(common::run_test(DAY, &run))
}