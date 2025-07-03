use clap::Parser;
mod cli;
use std::path::PathBuf;
use std::env;
use std::fs;
use unitt::test::{result_of, read_result};
use unitt::statistics::Statistics;
use unitt::config::Config;
use glob::glob;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Arguments::parse();

    let _ = env::set_current_dir("..")?;

    // Load config from file if exists, otherwise use default
    let mut config = Config::from_toml(fs::read_to_string("./unitt.toml")?.as_str())?;

    if let Some(tests) = args.tests.as_ref() {
        config = config.with_tests(tests);
    }
    if let Some(cache) = args.cache.as_ref() {
        config = config.with_cache(cache);
    }
    if let Some(target) = args.target.as_ref() {
        config = config.with_target(target);
    }

    // Use glob to find all matching test files
    let pattern = format!("{}/{}", config.tests, config.target);
    for entry in glob(&pattern)? {
        let file = entry?;
        let arturo = PathBuf::from("./bin/arturo.exe");

        // Run Arturo and generate the result file
        let result = result_of(arturo.clone(), file.clone()).await?;
        if !result.status.success() {
            eprintln!("Arturo execution failed for {}", file.display());
            continue;
        }

        // Read the generated JSON result file
        let json_file = format!("{}.json", file.to_str().unwrap());
        let result_file = PathBuf::from(&config.cache).join(json_file);
        let json = fs::read_to_string(&result_file)?;
        let module = read_result(json).await;

        // Calculate and print statistics
        let stats = Statistics::from(module);
        println!("{} >> Passed: {} | Failed: {} | Skipped: {}", 
            file.to_str().unwrap(), 
            stats.passed, 
            stats.failed, 
            stats.skipped
        );
    }

    Ok(())
}
