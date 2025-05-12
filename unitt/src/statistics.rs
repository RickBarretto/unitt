use crate::test;
use crate::test::{Module, Spec, Test};


#[derive(Debug, PartialEq)]
pub struct Statistics {
    pub passed: u64,
    pub skipped: u64,
    pub failed: u64,
}

impl Statistics {
    pub fn from(module: test::Module) -> Self {
        let mut passed = 0u64;
        let mut failed = 0u64;
        let skipped = 0u64;

        // Count assertions in standalone tests
        module.standalone.iter().for_each(|test| {
            for (_, result) in &test.assertions {
                if *result {
                    passed += 1;
                } else {
                    failed += 1;
                }
            }
        });
        // Count assertions in specs
        for spec in &module.specs {
            for test in &spec.tests {
                for (_, result) in &test.assertions {
                    if *result {
                        passed += 1;
                    } else {
                        failed += 1;
                    }
                }
            }
        }
        Statistics { passed, skipped, failed }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_statistics_from_with_standalone_tests() {

        let standalone = vec![
            Test {
                id: "standalone-1".into(),
                description: "Standalone Test #1".into(),
                assertions: vec![
                    ("char? 15".into(), false),
                ]
            },
            Test {
                id: "standalone-2".into(),
                description: "Standalone Test #1".into(),
                assertions: vec![
                    ("char? 15".into(), true),
                ]
            },
            Test {
                id: "standalone-3".into(),
                description: "Standalone Test #1".into(),
                assertions: vec![
                    ("char? 15".into(), true),
                ]
            },
        ];

        let expected = Module {
            standalone,
            specs: vec![]
        };

        let stats = Statistics::from(expected);

        assert_eq!(stats.passed, 2);
        assert_eq!(stats.failed, 1);
        assert_eq!(stats.skipped, 0);
    }

    #[tokio::test]
    async fn test_statistics_from_with_specs() {
        let specs = vec![
            Spec {
                id: "spec-1".into(),
                description: "Spec #1".into(),
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
            },
            Spec {
                id: "spec-2".into(),
                description: "Spec #2".into(),
                tests: vec![
                    Test {
                        id: "test-3".into(),
                        description: "Test #1".into(),
                        assertions: vec![
                            ("string? \"Arturo\"".into(), true),
                            ("char? 15".into(), false),
                        ]
                    }, 
                    Test {
                        id: "test-4".into(),
                        description: "Test #2".into(),
                        assertions: vec![
                            ("char? 'A'".into(), true),
                        ]
                    }
                ]
            }
        ];

        let expected = Module {
            standalone: vec![],
            specs
        };

        let stats = Statistics::from(expected);

        assert_eq!(stats.passed, 4);
        assert_eq!(stats.failed, 2);
        assert_eq!(stats.skipped, 0);
    }

    #[tokio::test]
    async fn test_statistics_from_with_mixed_data() {
        let standalone = vec![
            Test {assertions:vec![("assert1".to_string(),true)], id: "".into(), description: "".into() },
            Test {assertions:vec![("assert2".to_string(),false)], id: "".into(), description: "".into() },
        ];
        let specs = vec![Spec {tests:vec![
            Test{assertions:vec![
                ("assert3".to_string(),true),
                ("assert4".to_string(),false)], 
                id: "".into(), description: "".into() }
                ], 
            id: "".into(), description: "".into() }
        ];

        let module = Module {
            standalone,
            specs,
        };

        let stats = Statistics::from(module);

        assert_eq!(stats.passed, 2);
        assert_eq!(stats.failed, 2);
        assert_eq!(stats.skipped, 0);
    }
}

