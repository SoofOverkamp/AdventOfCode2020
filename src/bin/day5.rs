use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::ops::Range;
use std::error::Error;

const DAY: &str = "day5";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut boarding_ids: Vec<u32> = vec![];
    for boarding_pass in input {
        if boarding_pass.len() == 0 {
            continue;
        }
        let mut row = 0..127;
        let mut column = 0..7;
        for c in boarding_pass.chars() {
            match c {
                'F' => Ok(row.end = (row.start + row.end) / 2),
                'B' => Ok(row.start =  (row.start + row.end) / 2 + 1),
                'L' => Ok(column.end =  (column.start + column.end) / 2),
                'R' => Ok(column.start =  (column.start + column.end) / 2 + 1),
                _ => Err(format!("Unknown Char {}", c)),
            }?;
        }
        if row.start != row.end {
            Err(format!("Boarding pass {} not exhaustive: row.start {} differs from row.end{}", boarding_pass, row.start, row.end))?
        }
        if column.start != column.end {
            Err(format!("Boarding pass {} not exhaustive: column.start {} differs from column.end{}", boarding_pass, column.start, column.end))?
        }
        boarding_ids.push(row.start * 8 + column.start);
    }
    let no_boarding_error: Box<dyn Error> = "No boarding passes found".into();
    let max_id = boarding_ids.iter().max()
        .ok_or(no_boarding_error)?;

    let all_ids: Range<u32> = 0..128*8;

    let my_id = all_ids.filter(|id| *id > 0 && !boarding_ids.contains(id) &&
        boarding_ids.contains(&(*id+1)) &&
        boarding_ids.contains(&(*id-1)))
        .map(|id| id.to_string())
        .next();

    return Ok([Some(max_id.to_string()), my_id]);
}

#[test]
pub fn test_day5() {
    assert!(common::run_test(DAY, &run))
}