use std::collections::{HashMap, HashSet};

use aoc2020_niels_overkamp::common::{self, AOCResult};
use std::num::ParseIntError;

const DAY: &str = "day7";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}

pub fn run(input: &Vec<String>) -> AOCResult {
    let re = regex::Regex::new(r"(\d+) ([\w\s]+) bags?")?;
    let mut string_bags = HashMap::with_capacity(input.len());
    for line in input {
        let mut line = line.split(" bags contain ").take(2);
        let [bag, contains] = [line.next().unwrap(), line.next().unwrap()];
        let captures = re.captures_iter(contains);
        let contains: Vec<(u32, String)> = captures.map(|c| (c.get(1).unwrap().as_str().parse().unwrap(), c.get(2).unwrap().as_str().to_owned())).collect();
        let bag: StringBag = StringBag {
            name: bag.to_owned(),
            contains
        };
        string_bags.insert(bag.name.clone(), bag);
    }
    let mut ancestors = HashSet::new();
    for bag in string_bags.values() {
        if bag.contains("shiny gold", &string_bags, &mut HashSet::new()) {
            ancestors.insert(&bag.name);
            println!("shiny gold");
        }
    }

    let content_count = string_bags.get("shiny gold").unwrap().count_contents(&string_bags, &mut HashSet::new());

    return Ok([Some(ancestors.len().to_string()), Some(content_count.as_ref().map_or("-".to_owned(), ToString::to_string))]);
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct StringBag {
    name: String,
    contains: Vec<(u32, String)>,
}

impl StringBag {
    fn contains<'t>(&'t self, v: &str, map: &'t HashMap<String, StringBag>, visited: &mut HashSet<&'t String>) -> bool {
        if !visited.contains(&self.name) {
            visited.insert(&self.name);
            for (_, child) in &self.contains {
                if child.as_str() == v || map.get(child).unwrap().contains(v, map, visited) {
                    return true;
                }
            }
        }
        return false;
    }

    fn count_contents<'t>(&'t self, map: &'t HashMap<String, StringBag>, visited: &mut HashSet<&'t String>) -> Option<u64> {
        if visited.contains(&self.name) {
            return None;
        }
        visited.insert(&self.name);
        let mut sum = 0;
        for (count, child) in &self.contains {
            sum += (*count as u64) * (map.get(child).unwrap().count_contents(map, visited)? + 1);
        }
        visited.remove(&self.name);
        return Some(sum);
    }
}

#[test]
pub fn test_day7() {
    assert!(common::run_test(DAY, &run))
}