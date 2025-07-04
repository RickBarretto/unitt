
use std::fs;
use std::path::PathBuf;

use glob::glob;

use crate::models::config::Config;
use crate::models::test;

pub struct LoadedTest {
    pub filename: String,
    pub module: test::Module,
}

pub fn load_tests(config: &Config) -> impl Iterator<Item = LoadedTest> {
    let pattern = config.target.clone();
    let files: Vec<_> = glob(&pattern).unwrap().filter_map(Result::ok).collect();
    let cache = config.cache.clone();

    files.into_iter().map(move |file| {
        let module = module_from_path(cache.clone(), &file);

        LoadedTest {
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