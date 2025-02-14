use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Optional name to operate on
    #[arg(short, long)]
    pub fname: Option<String>,
}
