use std;

use crate::collector::LoadedTest;
use crate::models::config::Config;
use crate::models::statistics::Statistics;
use crate::models::test;

pub struct Summary {
    pub status: Statistics,
    pub file_count: u64,
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Files: {} | Passed: {} | Failed: {} | Skipped: {}",
            self.file_count, self.status.passed, self.status.failed, self.status.skipped
        )
    }
}

pub fn print_test_result(test: &test::Test, failfast: bool) {
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
        let icon = if *result { "✅" } else { "❌" };
        println!("         {}: {}", icon, assertion);
    }

    if status == "❌" && failfast {
        panic!("         Aborting due to failfast mode!");
    }

}

pub fn display_tests(loaded_tests: &[LoadedTest], config: &Config) {
    for LoadedTest { filename, module } in loaded_tests {
        println!("\n===== {} =====\n", filename);

        for test in &module.standalone {
            print_test_result(test, config.fail_fast);
        }

        for spec in &module.specs {
            println!("\nSuite: {} \n", spec.description);
            for test in &spec.tests {
                print_test_result(test, config.fail_fast);
            }
        }

        let stats = Statistics::from(module);
        println!(
            "\n\n{} >> Passed: {} | Failed: {} | Skipped: {}",
            filename, stats.passed, stats.failed, stats.skipped
        );
    }
}

pub fn display_summary(summary: &Summary) {
    println!("\n===== {} =====\n", "Final Summary");
    println!("{}", summary);
}

pub fn summary_of(loaded_tests: &[LoadedTest]) -> Summary {
    let (total_stats, file_count) = loaded_tests
        .iter()
        .map(|LoadedTest { module, .. }| Statistics::from(module))
        .fold(
            (
                Statistics {
                    passed: 0,
                    failed: 0,
                    skipped: 0,
                },
                0u64,
            ),
            |(mut acc, count), stats| {
                acc.passed += stats.passed;
                acc.failed += stats.failed;
                acc.skipped += stats.skipped;
                (acc, count + 1)
            },
        );

    Summary {
        status: total_stats,
        file_count,
    }
}
