mod cli;
mod collector;
mod display;
mod models;
mod runner;

use std::env;
use std::path::Path;

use clap::Parser;

use models::config::Config;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Arguments::parse();
    let _ = env::set_current_dir(&args.root);

    if !Path::new("unitt.toml").exists() {
        eprintln!("No `unitt.toml' found. Please create one and read our documentation for more information.");
        eprintln!("Github Repository: https://github.com/RickBarretto/unitt/.");
        std::process::exit(1);
    }

    let config: Config = args.clone()
        .merge_with(Config::from_toml("unitt.toml"));

    runner::reset_cache(config.cache.clone());
    runner::generate_tests(&config, &config.arturo).await;

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
