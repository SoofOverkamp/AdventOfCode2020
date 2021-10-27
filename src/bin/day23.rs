use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::str::ParseBoolError;
use std::ops::Index;
use std::num::ParseIntError;
use std::io;
use std::io::Write;
use std::collections::HashMap;

const DAY: &str = "day23";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut cups: Vec<usize> = input.into_iter().next().unwrap()
        .chars().map(|s| s.to_digit(10).unwrap() as usize)
        .collect();

    let len = cups.len();

    let res_1= {
        let mut cups = cups.clone();
        // do_moves(&mut cups, 100)?;
        let start_i = cups.iter().position(|c| *c == 1)
            .ok_or(format!("Expected cups ({:?}) to contain cup 1", cups))?;

        let mut res = String::new();
        for i in 1..len {
            res.push_str(cups[(start_i + i) % len].to_string().as_str());
        }
        res
    };

    let mut res_2= {
        let mut cups = cups.clone();
        cups.extend((len+1)..=1_000_000);
        do_moves(&mut cups, 10_000_000)?;
        let start_i = cups.iter().position(|c| *c == 1)
            .ok_or(format!("Expected cups ({:?}) to contain cup 1", cups))?;

        (cups[(start_i + 1) % len] * cups[(start_i + 2) % len]).to_string()
    };
                      
    Ok([Some(res_1), Some(res_2)])
}

fn do_moves(cups: &mut Vec<usize>, n: usize) -> Result<(), String> {
    let len = cups.len();

    let mut current_cup_index = 0;

    let mut index_cache = HashMap::new();

    let mut cache_misses = 0;

    for _ in 0..n {
        // println!("cups: {:?}", cups);
        // println!("current index: {}", current_cup_index);

        let in_claw = [
            cups[(current_cup_index + 1) % len],
            cups[(current_cup_index + 2) % len],
            cups[(current_cup_index + 3) % len],
        ];

        // println!("in claw: {:?}", in_claw);

        let mut destination_cup = cups[current_cup_index];
        loop  {
            destination_cup = (destination_cup + len - 2) % len + 1;
            if !in_claw.contains(&destination_cup) {
                break;
            }
        }

        let mut destination_index =  if let Some(i) = index_cache.get(&destination_cup) {
            *i
        } else {
            cache_misses += 1;
            cups.iter().position(|c| *c == destination_cup)
                .ok_or(format!("Tried to get cup {} but it was not there. current i: {}", destination_cup, current_cup_index))?
        };

        index_cache.clear();

        // println!("destination: {} at {}", destination_cup, destination_index);

        for i in 0..3 {
            let remove_i = (current_cup_index + 1) % len;
            if remove_i < destination_index {
                destination_index -= 1;
                index_cache = index_cache.into_iter().map(|(v, i)| (v, i-1)).collect();
            }
            let removed = cups.remove(remove_i);
            let dest_i = destination_index + i + 1;
            index_cache.insert(removed, dest_i);
            cups.insert(dest_i, removed);
            if destination_index < remove_i {
                currents_cup_index = (current_cup_index + 1) % len;
            }
        }
        current_cup_index = (current_cup_index + 1) % len;

        if current_cup_index % 10_000 == 0 {
            println!("{}", current_cup_index);
        }

        // println!()
    }
    Ok(())
}

#[test]
pub fn test_day23() {
    assert!(common::run_test(DAY, &run))
}