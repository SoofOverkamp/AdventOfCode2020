use chrono::{Datelike, Timelike, TimeZone};
use std::{path, fmt, fs};
use std::fmt::{Display, Formatter};
use std::error::Error;

pub struct AocDay {
    pub day: u8,
    pub year: u16,
}

impl AocDay {
    pub fn new(day: u8, year: u16) -> AocDay {
        AocDay { day, year }
    }

    pub fn today() -> AocDay {
        let now = chrono::Utc::now();
        if now.month() == 12 {
            if now.hour() >= 5 {
                Self::new(now.day().min(25) as u8, now.year() as u16)
            } else {
                Self::new((now.day() - 1).min(25).max(1) as u8, now.year() as u16)
            }
        } else if now.month() <= 2 {
            Self::new(25, now.year() as u16 - 1)
        } else {
            Self::new(1, now.year() as u16 )
        }
    }

    pub fn is_open(&self) -> bool {
        let start_moment = chrono::Utc
            .ymd(self.year as i32, 12, self.day as u32)
            .and_hms(5, 0, 0);
        return chrono::Utc::now().ge(&start_moment);
    }

    pub fn source_code_path(&self) -> String {
        format!("./src/bin/day{}.rs", self.day)
    }

    pub fn input_file_path(&self) -> String {
        format!("./inputs/day{}.txt", self.day)
    }

    pub fn test_input_file_path(&self) -> String {
        format!("./test_inputs/day{}.txt", self.day)
    }

    pub fn output_file_path(&self, part: &Part) -> String {
        format!("outputs/day{}-{}.txt", self.day, part.as_str())
    }

    pub fn has_source_code(&self) -> bool {
        path::Path::new(self.source_code_path().as_str()).exists()
    }

    pub fn has_input_file(&self) -> bool {
        path::Path::new(self.input_file_path().as_str()).exists()
    }

    pub fn has_test_input_file(&self) -> bool {
        path::Path::new(self.test_input_file_path().as_str()).exists()
    }

    pub fn output_file_age(&self, part: &Part) -> Result<Option<chrono::Duration>, Box<dyn Error>> {
        let metadata: Option<fs::Metadata> = fs::metadata(self.output_file_path(part).as_str())
            .map(Some).unwrap_or(None);
        if let Some(metadata) = metadata {
            Ok(Some(chrono::Duration::from_std(metadata.modified()?.elapsed()?)?))
        } else {
            Ok(None)
        }
    }

    pub fn input_file_url(&self) -> String {
        format!("https://adventofcode.com/{}/day/{}/input", self.year, self.day)
    }

    pub fn submit_url(&self) -> String {
        format!("https://adventofcode.com/{}/day/{}/answer", self.year, self.day)
    }

    pub fn puzzle_url(&self) -> String {
        format!("https://adventofcode.com/{}/day/{}", self.year, self.day)
    }

    pub fn leaderboard_url(&self) -> String {
        format!("https://adventofcode.com/{}/leaderboard/private/view/424060", self.year)
    }
}

pub enum Part {
    ONE, TWO,
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Part {
    pub fn as_str(&self) -> &'static str {
        match self {
            Part::ONE => "one",
            Part::TWO => "two",
        }
    }

    pub fn as_test(&self) -> &'static str {
        match self {
            Part::ONE => "test_part_one",
            Part::TWO => "test_part_two",
        }
    }

    pub fn as_digit(&self) -> u8 {
        match self {
            Part::ONE => 1,
            Part::TWO => 2,
        }
    }
}
