use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::str::FromStr;
use regex::internal::Inst;
use crate::Instr::Mask;
use std::collections::HashMap;

const DAY: &str = "day14";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let mut instrs = parse_input(input)?;


    Ok([Some(run1(&instrs)?.to_string()), Some(run2(&instrs)?.to_string())])
}

fn run1(instrs: &Vec<Instr>) -> Result<u64, String> {
    let mut instrs = instrs.iter();
    let first = instrs.next();
    if let Some(Mask([mask0, mask1])) = &first {
        let mut mask0 = *mask0;
        let mut mask1 = *mask1;
        let mut hash_mem = HashMap::new();
        for instr in instrs {
            match instr {
                Mask([m0, m1]) => {
                    mask0 = *m0;
                    mask1 = *m1
                }
                Instr::Write((addr, val)) => {
                    hash_mem.insert(addr, (val | mask1) & mask0);
                }
            };
        }
        Ok(hash_mem.values().sum())
    } else {
        Err(format!("expected first instruction to be mask got {:?}", first))
    }
}

fn run2(instrs: &Vec<Instr>) -> Result<u64, String> {
    let mut instrs = instrs.iter();
    let first = instrs.next();
    if let Some(Mask([mask0, mask1])) = &first {
        let mut addr_mask = (*mask0 ^ *mask1) as usize;
        let mut mask1 = *mask1;
        let mut hash_mem = HashMap::new();
        for instr in instrs {
            match instr {
                Mask([m0, m1]) => {
                    addr_mask = (*m0 ^ *m1) as usize;
                    mask1 = *m1
                }
                Instr::Write((addr, val)) => {
                    let mut addrs = vec![(*addr) | mask1 as usize];
                    let mut p = 1;
                    while p <= addr_mask {
                        if addr_mask & p == p {
                            let mut new_addrs = Vec::with_capacity(addrs.len() * 2);
                            for addr in addrs {
                                new_addrs.push(addr |  (addr_mask & p));
                                new_addrs.push(addr & !(addr_mask & p));
                            }
                            addrs = new_addrs;
                        }
                        p = p << 1;
                    }
                    for addr in addrs {
                        hash_mem.insert(addr, *val);
                    }
                }
            };           
        }
        Ok(hash_mem.values().sum())
    } else {
        Err(format!("expected first instruction to be mask got {:?}", first))
    }
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    Mask([u64; 2]),
    Write((usize, u64)),
}

fn parse_mask(s: &str) -> Option<[u64; 2]> {
    let mut l = s.len() as u64;
    let mut n0 = 0;
    let mut n1 = 0;
    for c in s.chars() {
        l -= 1;
        match c {
            'X' => Some(n0 |= 1 << l),
            '0' => Some(()),
            '1' => {
                n0 |= 1 << l;
                n1 |= 1 << l;
                Some(())
            }
            _ => None
        }?;
    };
    return Some([n0, n1])
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().skip(1);
        (|| {
            match chars.next()? {
                'a' => {
                    let mask = s.split(" = ").skip(1).next()?;
                    Some(Instr::Mask(parse_mask(mask)?))
                }
                'e' => {
                    let addr: String = chars.skip(2).take_while(|c| *c != ']').collect();
                    let addr: usize = addr.parse().ok()?;
                    let val: u64 = s.split(" = ").skip(1).next()?.parse().ok()?;
                    Some(Instr::Write((addr, val)))
                }
                _ => None
            }
        })().ok_or(format!("Could not parse {} into Instr", s))
    }
}

fn parse_input(input: &Vec<String>) -> Result<Vec<Instr>, String> {
    input.iter().map(|s| s.parse::<Instr>()).collect()
}

#[test]
pub fn test_day14() {
    assert!(common::run_test(DAY, &run))
}