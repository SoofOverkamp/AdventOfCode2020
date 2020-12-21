use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::error::Error;
use crate::Rule::{Leaf, Node};
use std::num::ParseIntError;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::IterMut;
use std::collections::HashMap;

const DAY: &str = "day19";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let (mut rules, input) = parse(input)?;

    let mut count_1: u32 = 0;
    for line in input.iter() {
        if matches(line.as_str(), &rules) {
            count_1 += 1;
        }
    }

    let mut count_2 = if rules.len() >= 11 {
        rules.insert(8, Node(vec![vec![42], vec![42, 8]]));
        rules.insert(11, Node(vec![vec![42, 31], vec![42, 11, 31]]));
        println!("{:?}", rules);
        let mut count: u32 = 0;
        for line in input.iter() {
            if matches(line.as_str(), &rules) {
                count += 1;
            }
        }
        count.to_string()
    } else {
        "-".to_owned()
    };

    Ok([Some(count_1.to_string()), Some(count_2)])
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

fn matches(line: &str, rules: &HashMap<usize, Rule>) -> bool {
    let mut stack_node_0 = StackNode::new();
    let rule_0 = rules.get(&0).unwrap();
    loop {
        match partial_match_rule(line, rule_0, rules, &mut stack_node_0) {
            None => break false,
            Some("") => break true,
            _ => (),
        }
    }
}

fn partial_match_rule<'a>(line: &'a str, rule: &Rule, rules: &HashMap<usize, Rule>, stack_node: &mut StackNode) -> Option<&'a str> {
    let mut checked = stack_node.checked_index;
    let result = match rule {
        Leaf(c) => {
            if checked > 0 {
                None
            } else {
                checked = 1;
                line.strip_prefix(c)
            }
        },
        Node(parts) => {
            let mut matches;
            let mut s;
            let mut result = None;
            for part in parts.iter().skip(checked) {
                matches = false;
                s = line;
                let mut i = 0;
                let mut string_stack = vec![];
                let matches = loop {
                    let child_rule = part[i];
                    let child_node = stack_node.get_mut_or_insert_default(i);
                    let matchh = partial_match_rule(s, rules.get(&child_rule).unwrap(), rules, child_node);
                    if let Some(res) = matchh {
                        string_stack.push(s);
                        s = res;
                        i += 1;
                        if i == part.len() {
                            break true;
                        }
                    } else {
                        matches = false;
                        child_node.checked_index = 0;
                        child_node.children.clear();
                        if i == 0 {
                            break false;
                        }
                        s = string_stack.pop().unwrap();
                        i -= 1;
                    }
                };
                if matches {
                    result = Some(s);
                    break;
                } else {
                    stack_node.children.clear();
                }
                checked += 1;
            }
            result
        }
    };
    stack_node.checked_index = checked;
    result
}

#[derive(Debug)]
enum Rule {
    Leaf(String),
    Node(Vec<Vec<usize>>)
}

#[derive(Debug)]
struct StackNode {
    checked_index: usize,
    children: Vec<StackNode>,
}

impl StackNode {
    fn new() -> Self {
        StackNode {
            checked_index: 0,
            children: Vec::new(),
        }
    }

    fn push_default_if_larger(&mut self, i: usize) {
        if self.children.len() == i {
            self.children.insert(i, Self::new());
        } else if self.children.len() < i {
            panic!("Can only get or insert with index <= length, got index {} and length {}", i, self.children.len());
        }
    }

    fn get_or_insert_default(&mut self, i: usize) -> &StackNode {
        self.push_default_if_larger(i);
        &self.children[i]
    }

    fn get_mut_or_insert_default(&mut self, i: usize) -> &mut StackNode {
        self.push_default_if_larger(i);
        &mut self.children[i]
    }

}



#[test]
pub fn test_day19() {
    assert!(common::run_test(DAY, &run))
}