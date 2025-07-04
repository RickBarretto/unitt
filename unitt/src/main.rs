use clap::Parser;
mod cli;
mod test;
mod config;
mod statistics;

use std::path::PathBuf;
use std::env;
use std::fs;
use test::{run_test_file};
use statistics::Statistics;
use config::Config;
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
            let result = run_test_file(arturo, file.clone()).await;
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

    display_tests(&config, &mut total_stats, &mut file_count)?;

    println!("\nSummary: Files: {} | Passed: {} | Failed: {} | Skipped: {}", 
        file_count, total_stats.passed, total_stats.failed, total_stats.skipped);

    Ok(())
}

fn display_tests(config: &Config, total_stats: &mut Statistics, file_count: &mut u64) -> Result<(), Box<dyn std::error::Error>> {
    for ModuleStreamItem {filename, module} in all_modules_of(config) {

        println!("\n===== {} =====\n", filename);
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

        let stats = Statistics::from(module);
        println!("\n\n{} >> Passed: {} | Failed: {} | Skipped: {}", 
            filename, 
            stats.passed, 
            stats.failed, 
            stats.skipped
        );
        total_stats.passed += stats.passed;
        total_stats.failed += stats.failed;
        total_stats.skipped += stats.skipped;
        *file_count += 1;
    }
    Ok(())
}

struct ModuleStreamItem {
    pub filename: String,
    pub module: test::Module,
}


fn all_modules_of(config: &Config) -> impl Iterator<Item = ModuleStreamItem> {
    let pattern = config.target.clone();
    let files: Vec<_> = glob(&pattern).unwrap().filter_map(Result::ok).collect();
    let cache = config.cache.clone();

    files.into_iter().map(move |file| {
        let module = module_from_path(cache.clone(), &file);

        ModuleStreamItem {
            filename: file.file_name().unwrap().to_string_lossy().into(),
            module,
        }
    })
}

fn module_from_path(cache: String, file: &PathBuf) -> test::Module {
    let result_file: PathBuf = PathBuf::from(&cache).join(format!("{}.json", file.to_string_lossy()));
    let json: String = fs::read_to_string(&result_file).unwrap_or_default();
    test::Module::from_json(&json)
}
