use std;

use crate::collector::LoadedTest;
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


pub fn display_tests<'a, I>(loaded_tests: I) -> Summary
where
    I: IntoIterator<Item = LoadedTest>,
{
    let mut total_stats = Statistics { passed: 0, failed: 0, skipped: 0 };
    let mut file_count = 0u64;
    for LoadedTest {filename, module} in loaded_tests {
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
