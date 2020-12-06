use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::collections::HashSet;

const DAY: &str = "day6";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut question_1_count: usize = 0;
    let mut question_2_count: usize = 0;
    let mut group_set_1 = HashSet::new();
    let mut group_set_2: Option<HashSet<char>> = None;
    for line in input {
        if line.len() == 0 {
            question_1_count += group_set_1.len();
            question_2_count += group_set_2.as_ref().map(HashSet::len).unwrap_or(0);
            group_set_1 = HashSet::new();
            group_set_2 = None;
        } else {
            // for char in line.chars() {
            //     group_set_1.insert(char);
            // }
            let person_set = line.chars().collect();
            group_set_1 = group_set_1.union(&person_set).map(ToOwned::to_owned).collect();
            group_set_2 = match &group_set_2 {
                Some(set) => Some(set.intersection(&person_set).map(ToOwned::to_owned).collect()),
                None => Some(person_set)
            }
        }
    }
    question_1_count += group_set_1.len();
    question_2_count += group_set_2.as_ref().map(HashSet::len).unwrap_or(0);

    return Ok([Some(question_1_count.to_string()), Some(question_2_count.to_string())]);
}

#[test]
pub fn test_day6() {
    assert!(common::run_test(DAY, &run))
}