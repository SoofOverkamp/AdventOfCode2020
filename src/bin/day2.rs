use core::ops;

use aoc2020_niels_overkamp::common::{self, AOCResult};

const DAY: &str = "day2";

struct Pass {
    range: ops::Range<usize>,
    index1: usize,
    index2: usize,
    limit_char: char,
    password: String,
}

pub fn main() {
    if common::run_test(DAY, &run) {
        common::run(DAY, &run);
    }
}

#[test]
pub fn test_day2() {
    assert!(common::run_test(DAY, &run))
}

fn run(lines: &Vec<String>) -> AOCResult {
    let re = regex::Regex::new(r"(?P<rl>\d+)-(?P<ru>\d+) (?P<c>\w): (?P<p>\w+)")?;
    let mut sled_valid_count: u32 = 0;
    let mut valid_count: u32 = 0;
    for line in lines {
        let cap = re.captures_iter(&line).next().ok_or("No matches found")?;
        let rl = cap.name("rl").ok_or("parsing failed on rl")?.as_str().parse().ok().ok_or("parsing failed on rl")?;
        let ru: usize = cap.name("ru").ok_or("parsing failed on ru")?.as_str().parse().ok().ok_or("parsing failed on ru")?;
        let c = cap.name("c").ok_or("parsing failed on c")?.as_str().chars().next().ok_or("parsing failed on c")?;
        let p = String::from(cap.name("p").ok_or("parsing failed on c")?.as_str());
        let pass = Pass {
            range: rl..(ru + 1),
            index1: rl,
            index2: ru,
            limit_char: c,
            password: p,
        };

        if pass.range.contains(&pass.password.chars().filter(|c| *c == pass.limit_char).count()) {
            // println!("line {} is sled valid", line);
            sled_valid_count += 1;
        }

        let mut i1 = pass.password.chars().skip(pass.index1 - 1);
        let c1 = i1.next().ok_or("password index not in password length")?;
        let c2 = i1.skip(pass.index2 - pass.index1 - 1).next().ok_or("password index not in password length")?;

        if (c1 == pass.limit_char) ^ (c2 == pass.limit_char) {
            // println!("line {} is valid", line);
            valid_count += 1;
        }
    }
    println!("{} sled valid passwords", sled_valid_count);
    println!("{} valid passwords", valid_count);
    return Ok((Some(sled_valid_count.to_string()), Some(valid_count.to_string())));
}

