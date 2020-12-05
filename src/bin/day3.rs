use aoc2020_niels_overkamp::common;
use aoc2020_niels_overkamp::common::AOCResult;

const DAY: &str = "day3";


fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &prog)
}

#[test]
pub fn test_day3() {
    assert!(common::run_test(DAY, &prog))
}

fn prog(input: &Vec<String>) -> AOCResult {
    return Ok([run1(input), run2(input)]);
}

fn run(input: &Vec<String>, dx: usize, dy: usize) -> usize {
    let input: Vec<Vec<char>> = input.iter()
        .map(|s| s.chars().collect())
        .collect();

    let mut x = 0;
    let mut tree_count = 0;
    for line in input.chunks(dy) {
        let line = &line[0];
        if line[x] == '#' {
            tree_count += 1;
        }
        x += dx;
        x %= line.len();
    }

    return tree_count;
}

fn run1(input: &Vec<String>) -> Option<String> {
    Some(run(input, 3, 1).to_string())
}

fn run2(input: &Vec<String>) -> Option<String> {
    let mut prod = 1;
    for (dx, dy) in [(1,1), (3,1), (5,1), (7,1), (1,2)].iter() {
        prod *= run(input, *dx, *dy);
    }
    return Some(prod.to_string());
}