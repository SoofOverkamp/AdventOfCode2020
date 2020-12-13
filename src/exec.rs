use std::{fs, process, io};
use std::error::Error;
use std::io::{Read, stderr, stdin, stdout, Write};
use std::str::FromStr;

use crate::aoc_day::{AocDay, Part};
use crate::command::{Command, CommandType, Terminal, format_duration};
use crate::web::WebContext;

pub struct Runner<> {
    pub day: AocDay,
    pub part: Part,
    pub terminal: Terminal,
    pub web_context: WebContext,
}
// TODO replace TODO with issues

impl Runner {
    pub fn new(day: AocDay, part: Part, web_context: WebContext) -> Self {
        Self {
            day,
            part,
            web_context,
            terminal: Terminal::new(),
        }
    }

    pub fn close(self) -> io::Result<()> {
        self.web_context.close()
    }

    pub fn make_missing_files_if_open(&mut self) -> Result<(), Box<dyn Error>> {
        let Self {
            ref day,
            ref mut terminal,
            ref mut web_context,
            ..
        } = self;
        if day.is_open() {
            if !day.has_source_code() {
                let mut child = process::Command::new("gucci")
                    .args(&[
                        "-s",
                        format!("day=day{}", day.day).as_str(),
                        "templates/day.rs.tpl"
                    ])
                    .stdout(process::Stdio::from(fs::File::create(day.source_code_path())?))
                    .stderr(process::Stdio::inherit())
                    .spawn()?;
                if !child.wait()?.success() {
                    return Err("Error while running gucci".into());
                }

                process::Command::new("git").args(&["add", day.source_code_path().as_str()]).status()?;
            }
            if !day.has_input_file() {
                let answer = terminal.yes_no(Some(true), Some("Fetch input file from Advent of Code website? "))?;
                let res: Result<(), Box<dyn Error>> = match answer {
                    Some(true) => {
                        println!("Fetching...");
                        web_context.curl_request_to_named_file(
                            day.input_file_url().as_str(),
                            day.input_file_path(),
                        )?;
                        process::Command::new("git").args(&["add", day.input_file_path().as_str()]).status()?;
                        return Ok(());
                    }
                    Some(false) | None => Ok(())
                };
                res?;
            }
            if !day.has_test_input_file() {
                fs::File::create(day.test_input_file_path())?;
                process::Command::new("git").args(&["add", day.test_input_file_path().as_str()]).status()?;

            }
        }
        return Ok(());
    }

    pub fn test(&self) -> Result<bool, Box<dyn Error>> {
        let output = process::Command::new("cargo")
            .args(&[
                "test",
                "-q",
                format!("test_day{}", self.day.day).as_str(),    // TODO specify which part to test
                "--bin",
                format!("day{}", self.day.day).as_str()
            ])
            .output()?;
        eprintln!("{}", String::from_utf8(output.stderr)?);
        println!("{}", String::from_utf8(output.stdout)?);
        return Ok(output.status.success());
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let test_succeeded = self.test()?;
        if test_succeeded || self.terminal.yes_no(Some(false), Some("Test failed, run anyways? "))?.unwrap_or(false) {
            let output_file = format!("outputs/day{}-{}.txt", self.day.day, self.part.as_str());
            let output = process::Command::new("cargo")
                .args(&[
                    "run",
                    "-q",
                    "--bin", format!("day{}", self.day.day).as_str(),
                    "--",
                    "--part", self.part.as_digit().to_string().as_str(),
                    "--output-file", self.day.output_file_path(&self.part).as_str()
                ])
                .output()?; // TODO Spawn and do fancy terminal things
            eprintln!("{}", String::from_utf8(output.stderr)?);
            println!("{}", String::from_utf8(output.stdout)?);
            if output.status.success() {
                let mut output_file = fs::File::open(output_file)?;
                let mut output = String::new();
                output_file.read_to_string(&mut output)?;
                println!("For day {} part {} got result {}",
                         self.terminal.strings.emph_style.paint(self.day.day.to_string()),
                         self.terminal.strings.emph_style.paint(&self.part.to_string()),
                         self.terminal.strings.emph_style.paint(&output)
                );
                // TODO Query puzzle status from AOC before submitting
                if self.terminal.yes_no(Some(true), Some("Submit to Advent of Code? "))?
                    .unwrap_or(false) {
                    self.do_submit(output)?;
                }
            }
        }
        return Ok(());
    }

    fn submit(&mut self) -> Result<(), Box<dyn Error>> {
        let output_age = self.day.output_file_age(&self.part)?;
        if let Some(output_age) = output_age {
            let mut output = String::new();
            fs::File::open(self.day.output_file_path(&self.part))?.read_to_string(&mut output)?;
            let answer = self.terminal.options(Some("r"), &["Rerun", "Submit", "Cancel"],
                                               Some(format!("Found output {} with age {} ",
                                                            self.terminal.strings.emph_style.paint(&output),
                                                            format_duration(output_age)).as_str()))?;

            println!("{:?}", &answer);
            if let Some(s) = answer {
                match s.as_str() {
                    "r" => self.run()?,
                    "s" => self.do_submit(output)?,
                    _ => (),
                }
            }
        } else {
            self.run()?
        }
        Ok(())
    }

    fn do_submit(&mut self, submission: String) -> Result<(), Box<dyn Error>> {
        // TODO get tls session from curl and wrap around lynx, OR parse output file into text or minimal html
        let data = format!("level={}&answer={}", self.part.as_digit(), submission);
        self.web_context.curl_post_to_lynx(self.day.submit_url().as_str(), data.as_bytes())
    }

    pub fn switch_part(&mut self) {
        match self.part {
            Part::ONE => self.part = Part::TWO,
            Part::TWO => self.part = Part::ONE,
        }
    }

    pub fn change_day(&mut self, args: &[&str]) -> Result<(), String> {
        let day = args.first()
            .ok_or("Expected 1 argument got 0")?
            .parse::<u8>()
            .map_err(|e| format!("Invalid argument 1: {}", e))?;
        if day <= 25 {
            self.day.day = day;
            Ok(())
        } else {
            Err(format!("AoC is played on 1-25 December, {} December is not a valid day.", day))
        }
    }

    pub fn change_year(&mut self, args: &[&str]) -> Result<(), String> {
        let year = args.first()
            .ok_or("Expected 1 argument got 0")?
            .parse::<u16>()
            .map_err(|e| format!("Invalid argument 1: {}", e))?;
        if year >= 2015 {
            self.day.year = year;
            Ok(())
        } else {
            Err(format!("AoC started in 2015, {} is not a valid year.", year))
        }
    }

    pub fn print_day(&self) {
        println!("Current Advent of Code Day is the {} of {}", self.day.day, self.day.year); // TODO make impl Display for AOCDay
    }

    pub fn start_runner(&mut self) -> Result<(), Box<dyn Error>> {
        self.print_day();
        self.terminal.help();
        self.make_missing_files_if_open()?;

        loop {
            stderr().flush()?;
            print!("> ");
            stdout().flush()?;
            let mut line = String::new();
            stdin().read_line(&mut line)?;
            let args: Vec<&str> = line.trim().split_whitespace().collect();
            match args.first().map(|s| Command::from_str(*s).map_err(|e| (s, e))) {
                None => (),
                Some(Err((s, _))) => eprintln!("Unknown command \"{}\"", *s),
                Some(Ok(command)) => {
                    let args = &args[1..];
                    match command.command_type {
                        CommandType::Test => self.test().map(|_| ()),
                        CommandType::Run => self.run(),
                        CommandType::Submit => self.submit(),
                        CommandType::NextPart => Ok(self.switch_part()),
                        CommandType::Puzzle => Ok(self.web_context.lynx(self.day.puzzle_url())?),
                        CommandType::Day => Ok({
                            self.change_day(args)?;
                            self.print_day();
                            self.make_missing_files_if_open()?;
                        }),
                        CommandType::Year => Ok({
                            self.change_year(args)?;
                            self.print_day();
                        }),
                        CommandType::Leaderboard => Ok(self.web_context.lynx(self.day.leaderboard_url())?),
                        CommandType::Help => Ok(self.terminal.help()),
                        CommandType::Quit => break,
                    }?;
                }
            }
        }

        return Ok(());
    }
}
