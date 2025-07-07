use std::path::PathBuf;
use std::process::Output;

use serde;
use tokio::process::Command;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Module {
    pub standalone: Vec<Test>,
    pub specs: Vec<Spec>,
}

impl Module {
    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).expect("Have right format.")
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, Default)]
pub struct Spec {
    pub id: String,
    pub description: String,
    pub tests: Vec<Test>,
}

#[derive(Debug, PartialEq, serde::Deserialize, Default)]
pub struct Test {
    pub id: String,
    pub description: String,
    pub assertions: Vec<(String, bool)>,
}

pub async fn run_test_file(arturo: &String, test_file: PathBuf) -> Result<Output, std::io::Error> {
    let file = test_file.to_str().unwrap();

    Command::new(arturo)
        .arg(file)
        .arg(format!("--filename:{file}"))
        .output()
        .await
}

#[cfg(test)]
mod test {
    use std::env;
    use std::fs;
    use std::vec;

    use super::*;
    use tokio;

    #[tokio::test]
    async fn should_read_from_generated_file() {
        let _ = env::set_current_dir("..").unwrap();

        let arturo = String::from("./bin/arturo.exe");
        let file = PathBuf::from("specs/simple.spec.art");

        let _ = run_test_file(&arturo, file.clone()).await.unwrap();
        let json_file = format!("{}.json", file.to_str().unwrap());
        let result_file = PathBuf::from(".unitt").join(json_file);

        let json = fs::read_to_string(dbg!(result_file)).unwrap();

        let result = Module::from_json(&json);

        assert_eq!(
            result.standalone[0].description,
            "I should be standalone".to_string()
        );
        assert_eq!(
            result.standalone[0].assertions,
            vec![("string? \"I\\'m standalone\"".to_string(), true)]
        );

        assert_eq!(
            result.standalone[1].description,
            "I should be standalone #2".to_string()
        );
        assert_eq!(
            result.standalone[1].assertions,
            vec![("string? \"I\\'m standalone\"".to_string(), true)]
        );

        assert_eq!(result.specs[0].description, "I'm a scope".to_string());
        assert_eq!(
            result.specs[0].tests[0].description,
            "should be into Scope #1".to_string()
        );
        assert_eq!(
            result.specs[0].tests[0].assertions,
            vec![("true".to_string(), true)]
        );
        assert_eq!(
            result.specs[0].tests[1].description,
            "should be into Scope #1".to_string()
        );
        assert_eq!(
            result.specs[0].tests[1].assertions,
            vec![("false".to_string(), false)]
        );

        assert_eq!(result.specs[1].description, "scope #2".to_string());
        assert_eq!(
            result.specs[1].tests[0].description,
            "should be into Scope #2".to_string()
        );
        assert_eq!(
            result.specs[1].tests[0].assertions,
            vec![("true".to_string(), true)]
        );
    }

    #[tokio::test]
    async fn should_execute_arturo() {
        let _ = env::set_current_dir("..").unwrap();

        let arturo = String::from("./bin/arturo.exe");
        let file = PathBuf::from("specs/lib/collections/append.spec.art");
        let result = run_test_file(&arturo, file).await.unwrap();

        let _ = dbg!(String::from_utf8(result.clone().stdout)
            .unwrap()
            .split("\n")
            .collect::<Vec<&str>>());
        assert!(result.status.success());
    }

    #[tokio::test]
    async fn should_deserialize_json() {
        let example_file = r#"
        {
            "standalone": [
                {
                    "id": "standalone-1",
                    "description": "Standalone Test #1",
                    "assertions": [[ "char? 15", false ]]
                }
            ],
            "specs": [
                {
                    "id": "spec-1",
                    "description": "Spec #1",
                    "tests": [
                        {
                            "id": "test-1",
                            "description": "Test #1",
                            "assertions": [
                                [ "string? \"Arturo\"", true ],
                                [ "char? 15", false ]
                            ]
                        },             
                        {
                            "id": "test-2",
                            "description": "Test #2",
                            "assertions": [[ "char? 'A'", true ]]
                        }
                    ]
                }
            ]
        }
        "#;

        let expected = Module {
            standalone: vec![Test {
                id: "standalone-1".into(),
                description: "Standalone Test #1".into(),
                assertions: vec![("char? 15".into(), false)],
            }],
            specs: vec![Spec {
                id: "spec-1".into(),
                description: "Spec #1".into(),
                tests: vec![
                    Test {
                        id: "test-1".into(),
                        description: "Test #1".into(),
                        assertions: vec![
                            ("string? \"Arturo\"".into(), true),
                            ("char? 15".into(), false),
                        ],
                    },
                    Test {
                        id: "test-2".into(),
                        description: "Test #2".into(),
                        assertions: vec![("char? 'A'".into(), true)],
                    },
                ],
            }],
        };

        let actual = Module::from_json(example_file);
        assert_eq!(expected, actual);
    }
}
