use clap::Parser;
mod cli;
mod display;
mod models;

use models::config::{Config};
use models::test::{run_test_file};


use std::path::PathBuf;
use std::env;
use glob::glob;
use tokio::task::JoinSet;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env::set_current_dir("..")?;

    let args = cli::Arguments::parse();
    let config: Config = cli::actual_config(args)?;
    let pattern = format!("{}/{}", config.tests, config.target);

    let arturo = PathBuf::from("./bin/arturo.exe");
    collect_tests(&pattern, &arturo).await;

    println!("\nFinal Summary:");
    println!("{}", display::display_tests(&config));

    Ok(())
}

async fn collect_tests(pattern: &str, arturo: &PathBuf) {
    let test_files: Vec<_> = glob(pattern)
        .expect("Invalid glob pattern")
        .filter_map(Result::ok)
        .collect();

    let mut join_set = JoinSet::new();
    for file in &test_files {
        let (arturo, file) = (arturo.clone(), file.clone());
        join_set.spawn(async move {
            let result = run_test_file(arturo, file.clone()).await;
            (file, result)
        });
    }

    while let Some(res) = join_set.join_next().await {
        let (file, result) = res.expect("JoinSet error");
        if let Err(e) = result {
            eprintln!("Arturo execution failed for {}: {}", file.display(), e);
            continue;
        }
        if !result.unwrap().status.success() {
            eprintln!("Arturo execution failed for {}", file.display());
            continue;
        }
    }
}
