use std::error::Error;

use crate::aoc_day::{AocDay, Part};
use crate::exec::Runner;

mod command;
mod web;
mod exec;
mod aoc_day;

fn main() -> Result<(), Box<dyn Error>> {
    let mut runner = Runner::new(
        AocDay::today(),
        Part::ONE,
        web::WebContext::new("web/cookie".to_owned(), "web/lynx.cfg".to_owned())?,
    );

    runner.start_runner()?;
    runner.close()?;

    return Ok(());
}


