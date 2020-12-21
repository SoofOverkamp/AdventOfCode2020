use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::collections::HashMap;
use std::error::Error;
use crate::Rule::{Leaf, Node};
use std::num::ParseIntError;

const DAY: &str = "day19";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let (mut rules, input) = parse(input)?;

    let mut re = String::from("^");
    rules.get(&0).unwrap().write_regex(&rules, &mut re);
    re.push('$');
    let re = regex::Regex::new(re.as_str())?;

    let mut count_1 = 0;
    for line in input.iter() {
        if re.is_match(line) {
            count_1 += 1;
        }
    }

    let count_2 = if rules.len() >= 11 {
        rules.insert(8, Node(vec![vec![42], vec![42, 8]]));
        rules.insert(11, Node(vec![vec![42, 31], vec![42, 11, 31]]));

        let mut re = String::from("^");
        let mut re_42 = String::new();
        rules.get(&42).unwrap().write_regex(&rules, &mut re_42);

        re.push_str(&re_42);
        re.push('+');
        re.push_str(&re_42);

        let mut re_31 = String::new();
        rules.get(&31).unwrap().write_regex(&rules, &mut re_31);

        let mut re_map = Vec::new();
        let mut count: u32 = 0;

        for line in input.iter() {
            let mut n = 1;
            loop {
                if re_map.len() == n - 1 {
                    let mut re_1 = re.clone();
                    let re_n = format!("{{{}}}", n);
                    re_1.push_str(&re_n);
                    re_1.push_str(r"(.*)$");
                    
                    let mut re_2 = String::from("^");
                    re_2.push_str(&re_31);
                    re_2.push_str(&re_n);
                    re_2.push('$');
                    re_map.push([
                        regex::Regex::new(&re_1)?,
                        regex::Regex::new(&re_2)?
                    ]);
                }
                let [re_1, re_2] = re_map.get(n - 1).unwrap();
                if let Some(captures) = re_1.captures(line) {
                    match captures.get(1).unwrap().as_str() {
                        "" => break,
                        left => {
                            if re_2.is_match(left) {
                                count += 1;
                                break;
                            }
                        }
                    };
                    n += 1;
                } else {
                    break;
                }
            };
        }
        count.to_string()
    } else {
        "-".to_owned()
    };

    Ok([Some(count_1.to_string()), Some(count_2.to_string())])
}

fn parse<'a>(input: &'a Vec<String>) -> Result<(HashMap<usize, Rule>, Vec<&'a String>), Box<dyn Error>> {
    let mut input = input.iter();
    let mut rules = HashMap::new();
    for line in &mut input {
        if line.as_str() == "" {
            break;
        }
        let split: Vec<&str> = line.split(":").collect();
        let num: usize = split.get(0).ok_or("Expected rule, got empty string")?.parse()?;

        let rule = *split.get(1).ok_or("Expected rule, got only rule number")?;
        let rule = if rule.contains("\"") {
            let c = rule.strip_prefix(" \"").ok_or(format!("Expected \".\" got {}", rule))?
                .strip_suffix("\"").ok_or(format!("Expected \".\" got {}", rule))?
                .to_owned();
            Leaf(c)
        } else {
            let rule: std::str::Split<&str> = rule.split("|");
            let rule: Vec<Vec<usize>> = rule.map(|part| {
                let part: Result<Vec<usize>, _> = part.trim().split(" ").map(|s| s.parse()).collect();
                part
            }).collect::<Result<Vec<Vec<usize>>, ParseIntError>>()?;
            Node(rule)
        };
        rules.insert(num, rule);
    }

    Ok((rules, input.collect()))
}

#[derive(Debug)]
enum Rule {
    Leaf(String),
    Node(Vec<Vec<usize>>)
}

impl Rule {
    fn write_regex(&self, rules: &HashMap<usize, Rule>, string: &mut String) {
        match self {
            Leaf(s) => string.push_str(s),
            Node(parts) => {
                string.push_str("(?:");
                parts.iter().take(parts.len() - 1).for_each(|part| {
                    part.iter().for_each(|child_rule| rules.get(child_rule).unwrap().write_regex(rules, string));
                    string.push('|');
                });
                parts.last().unwrap().iter().for_each(|child_rule| rules.get(child_rule).unwrap().write_regex(rules, string));
                string.push(')')
            }
        };
    }
}

#[test]
pub fn test_day19() {
    assert!(common::run_test(DAY, &run))
}