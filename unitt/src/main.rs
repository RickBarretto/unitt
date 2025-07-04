use clap::Parser;
mod cli;
mod models;

use models::config::{Config};
use models::{test};
use models::test::{run_test_file};
use models::statistics::{Statistics};


use std::path::PathBuf;
use std::env;
use std::fs;
use glob::glob;
use tokio::task::JoinSet;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env::set_current_dir("..")?;

    let args = cli::Arguments::parse();
    let config: Config = actual_config(args)?;
    let pattern = format!("{}/{}", config.tests, config.target);

    let arturo = PathBuf::from("./bin/arturo.exe");
    collect_tests(&pattern, &arturo).await;

    println!("\nFinal Summary:");
    println!("{}", display_tests(&config));

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

fn actual_config(args: cli::Arguments) -> Result<Config, Box<dyn std::error::Error>> {
    let toml = fs::read_to_string("./unitt.toml")?;
    let config = Config::from_toml(&toml)?;
    Ok(Config {
        tests: args.tests.clone().unwrap_or(config.tests),
        cache: args.cache.clone().unwrap_or(config.cache),
        target: args.target.clone().unwrap_or(config.target),
    })
}

fn print_test_result(test: &test::Test) {
    let all_passed = test.assertions.iter().all(|(_, r)| *r);
    let all_failed = test.assertions.iter().all(|(_, r)| !*r);
    let skipped = test.assertions.is_empty();

    let status = match (skipped, all_passed, all_failed) {
        (true, _, _) => "⏩",
        (false, true, _) => "✅",
        _ => "❌",
    };

    println!("    {} - {}", status, test.description);

    if skipped {
        println!("         skipped!");
        return;
    }

    for (assertion, result) in &test.assertions {
        let icon = if *result {"✅"} else {"❌"};
        println!("         {}: {}", icon, assertion);
    }

}

fn display_tests(config: &Config) -> Summary {
    let mut total_stats = Statistics { passed: 0, failed: 0, skipped: 0 };
    let mut file_count = 0u64;
    for ModuleStreamItem {filename, module} in all_modules_of(config) {

        println!("\n===== {} =====\n", filename);
        for test in &module.standalone {
            print_test_result(test);
        }
        for spec in &module.specs {
            println!("\nSuite: {} \n", spec.description);
            for test in &spec.tests {
                print_test_result(test);
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
        file_count += 1;
    }

    Summary { status: total_stats, file_count }
}

struct Summary {
    status: Statistics,
    file_count: u64,
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Files: {} | Passed: {} | Failed: {} | Skipped: {}", 
            self.file_count, self.status.passed, self.status.failed, self.status.skipped)
    }
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
