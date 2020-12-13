use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::iter::FromIterator;

const DAY: &str = "day10";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut numbers: Vec<usize> = input.iter().map(|line| line.parse().unwrap()).collect();
    numbers.sort();
    let device_rating = numbers[numbers.len() - 1] + 3;
    numbers.push(device_rating);
    let mut difference_count: [u32; 3] = [0,0,0];
    let mut previous_jolt = 0;

    for number in numbers.iter() {
        let diff = number - previous_jolt;
        if diff > 0 && diff <= 3 {
            difference_count[diff - 1] += 1;
            previous_jolt = *number;
        } else {
            return Err(format!("invalid diff {} between {} and {}", diff, previous_jolt, number).into());
        }
    }

    numbers.insert(0, 0);
    let mut found_arrangements = Vec::from_iter((0..device_rating + 1).map(|_| None));

    let count = arrangements(numbers.as_slice(), &mut found_arrangements);

    return Ok([Some((difference_count[0] * difference_count[2]).to_string()), Some(count.to_string())]);
}

fn arrangements(numbers: &[usize], found_arrangements_count: &mut Vec<Option<u64>>) -> u64 {
    if numbers.len() == 0 {
        panic!("Invalid input: slice of length 0");
    }
    let number = numbers[0];
    if let Some(result) = found_arrangements_count[number] {
        return result;
    }
    return if numbers.len() == 1 {
        found_arrangements_count[number] = Some(1);
        1
    } else {
        let mut count = 0;
        for i in 1..4 {
            if i < numbers.len() && numbers[i] - number <= 3 {
                count += arrangements(&numbers[i..], found_arrangements_count);
            } else {
                break;
            }
        }
        found_arrangements_count[number] = Some(count);
        count
    }
}

#[test]
pub fn test_day10() {
    assert!(common::run_test(DAY, &run))
}