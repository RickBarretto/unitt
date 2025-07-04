use std::{fs, path::PathBuf};

use glob::glob;

use crate::models::config::Config;
use crate::models::statistics::Statistics;
use crate::models::test;

pub struct Summary {
    status: Statistics,
    file_count: u64,
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Files: {} | Passed: {} | Failed: {} | Skipped: {}", 
            self.file_count, self.status.passed, self.status.failed, self.status.skipped)
    }
}

pub fn print_test_result(test: &test::Test) {
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

pub fn display_tests(config: &Config) -> Summary {
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