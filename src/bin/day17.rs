use core::mem;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::iter::Map;
use std::num::TryFromIntError;
use std::slice::Iter;

use aoc2020_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day17";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut d3_space: HashSet<[i64; 3]> = HashSet::new();
    let mut d4_space: HashSet<[i64; 4]> = HashSet::new();
    for (line, y) in input.iter().zip(0..) {
        for (cell, x) in line.chars().zip(0..) {
            match cell {
                '#' => {
                    d3_space.insert([x, y, 0]);
                    d4_space.insert([x, y, 0, 0]);
                    Ok(())
                }
                '.' => Ok(()),
                _ => Err(format!("Unknown character {} in {}", cell, line)),
            }?;
        }
    }

    d3_print(&d3_space);

    let mut old_space = d3_space.clone();
    let mut neighbouring_set = HashSet::new();
    for _ in 0..6 {
        // Death
        for cell in old_space.iter() {
            let mut active_count: u8 = 0;
            for neighbour in neighbours(*cell).iter() {
                if old_space.contains(neighbour) {
                    active_count += 1;
                }
                neighbouring_set.insert(*neighbour);
            }
            if active_count > 3 || active_count < 2 {
                d3_space.remove(cell);
            } else {
                d3_space.insert(*cell);
            }
        }

        // Birth
        for cell in neighbouring_set.iter() {
            if old_space.contains(cell) {
                continue;
            }
            let mut active_count: u8 = 0;
            for neighbour in neighbours(*cell).iter() {
                if old_space.contains(neighbour) {
                    active_count += 1;
                }
            }
            if active_count == 3 {
                d3_space.insert(*cell);
            } else {
                d3_space.remove(cell);
            }
        }
        println!("New iteration\n");
        d3_print(&d3_space);

        old_space = d3_space.clone();
        neighbouring_set.clear();
    }

    let sol1 = old_space.len().to_string();


    d4_print(&d4_space);

    let mut old_space = d4_space.clone();
    let mut neighbouring_set = HashSet::new();
    for _ in 0..6 {
        // Death
        for cell in old_space.iter() {
            let mut active_count: u8 = 0;
            for neighbour in d4_neighbours(*cell).iter() {
                if old_space.contains(neighbour) {
                    active_count += 1;
                }
                neighbouring_set.insert(*neighbour);
            }
            if active_count > 3 || active_count < 2 {
                d4_space.remove(cell);
            } else {
                d4_space.insert(*cell);
            }
        }

        // Birth
        for cell in neighbouring_set.iter() {
            if old_space.contains(cell) {
                continue;
            }
            let mut active_count: u8 = 0;
            if *cell == [1,2,0,-1] {
                print!("");
            };
            for neighbour in d4_neighbours(*cell).iter() {
                if neighbour[1..4] == [2,0,0] || *neighbour == [2,1,0,0] {
                    print!("");
                }
                if old_space.contains(neighbour) {
                    active_count += 1;
                }
            }
            if active_count == 3 {
                d4_space.insert(*cell);
            } else {
                d4_space.remove(cell);
            }
        }
        println!("New iteration\n");
        d4_print(&d4_space);

        old_space = d4_space.clone();
        neighbouring_set.clear();
    }

    let sol2 = old_space.len().to_string();



    Ok([Some(sol1), Some(sol2)])
}

const NEIGHBOUR_DS: [[i64; 3]; 27] = [[-1, -1, -1], [-1, -1, 0], [-1, -1, 1], [-1, 0, -1],
    [-1, 0, 0], [-1, 0, 1], [-1, 1, -1], [-1, 1, 0], [-1, 1, 1], [0, -1, -1], [0, -1, 0], [0, -1, 1],
    [0, 0, -1], [0, 0, 0], [0, 0, 1], [0, 1, -1], [0, 1, 0], [0, 1, 1], [1, -1, -1], [1, -1, 0], [1, -1, 1],
    [1, 0, -1], [1, 0, 0], [1, 0, 1], [1, 1, -1], [1, 1, 0], [1, 1, 1]];

fn neighbours<'a>([x, y, z]: [i64; 3]) -> Vec<[i64; 3]> {
    NEIGHBOUR_DS.iter().filter(|cell| **cell != [0,0,0]).map(|[dz, dy, dx]: &[i64; 3]| [x + dx, y + dy, z + dz]).collect()
}

fn d4_neighbours([x, y, z, w]: [i64; 4]) -> Vec<[i64; 4]> {
    let mut neighbours = Vec::with_capacity(80);
    for dw in [-1, 0, 1].iter() {
        for [dx, dy, dz] in NEIGHBOUR_DS.iter() {
            if *dw != 0 || *dx != 0 || *dy != 0 || *dz != 0 {
                neighbours.push([x + *dx, y + *dy, z + *dz, w + *dw]);
            }
        }
    }
    neighbours
}

fn d3_print(space: &HashSet<[i64; 3]>) {
    let [max, may, maz, mix, miy, miz] = space.iter().fold([0, 0, 0, 0, 0, 0],
                                                           |[max, may, maz, mix, miy, miz], [x, y, z]|
                                                               [max.max(*x), may.max(*y), maz.max(*z), mix.min(*x), miy.min(*y), miz.min(*z)]);
    for z in miz..(maz + 1) {
        println!("Layer {}:", z);
        for y in miy..(may + 1) {
            for x in mix..(max + 1) {
                if z == 0 && y == 0 && x == 0 {
                    if space.contains(&[x, y, z]) {
                        print!("X")
                    } else {
                        print!("x")
                    }
                } else {
                    if space.contains(&[x, y, z]) {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
            }
            println!();
        }
        println!();
    }
}

fn d4_print(space: &HashSet<[i64; 4]>) {
    let [max, may, maz,maw, mix, miy, miz, miw] = space.iter().fold([0, 0, 0, 0, 0, 0, 0, 0],
                                                           |[max, may, maz,maw, mix, miy, miz, miw], [x, y, z,w]|
                                                               [max.max(*x), may.max(*y), maz.max(*z), maw.max(*w),
                                                                   mix.min(*x), miy.min(*y), miz.min(*z), miw.min(*w)]);
    for w in miw..(maw+1) {
        for z in miz..(maz + 1) {
            println!("Layer z={} w={}:", z, w);
            for y in miy..(may + 1) {
                for x in mix..(max + 1) {
                    if y == 0 && x == 0 {
                        if space.contains(&[x, y, z, w]) {
                            print!("X")
                        } else {
                            print!("x")
                        }
                    } else {
                        if space.contains(&[x, y, z, w]) {
                            print!("#")
                        } else {
                            print!(".")
                        }
                    }
                }
                println!();
            }
            println!();
        }
    }
}

#[test]
pub fn test_day17() {
    assert!(common::run_test(DAY, &run))
}