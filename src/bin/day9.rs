use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::error::Error;

const DAY: &str = "day9";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let batch_size: usize = input[0].parse()?;
    let numbers: Vec<u64> = input.iter()
        .skip(1)
        .map(|line| line.parse().unwrap()).collect();
    let mut sum = None;
    'outer: for (i, number) in (batch_size..).zip(numbers.iter().skip(batch_size)) {
        let mut found = false;
        'f1: for f1 in numbers.split_at(i-batch_size).1.split_at(batch_size).0 {
            for f2 in numbers.split_at(i-batch_size).1.split_at(batch_size).0 {
                if f1 != f2 && f1 + f2 == *number {
                    found = true;
                    break 'f1;
                }
            }
        }
        if !found {
            sum = Some(number);
            break 'outer;
        }
    }
    let sum = *sum.ok_or::<Box<dyn Error>>("No value found not conforming to addition rule".into())?;


    let mut local_sum = 0;
    let mut range = 0..0;
    for number in &numbers {
        local_sum += *number;
        while local_sum > sum {
            local_sum -= numbers[range.start];
            range.start += 1;
        }
        range.end += 1;
        if local_sum == sum {
            break;
        }
    }

    if range.start >= numbers.len() || range.end < range.start {
        return Err("no valid range found".into());
    }

    let number_range = &numbers[range];


    let weakness = number_range.iter().min().unwrap() + number_range.iter().max().unwrap();

    return Ok([Some(sum.to_string()), Some(weakness.to_string())]);
}

#[test]
pub fn test_day9() {
    assert!(common::run_test(DAY, &run))
}