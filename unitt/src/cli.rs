use std::fs;
use std::path::PathBuf;

use clap::Parser;

use crate::cli;
use crate::models::config::Config;

#[derive(Parser, Debug)]
#[command(name = "unitt")]
#[command(version, author="RickBarretto")]
#[command(about = "Lean unit testing tool for Arturo")]
pub struct Arguments {
    #[arg(long, default_value="specs", help="Path to test files directory.")]
    pub tests: Option<String>,

    #[arg(long, default_value="unitt", help="Path to cache directory.")]
    pub cache: Option<String>,
    
    #[arg(long, default_value="**/*.spec.art", help="Glob pattern to match test files.")]
    pub target: Option<String>,

    #[arg(long, help="Exits on first failure found.")]
    pub fail_fast: bool,
    #[arg(long, help="Suppresses error messages on test failures. Also disables exit code 1 on failure.")]
    pub suppress: bool,

    #[arg(long, default_value="arturo", help="Path to the Arturo binary.")]
    pub arturo: Option<String>,

    #[arg(long, default_value=".", help="Root directory for the tests.")]
    pub root: PathBuf,
}

pub fn actual_config(args: &cli::Arguments) -> Result<Config, Box<dyn std::error::Error>> {
    let toml = fs::read_to_string("./unitt.toml")?;
    let config = Config::from_toml(&toml)?;
    Ok(Config {
        tests: args.tests.clone().unwrap_or(config.tests),
        cache: args.cache.clone().unwrap_or(config.cache),
        target: args.target.clone().unwrap_or(config.target),
        fail_fast: args.fail_fast,
        arturo: args.arturo.clone().unwrap_or(config.arturo),
    })
}
