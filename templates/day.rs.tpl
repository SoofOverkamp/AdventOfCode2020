use aoc2020_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "{{.day}}";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    Ok((None, None))
}

#[test]
pub fn test_{{.day}}() {
    assert!(common::run_test(DAY, &run))
}