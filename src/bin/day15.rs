use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::collections::HashMap;
use std::ops::RangeFrom;

const DAY: &str = "day15";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut map = HashMap::new();
    let mut last_said = 0;
    let mut turn: u64 = 0;
    for number in input.first().ok_or("Go no input")?.split(",") {
        if turn > 0 {
            map.insert(last_said, turn - 1);
        }
        last_said = number.parse()?;
        turn += 1;
    }

    let mut solution_2020= 999;

    for turn in turn..30_000_000 {
        let temp = last_said;
        last_said = match map.get(&last_said) {
            None => 0,
            Some(prev_turn) => turn - *prev_turn - 1,
        };
        map.insert(temp, turn - 1);
        if turn == 2019 {
            solution_2020 = last_said;
        }
    }

    Ok([Some(solution_2020.to_string()), Some(last_said.to_string())])
}

#[test]
pub fn test_day15() {
    assert!(common::run_test(DAY, &run))
}