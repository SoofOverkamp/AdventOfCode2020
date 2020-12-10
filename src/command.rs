use core::fmt;
use std::{error, io};
use std::io::{stdin, stdout, Write};
use std::str::FromStr;
use chrono::Duration;

pub enum CommandType {
    Test,
    Run,
    Submit,
    NextPart,
    Puzzle,
    Leaderboard,
    Day,
    Year,
    Help,
    Quit,
}

pub struct Command {
    pub command_type: CommandType,
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandParseErr(());

impl fmt::Display for CommandParseErr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("invalid CommandType syntax")
    }
}

impl error::Error for CommandParseErr {}

impl FromStr for CommandType {
    type Err = CommandParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace(" ", "").as_str() {
            "test" | "t" => Ok(CommandType::Test),
            "run" | "r" => Ok(CommandType::Run),
            "submit" | "s" => Ok(CommandType::Submit),
            "nextpart" | "n" => Ok(CommandType::NextPart),
            "puzzle" | "p" => Ok(CommandType::Puzzle),
            "leaderboard" | "l" => Ok(CommandType::Leaderboard),
            "changeday" | "day" | "d" => Ok(CommandType::Day),
            "changeyear" | "year" | "y" => Ok(CommandType::Year),
            "help" | "h" | "?" => Ok(CommandType::Help),
            "quit" | "q" => Ok(CommandType::Quit),
            _ => Err(CommandParseErr(()))
        }
    }
}

impl FromStr for Command {
    type Err = CommandParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut arguments = s.split_whitespace();
        let command_type = arguments.next().ok_or(CommandParseErr(()))?.parse::<CommandType>()?;
        let arguments = arguments.map(ToOwned::to_owned).collect();
        return Ok(Command {
            command_type,
            arguments,
        });
    }
}

pub struct Terminal {
    pub strings: TerminalStrings,
}

pub struct TerminalStrings {
    pub emph_style: ansi_term::Style,
    help: String,
    y_n_true: String,
    y_n_false: String,
    y_n_none: String,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            strings: TerminalStrings::new(),
        }
    }

    pub fn yes_no(&self, default: Option<bool>, question: Option<&str>) -> io::Result<Option<bool>> {
        print!("{}{}> ", question.unwrap_or(""), self.strings.yes_no_text(default));
        stdout().flush()?;
        return Self::read_yes_no(default);
    }

    pub fn read_yes_no(default: Option<bool>) -> io::Result<Option<bool>> {
        let mut input = String::new();
        if stdin().read_line(&mut input)? == 0 {
            return Ok(None);
        }
        return Ok(
            match input.as_str().strip_suffix("\n").unwrap() {
                "y" | "Y" => Some(true),
                "n" | "N" => Some(false),
                _ => default
            }
        );
    }

    pub fn options(&self, default: Option<&str>, options: &[&str], question: Option<&str>) -> io::Result<Option<String>> {
        let mut s = String::new();
        let options: &[String] = &options.iter()
            .map(|o| {
                let (i, c) = (*o).char_indices().next().unwrap();
                let (emph_c, rest) = (self.strings.emph_style.paint(c.to_string().to_uppercase()).to_string(), &o[(i+1)..]);
                s.push_str(format!("{}{}/", emph_c, rest).as_str());
                return c.to_lowercase().to_string();
            })
            .collect::<Vec<String>>()[..];

        print!("{}({})> ",
               question.unwrap_or(""),
               s.trim_end_matches("/"),
        );

        stdout().flush()?;
        return Self::read_options(default, options);
    }

    pub fn read_options(default: Option<&str>, options: &[String]) -> io::Result<Option<String>> {
        let mut input = String::new();
        if stdin().read_line(&mut input)? == 0 {
            return Ok(None);
        }
        let input = input.to_lowercase();
        let mut chars = input.trim_end().chars();
        return Ok(chars.next().map(|c| c.to_lowercase().to_string()).filter(|c| options.contains(&c)).or(default.map(|d| d.to_owned())));
    }

    pub fn help(&self) {
        println!("{}", self.strings.help)
    }
}

impl TerminalStrings {
    fn new() -> Self {
        let emph_style = ansi_term::Style::new().bold().underline();
        TerminalStrings {
            help: format!("Choose action: {}est, {}un, {}ubmit, {}ext Part, open {}uzzle, open {}eaderboard, change {}ay, change {}ear, {}elp, {}uit",
                          emph_style.paint("T"),
                          emph_style.paint("R"),
                          emph_style.paint("S"),
                          emph_style.paint("N"),
                          emph_style.paint("P"),
                          emph_style.paint("L"),
                          emph_style.paint("D"),
                          emph_style.paint("Y"),
                          emph_style.paint("H"),
                          emph_style.paint("Q")),
            y_n_true: format!("{}/{} ", emph_style.paint("Y"), emph_style.paint("n")),
            y_n_false: format!("{}/{} ", emph_style.paint("y"), emph_style.paint("N")),
            y_n_none: format!("{}/{} ", emph_style.paint("y"), emph_style.paint("n")),
            emph_style,
        }
    }

    pub fn yes_no_text(&self, default: Option<bool>) -> &String {
        match default {
            Some(true) => &self.y_n_true,
            Some(false) => &self.y_n_false,
            None => &self.y_n_none,
        }
    }
}

pub fn format_duration(duration: Duration) -> String {
    if duration.num_weeks() != 0 {
        return format!("{} weeks", duration.num_weeks());
    }
    if duration.num_days() != 0 {
        return format!("{} days", duration.num_days());
    }
    if duration.num_hours() != 0 {
        return format!("{} hours", duration.num_hours());
    }
    if duration.num_minutes() != 0 {
        return format!("{} minutes", duration.num_minutes());
    }
    if duration.num_seconds() != 0 {
        return format!("{} seconds", duration.num_seconds());
    }
    return "now".to_owned();
}
