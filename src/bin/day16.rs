use aoc2020_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day16";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    Ok([None, None])
}

#[test]
pub fn test_day16() {
    assert!(common::run_test(DAY, &run))
}