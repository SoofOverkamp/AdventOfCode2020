use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::num::ParseIntError;
use std::time::Instant;

const DAY: &str = "day13";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut input = input.iter();
    let timestamp = input.next().ok_or("Expected 2 input lines got 0")?;
    let ids = input.next().ok_or("Expected 2 input lines got 1")?;

    let timestamp: u64 = timestamp.parse()?;

    let ids: Result<Vec<(u64, u64)>, ParseIntError> = ids.split(",")
        .zip(0..)
        .filter(|(id, i)| *id != "x")
        .map(|(id, i)| id.parse::<u64>().map(|id| (i, id)))
        .collect();

    let ids = ids?;

    return Ok([Some(run_one(&ids, timestamp).to_string()), Some(run_two(&ids).to_string())]);
}

fn run_one(ids: &Vec<(u64, u64)>, timestamp: u64) -> u64 {
    let mut depart_timestamp = timestamp;

    loop {
        for (_, id) in ids {
            if depart_timestamp % *id == 0 {
                return (depart_timestamp - timestamp) * (*id);
            }
        }
        depart_timestamp += 1;
    }

}

fn run_two(ids: &Vec<(u64, u64)>) -> i64 {
    let now = Instant::now();
    let mut ids :Vec<(i64, i64)> = ids.iter().map(|(i, id)| (*i as i64, *id as i64)).collect();
    println!("{:?}", ids);
    let mut left = ids.pop().unwrap();
    let mut maybe_right = ids.pop();
    while let Some(right) = maybe_right {
        left = combine(left, right);
        maybe_right = ids.pop();
    }

    println!("{:?}", now.elapsed());
    return -left.0
}

fn combine(left: (i64, i64), right: (i64, i64)) -> (i64, i64) {
    let ( i, k) = left;
    let ( j, m) = right;

    let ij = i - j;
    let mut x = 1;
    loop {
        if (k * x - ij) % m == 0 {
            return (i - x*k, k*m)
        }
        x+=1;
    }
}

#[test]
pub fn test_day13() {
    assert!(common::run_test(DAY, &run))
}