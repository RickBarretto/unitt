use std::fs;

use clap::Parser;

use crate::{cli, models::config::Config};

#[derive(Parser, Debug)]
#[command(name = "unitt")]
#[command(version = "v3.0.0")]
#[command(about = "Lean unit testing tool for Arturo")]
pub struct Arguments {
    #[arg(long)]
    pub tests: Option<String>,
    #[arg(long)]
    pub cache: Option<String>,
    #[arg(long)]
    pub target: Option<String>,

    #[arg(long)]
    pub fail_fast: bool,
    #[arg(long)]
    pub supress: bool,
}

pub fn actual_config(args: cli::Arguments) -> Result<Config, Box<dyn std::error::Error>> {
    let toml = fs::read_to_string("./unitt.toml")?;
    let config = Config::from_toml(&toml)?;
    Ok(Config {
        tests: args.tests.clone().unwrap_or(config.tests),
        cache: args.cache.clone().unwrap_or(config.cache),
        target: args.target.clone().unwrap_or(config.target),
    })
}
