use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::collections::HashSet;
use std::num::ParseIntError;

const DAY: &str = "day8";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let instr: Result<Vec<(&str, i64)>, ParseIntError> = input.iter().map(|line| {
        let mut split = line.split(" ");
        return Ok((split.next().unwrap(), split.next().unwrap().parse()?));
    }).collect();

    let mut instr = instr?;

    let (loop_val, start_nodes) = execute(&instr)?;

    let fix = find_bug(&instr, &start_nodes)?;
    instr[fix.0] = (fix.1.as_str(), instr[fix.0].1);

    return Ok([Some(loop_val.to_string()), Some(execute(&instr)?.0.to_string())]);
}

fn execute(instr: &Vec<(&str, i64)>) -> Result<(i64, HashSet<usize>), String>{
    let mut acc = 0;
    let mut pc = 0;
    let mut visited = HashSet::new();
    loop {
        if visited.contains(&pc) || pc >= instr.len() {
            break;
        }
        visited.insert(pc);
        let (op, val) = instr[pc as usize];
        match op {
            "acc" => acc += val,
            "jmp" => pc = ((pc as i64) + (val - 1)) as usize,
            "nop" => (),
            _ => return Err(format!("Don't know instruction {}", op)),
        }
        pc += 1;
    }
    return Ok((acc, visited));
}

fn find_bug(instr: &Vec<(&str, i64)>, start_nodes: &HashSet<usize>) -> Result<(usize, String), String> {
    let len = instr.len();
    let mut end_nodes: HashSet<usize> = HashSet::new();
    end_nodes.insert(len);
    loop {
        let end_nodes_count = end_nodes.len();
        for (pc, (op, val)) in (0..).zip(instr) {
            if match *op {
                "acc" | "nop" => end_nodes.contains(&(pc + 1)),
                "jmp" => end_nodes.contains(&((pc as i64 + val) as usize)),
                _ => return Err(format!("Don't know instruction {}", op)),
            } {
                end_nodes.insert(pc);
            }
        }
        if end_nodes_count == end_nodes.len() {
            break;
        }
    }
    for pc in start_nodes {
        let pc = *pc;
        let (op, val) = instr[pc];
        let (found, op) = match op {
            "jmp" => (end_nodes.contains(&(pc + 1)), "nop"),
            "nop" => (end_nodes.contains(&(((pc as i64) + val) as usize)), "jmp"),
            "acc" => (false, ""),
            _ => return Err(format!("Don't know instruction {}", op)),
        };
        if found {
            return Ok((pc, op.to_owned()));
        }
    }

    return Err("no fix could be found".to_owned());
}

#[test]
pub fn test_day8() {
    assert!(common::run_test(DAY, &run))
}