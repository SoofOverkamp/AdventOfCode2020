use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::collections::hash_map::RandomState;
use std::error::Error;
use std::iter::FromIterator;

const DAY: &str = "day16";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let (rules, ticket, near_tickets) = parse(input)?;

    let mut invalid_fields = Vec::new();
    let mut valid_tickets = Vec::new();
    for near_ticket in near_tickets.iter() {
        let mut valid = true;
        for field in near_ticket {
            let mut field_valid = false;
            for [range1, range2] in rules.values() {
                if range1.contains(&field) || range2.contains(&field) {
                    field_valid = true;
                    break;
                }
            }
            if ! field_valid {
                invalid_fields.push(*field);
                valid = false;
            }
        }
        if valid {
            valid_tickets.push(near_ticket);
        }
    }

    let names_set = HashSet::from_iter(rules.keys());

    let mut field_options: Vec<HashSet<&String>> = ticket.iter().map(|_| names_set.clone()).collect();
    let mut found_fields = HashMap::new();

    for near_ticket in valid_tickets {
        for (field, i) in near_ticket.iter().zip(0..) {
            for option in (*field_options.get(i).unwrap()).clone().iter() {
                let [range1, range2] = rules.get(*option).unwrap();
                if !range1.contains(field) && !range2.contains(field) {
                    field_options.get_mut(i).unwrap().remove(option);
                }
            }
            let options = field_options.get(i).unwrap();
            if options.len() == 1 {
                let option = *options.iter().next().unwrap();
                let result = remove_duplicates(option, field_options, found_fields);
                field_options = result.0;
                found_fields = result.1;
                found_fields.insert(option, i);
            }
        }
    }
    println!("{:?}", found_fields);

    let departure_product: u64 = found_fields.iter()
        .filter(|(name, _)| (**name).starts_with("departure"))
        .map(|(_, i)| (*ticket.get(*i).unwrap()) as u64)
        .product();

    Ok([Some(invalid_fields.iter().sum::<u32>().to_string()), Some(departure_product.to_string())])
}

fn remove_duplicates<'a>(to_remove: &'a String, mut from: Vec<HashSet<&'a String>>, mut collect: HashMap<&'a String, usize>) -> (Vec<HashSet<&'a String>>, HashMap<&'a String, usize>) {
    for i in 0..from.len() {
        let entry = from.get_mut(i).unwrap();
        entry.remove(to_remove);
        if entry.len() == 1 {
            let entry_entry = *entry.iter().next().unwrap();
            collect.insert(entry_entry, i);
            let result = remove_duplicates(entry_entry, from, collect);
            from = result.0;
            collect = result.1;
        }
    }
    (from, collect)
}


fn parse(input: &Vec<String>) -> Result<(HashMap<String, [Range<u32>; 2], RandomState>, Vec<u32>, Vec<Vec<u32>>), Box<dyn Error>> {
    let mut input = input.iter();

    let field_rule_re = regex::Regex::new(r"([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)")?;

    let parse_rule = |line: &str| {
        let captures = field_rule_re.captures(line)?;
        let name = captures.get(1)?.as_str().to_owned();
        let first_range: Range<u32> = captures.get(2)?.as_str().parse().ok()?..
            captures.get(3)?.as_str().parse().ok()?;
        let first_range = first_range.start..(first_range.end+1);
        let second_range: Range<u32> = captures.get(4)?.as_str().parse().ok()?..
            captures.get(5)?.as_str().parse().ok()?;
        let second_range = second_range.start..(second_range.end+1);
        return Some((name, [first_range, second_range]));
    };

    let parse_ticket = |line: &str| {
        let ticket: Result<Vec<u32>,_> = line.split(",").map(|n| n.parse()).collect();
        return ticket;
    };

    let mut rules = HashMap::new();
    while let Some(line) = input.next() {
        if line == "" { break };
        let rule = parse_rule(line).ok_or(format!("Could not parse input line {}", line))?;
        rules.insert(rule.0, rule.1);
    }

    let mut input = input.skip(1);

    let ticket = input.next().ok_or(format!("Could not find own ticket"))?;
    let ticket = parse_ticket(ticket)?;

    let mut input = input.skip(2);

    let mut near_tickets = Vec::new();
    while let Some(near_ticket) = input.next() {
        let near_ticket = parse_ticket(near_ticket)?;
        near_tickets.push(near_ticket);
    }

    return Ok((rules, ticket, near_tickets));
}

#[test]
pub fn test_day16() {
    assert!(common::run_test(DAY, &run))
}