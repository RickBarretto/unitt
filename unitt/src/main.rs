mod cli;
mod collector;
mod display;
mod models;
mod runner;

use std::{env, path::{Path, PathBuf}};

use clap::Parser;

use models::config::Config;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env::set_current_dir(Path::new(".."))?;

    let args = cli::Arguments::parse();
    let config: Config = cli::actual_config(&args)?;

    let arturo = config.arturo.clone();
    runner::reset_cache(config.cache.clone());
    runner::generate_tests(&config, &arturo).await;

    let tests: Vec<collector::LoadedTest> = collector::load_tests(&config).collect();
    let summary = display::summary_of(&tests);

    display::display_tests(&tests, &config);
    display::display_summary(&summary);

    if summary.status.failed > 0 && !args.suppress {
        eprintln!("Some tests failed!");
        std::process::exit(1);
    }

    Ok(())
}
