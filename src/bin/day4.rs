use aoc2020_niels_overkamp::common;
use std::str::FromStr;
use aoc2020_niels_overkamp::common::{AOCResult};

const DAY: &str = "day4";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::run(DAY, &run)
}


#[test]
pub fn test_day4() {
    assert!(common::run_test(DAY, &run))
}

enum Color {
    Amb, Blu, Brn, Gry, Grn, Hzl, Oth
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(Color::Amb),
            "blu" => Ok(Color::Blu),
            "brn" => Ok(Color::Brn),
            "gry" => Ok(Color::Gry),
            "grn" => Ok(Color::Grn),
            "hzl" => Ok(Color::Hzl),
            "oth" => Ok(Color::Oth),
            unknown => Err(format!("No such Color {}", unknown))
        }

    }
}

#[allow(dead_code)]
struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: (u8, String),
    hcl: [u8; 3],
    ecl: Color,
    pid: u32,
}

struct OptionPassport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

const EMPTY_OPTION_PASSPORT: OptionPassport = OptionPassport {
    byr: None,
    iyr: None,
    eyr: None,
    hgt: None,
    hcl: None,
    ecl: None,
    pid: None,
    cid: None
};

impl OptionPassport {
    fn has_all_fields(&self) -> bool {
        self.byr.is_some() && self.iyr.is_some() && self.eyr.is_some() &&
            self.hgt.is_some() && self.hcl.is_some() && self.ecl.is_some() &&
            self.pid.is_some()
    }
    
    fn set_pass_field(&mut self, key:&str, value:&str) {
        match key {
            "byr" => self.byr = Some(String::from(value)),
            "iyr" => self.iyr = Some(String::from(value)),
            "eyr" => self.eyr = Some(String::from(value)),
            "hgt" => self.hgt = Some(String::from(value)),
            "hcl" => self.hcl = Some(String::from(value)),
            "ecl" => self.ecl = Some(String::from(value)),
            "pid" => self.pid = Some(String::from(value)),
            "cid" => self.cid = Some(String::from(value)),
            k => { eprintln!("Unknown passport key {}", k) }
        }
    }

    fn get_pass(&self) -> Option<Passport> {
        Some(Passport {
            byr: self.byr.as_ref().and_then(|s| s.parse::<u32>().ok())
                .and_then(|b| if b >= 1920 && b <= 2002 { Some(b as u16) } else { None })?,
            iyr: self.iyr.as_ref().and_then(|s| s.parse::<u32>().ok())
                .and_then(|i| if i >= 2010 && i <= 2020 { Some(i as u16) } else { None })?,
            eyr: self.eyr.as_ref().and_then(|s| s.parse::<u32>().ok())
                .and_then(|e| if e >= 2020 && e <= 2030 { Some(e as u16) } else { None })?,
            hgt: self.hgt.as_ref()
                .and_then(|s| {
                    return if s.ends_with("in") {
                        let v: u8 = s.chars().take(s.len() - 2).collect::<String>().parse().ok()?;
                        if v >= 59 && v <= 76 {
                            Some((v, String::from("in")))
                        } else {
                            None
                        }
                    } else if s.ends_with("cm") {
                        let v: u8 = s.chars().take(s.len() - 2).collect::<String>().parse().ok()?;
                        if v >= 150 && v <= 193 {
                            Some((v, String::from("cm")))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })?,
            hcl: self.hcl.as_ref().and_then(|s| if s.starts_with("#") && s.len() == 7 {Some(s)} else {None})
                .and_then(|s| hex::decode(&s.as_str()[1..]).ok())
                .and_then(|bs| {
                    if bs.len() == 3 {
                        Some([bs[0], bs[1], bs[1]])
                    } else {
                        None
                    }
                })?,
            ecl: self.ecl.as_ref().and_then(|s| s.parse::<Color>().ok())?,
            pid: self.pid.as_ref().and_then(|s| if s.len() == 9 {s.parse().ok()} else {None})?
        })
    }
}


fn run(input: &Vec<String>) -> AOCResult {
    let mut pass = OptionPassport {..EMPTY_OPTION_PASSPORT};
    let mut all_fields_count = 0;
    let mut valid_count = 0;
    // let mut option_pass_vec = vec![];
    for line in input {
        if line == "" {
            if pass.has_all_fields() {
                all_fields_count += 1;
                if pass.get_pass().is_some() {
                    valid_count += 1;
                }
            }
            pass = OptionPassport {..EMPTY_OPTION_PASSPORT};
        } else {
            line.split(' ')
                .map(|e| e.split(':'))
                .for_each(|mut kv| pass.set_pass_field(kv.next().unwrap(), kv.next().unwrap()));
        }
    }
    if pass.has_all_fields() {
        all_fields_count += 1;
        if pass.get_pass().is_some() {
            valid_count += 1;
        }
    }
    return Ok([Some(all_fields_count.to_string()), Some(valid_count.to_string())]);
}
