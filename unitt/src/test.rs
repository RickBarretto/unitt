
use std::path::PathBuf;
use std::process::Output;

use serde;
use tokio::process::Command;

#[derive(Debug, PartialEq)]
#[derive(serde::Deserialize)]
struct Module {
    standalone: Vec<Test>,
    specs: Vec<Spec>
}

#[derive(Debug, PartialEq)]
struct Statistics {
    pub passed: u64,
    pub skipped: u64,
    pub failed: u64,
}

#[derive(Debug, PartialEq)]
#[derive(serde::Deserialize)]
struct Spec {
    id: String,
    tests: Vec<Test>
}

#[derive(Debug, PartialEq)]
#[derive(serde::Deserialize)]
struct Test {
    id: String,
    description: String,
    assertions: Vec<(String, bool)>
}

type Json = String;

async fn result_of(arturo: PathBuf, test_file: PathBuf) -> Result<Output, std::io::Error> {
    let program = arturo.to_str().unwrap();
    let file = test_file.to_str().unwrap();
    
    Command::new(program)
        .arg(file)
        .arg(format!("--filename:{file}"))
        .output().await
}

async fn read_result(result: Json) -> Module {
    serde_json::from_str(&result).expect("Have right format.")
}


#[cfg(test)]
mod test {
    use std::env;

    use tokio;
    use super::{*};

    #[tokio::test]
    async fn test_execute_arturo() {
        let _ = env::set_current_dir("..").unwrap();

        let arturo = PathBuf::from("./bin/arturo.exe");
        let file = PathBuf::from("specs/lib/collections/append.spec.art");
        let result = result_of(arturo, file).await.unwrap();

        let _ = dbg!(String::from_utf8(result.clone().stdout).unwrap().split("\n").collect::<Vec<&str>>());
        assert!(result.status.success());
    }

    #[tokio::test]
    async fn test_json_read() {

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
            standalone: vec![
                Test {
                    id: "standalone-1".into(),
                    description: "Standalone Test #1".into(),
                    assertions: vec![
                        ("char? 15".into(), false),
                    ]
                }
            ],
            specs: vec![
                Spec {
                    id: "spec-1".into(),
                    tests: vec![
                        Test {
                            id: "test-1".into(),
                            description: "Test #1".into(),
                            assertions: vec![
                                ("string? \"Arturo\"".into(), true),
                                ("char? 15".into(), false),
                            ]
                        }, 
                        Test {
                            id: "test-2".into(),
                            description: "Test #2".into(),
                            assertions: vec![
                                ("char? 'A'".into(), true),
                            ]
                        }
                    ]
                }
            ]
        };

        let actual = read_result(example_file.into());
        assert_eq!(expected, actual.await);

    }


}