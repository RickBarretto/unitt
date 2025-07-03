use clap::Parser;
mod cli;
use std::path::PathBuf;
use std::env;
use std::fs;
use unitt::test::{result_of, read_result};
use unitt::statistics::Statistics;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = cli::Arguments::parse();

    let _ = env::set_current_dir("..")?;
    let arturo = PathBuf::from("./bin/arturo.exe");
    let file = PathBuf::from("specs/lib/collections/append.spec.art");

    // Run Arturo and generate the result file
    let result = result_of(arturo.clone(), file.clone()).await?;
    if !result.status.success() {
        eprintln!("Arturo execution failed");
        std::process::exit(1);
    }

    // Read the generated JSON result file
    let json_file = format!("{}.json", file.to_str().unwrap());
    let result_file = PathBuf::from(".unitt").join(json_file);
    let json = fs::read_to_string(&result_file)?;
    let module = read_result(json).await;

    // Calculate and print statistics
    let stats = Statistics::from(module);
    println!("Passed: {} | Failed: {} | Skipped: {}", stats.passed, stats.failed, stats.skipped);

    Ok(())
}
