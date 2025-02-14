use clap::Parser;
use std::error::Error;

use rvim::config::Config;
use rvim::run;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::parse();
    run(cfg)?;

    Ok(())
}
