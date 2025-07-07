use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "unitt")]
#[command(version, author="RickBarretto")]
#[command(about = "Lean unit testing tool for Arturo")]
pub struct Arguments {
    #[arg(long, default_value="specs", help="Path to test files directory.")]
    pub tests: String,
    #[arg(long, default_value="unitt", help="Path to cache directory.")]
    pub cache: String,
    #[arg(long, default_value="**/*.spec.art", help="Glob pattern to match test files.")]
    pub target: String,

    #[arg(long, help="Exits on first failure found.")]
    pub fail_fast: bool,
    #[arg(long, help="Suppresses error messages on test failures. Also disables exit code 1 on failure.")]
    pub suppress: bool,

    #[arg(long, default_value="arturo", help="Path to the Arturo binary.")]
    pub arturo: String,

    #[arg(long, default_value=".", help="Root directory for the tests.")]
    pub root: PathBuf,
}