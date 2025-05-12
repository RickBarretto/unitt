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
    use std::default::Default;

    use super::*;

    fn truly() -> (String, bool) {
        (String::from(""), true)
    }
    
    fn falsy() -> (String, bool) {
        (String::from(""), false)
    }

    #[tokio::test]
    async fn test_statistics_from_with_standalone_tests() {

        let standalone = vec![
            Test { assertions: vec![falsy()], ..Default::default() },
            Test { assertions: vec![truly()], ..Default::default() },
            Test { assertions: vec![truly()], ..Default::default() },
        ];

        let stats = Statistics::from(Module { standalone, specs: vec![]});

        assert_eq!(stats.passed, 2);
        assert_eq!(stats.failed, 1);
        assert_eq!(stats.skipped, 0);
    }

    #[tokio::test]
    async fn test_statistics_from_with_specs() {
        let specs = vec![
            Spec {
                id: "".into(),
                description: "".into(),
                tests: vec![
                    Test { assertions: vec![truly(), falsy()], ..Default::default()}, 
                    Test { assertions: vec![truly()], ..Default::default() }
                ]
            },
            Spec {
                id: "".into(), description: "".into(),
                tests: vec![
                    Test { assertions: vec![truly(), falsy()], ..Default::default() }, 
                    Test { assertions: vec![truly()], ..Default::default() }
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
            Test {assertions:vec![truly()], ..Default::default() },
            Test {assertions:vec![falsy()], ..Default::default() },
        ];
        let specs = vec![Spec {
            tests: vec![Test{ assertions:vec![truly(), falsy()], ..Default::default() }], 
            ..Default::default()
        }];

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

