use clap::Parser;
mod cli;
mod collector;
mod display;
mod models;
mod runner;

use models::config::{Config};

use std::path::PathBuf;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env::set_current_dir("..")?;

    let args = cli::Arguments::parse();
    let config: Config = cli::actual_config(args)?;
    let pattern = format!("{}/{}", config.tests, config.target);

    let arturo = PathBuf::from("./bin/arturo.exe");
    runner::reset_cache(config.cache.clone());
    runner::generate_tests(&pattern, &arturo).await;

    let tests = collector::load_tests(&config);

    println!("\nFinal Summary:");
    println!("{}", display::display_tests(tests));

    Ok(())
}
