use std::fs;

use glob::glob;
use tokio::task::JoinSet;

use crate::models::config::Config;
use crate::models::test::run_test_file;

pub fn reset_cache(cache: String) {
    let _ = fs::remove_dir_all(cache);
}

pub async fn generate_tests(config: &Config, arturo: &String) {
    let pattern = format!("{}/{}", config.tests, config.target);
    let test_files: Vec<_> = glob(&pattern)
        .expect("Invalid glob pattern")
        .filter_map(Result::ok)
        .collect();

    let mut join_set = JoinSet::new();
    for file in &test_files {
        let (arturo, file) = (arturo.clone(), file.clone());
        join_set.spawn(async move {
            let result = run_test_file(&arturo, file.clone()).await;
            (file, result)
        });
    }

    while let Some(res) = join_set.join_next().await {
        let (file, result) = res.expect("JoinSet error");
        if let Err(err) = result {
            panic!("Arturo execution failed for {}: {}", file.display(), err);
        }
        let output = result.unwrap();
        if !output.status.success() {
            panic!(
                "Arturo execution failed for {}:\n{}",
                file.display(),
                String::from_utf8_lossy(&output.stdout)
            );
        }
    }
}
