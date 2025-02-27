use std::error::Error;

use crate::app::App;
use crate::config::Config;

pub mod app;
pub mod config;
pub mod interface;
pub mod piece_table;
pub mod position;
pub mod views;

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let mut app = App::new(cfg);
    app.exec()?;

    Ok(())
}
