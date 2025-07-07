use std::path::PathBuf;

use clap::Parser;

use crate::models::config::Config;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(name = "unitt")]
#[command(version, author="RickBarretto")]
#[command(about = "Lean unit testing tool for Arturo")]
pub struct Arguments {
    // Arguments that can override the default configuration
    #[arg(long, help="Path to test files directory.")]
    pub tests: Option<String>,
    #[arg(long, help="Path to cache directory.")]
    pub cache: Option<String>,
    #[arg(long, help="Glob pattern to match test files.")]
    pub target: Option<String>,
    #[arg(long, help="Path to the Arturo binary.")]
    pub arturo: Option<String>,

    // Arguments that control the behavior of the tool
    #[arg(long, default_value=".", help="Root directory that contains `unitt.toml`.")]
    pub root: PathBuf,
    #[arg(long, help="Exits on first failure found.")]
    pub fail_fast: bool,
    #[arg(long, help="Suppresses error messages on test failures. Also disables exit code 1 on failure.")]
    pub suppress: bool,
}

impl Arguments {
    pub fn merge_with(self, config: Config) -> Config {
        Config {
            cache: self.cache.unwrap_or(config.cache),
            tests: self.tests.unwrap_or(config.tests),
            target: self.target.unwrap_or(config.target),
            arturo: self.arturo.unwrap_or(config.arturo),
            fail_fast: self.fail_fast,
        }
    }
}