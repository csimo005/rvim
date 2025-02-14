use std::error::Error;

use crate::config::Config;

pub mod config;

pub fn run(_cfg: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
