use std::error::Error;

use crate::config::Config;

pub mod config;
pub mod piece_table;
pub mod text_view;

pub fn run(_cfg: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
