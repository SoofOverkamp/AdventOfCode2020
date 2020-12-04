use aoc2020_niels_overkamp::common;
use aoc2020_niels_overkamp::common::AOCResult;

const DAY: &str = "day1";

fn main() {
    if common::run_test(DAY, &run) {
        common::run(DAY, &run);
    }
}

pub fn run(input: &Vec<String>) -> AOCResult {
    Ok((run1(input), run2(input)))
}

#[test]
pub fn test_day1() {
    assert!(common::run_test(DAY, &run))
}


fn run1(expenses: &Vec<String>) -> Option<String> {
    let expenses: Vec<u32> = expenses.iter().map(|s| s.parse().expect("Parse error")).collect();
    let mut result = None;
    'outer: for (i1, expense1) in Iterator::zip(0..expenses.len(), &expenses) {
        for (i2, expense2) in Iterator::zip(0..expenses.len(), &expenses) {
            if i1 != i2 && expense1 + expense2 == 2020 {
                println!("{0} + {1} = 2020; {0} * {1} = {2}", expense1, expense2, expense1 * expense2);
                result = Some((expense1 * expense2).to_string());
                break 'outer;
            }
        }
    }
    return result;
}

fn run2(expenses: &Vec<String>) -> Option<String> {
    let expenses: Vec<u32> = expenses.iter().map(|s| s.parse().expect("Parse error")).collect();
    let mut result = None;
    for (i1, expense1) in Iterator::zip(0..expenses.len(), &expenses) {
        for (i2, expense2) in Iterator::zip(0..expenses.len(), &expenses) {
            for (i3, expense3) in Iterator::zip(0..expenses.len(), &expenses) {
                if i1 != i2 && i2 != i3 && i1 != i3 && expense1 + expense2 + expense3== 2020 {
                    println!("{0} + {1} + {2} = 2020; {0} * {1} * {2} = {3}", expense1, expense2, expense3, expense1 * expense2 * expense3);
                    result = Some((expense1 * expense2 * expense3).to_string());
                    break;
                }
            }
        }
    }
    return result;
}

