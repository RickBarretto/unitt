use clap::Parser;
mod cli;
use std::path::PathBuf;
use std::env;
use std::fs;
use unitt::test::{result_of, read_result};
use unitt::statistics::Statistics;
use unitt::config::Config;
use glob::glob;
use tokio::task::JoinSet;

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

    // Global statistics
    let mut total_stats = Statistics { passed: 0, failed: 0, skipped: 0 };
    let mut file_count = 0u64;

    // Use glob to find all matching test files
    let pattern = format!("{}/{}", config.tests, config.target);

    // Collect all test files first
    let mut test_files = Vec::new();
    for entry in glob(&pattern)? {
        let file = entry?;
        test_files.push(file);
    }

    let arturo = PathBuf::from("./bin/arturo.exe");

    // Run Arturo on all test files concurrently using tokio
    let mut join_set = JoinSet::new();

    for file in &test_files {
        let arturo = arturo.clone();
        let file = file.clone();
        join_set.spawn(async move {
            let result = result_of(arturo, file.clone()).await;
            (file, result)
        });
    }

    while let Some(res) = join_set.join_next().await {
        let (file, result) = res?;
        if let Err(e) = result {
            eprintln!("Arturo execution failed for {}: {}", file.display(), e);
            continue;
        }
        if !result.unwrap().status.success() {
            eprintln!("Arturo execution failed for {}", file.display());
            continue;
        }
    }

    for entry in glob(&pattern)? {
        let file = entry?;

        // Read the generated JSON result file
        let json_file = format!("{}.json", file.to_str().unwrap());
        let result_file = PathBuf::from(&config.cache).join(json_file);
        let json = fs::read_to_string(&result_file)?;
        let module = read_result(json).await;

        // Display detailed tests results

        println!("\n===== {} =====\n", file.file_name().unwrap().to_string_lossy());
        for test in &module.standalone {
            let all_passed = test.assertions.iter().all(|(_, r)| *r);
            let all_failed = test.assertions.iter().all(|(_, r)| !*r);
            let status = if all_passed {
                "✅"
            } else if all_failed {
                "❌"
            } else {
                "❌"
            };
            println!("    {} - {}", status, test.description);
            for (assertion, result) in &test.assertions {
                let icon = if *result {"✅"} else {"❌"};
                println!("         {}: {}", icon, assertion);
            }
        }

        for spec in &module.specs {
            println!("\nSuite: {} \n", spec.description);
            for test in &spec.tests {
                let all_passed = test.assertions.iter().all(|(_, r)| *r);
                let all_failed = test.assertions.iter().all(|(_, r)| !*r);
                let skipped = test.assertions.is_empty();
                let status = if skipped {
                    "⏩"
                } else if all_passed {
                    "✅"
                } else if all_failed {
                    "❌"
                } else {
                    "❌"
                };
                println!("    {} - {}", status, test.description);
                if skipped {
                    println!("         skipped!");
                } else {
                    for (assertion, result) in &test.assertions {
                        let icon = if *result {"✅"} else {"❌"};
                        println!("         {}: {}", icon, assertion);
                    }
                }
            }
        }

        // Calculate and print statistics
        let stats = Statistics::from(module);
        println!("\n\n{} >> Passed: {} | Failed: {} | Skipped: {}", 
            file.to_str().unwrap(), 
            stats.passed, 
            stats.failed, 
            stats.skipped
        );
        total_stats.passed += stats.passed;
        total_stats.failed += stats.failed;
        total_stats.skipped += stats.skipped;
        file_count += 1;
    }

    // Print global statistics
    println!("\nSummary: Files: {} | Passed: {} | Failed: {} | Skipped: {}", 
        file_count, total_stats.passed, total_stats.failed, total_stats.skipped);

    Ok(())
}
