use super::test;

#[derive(Debug, PartialEq)]
pub struct Statistics {
    pub passed: u64,
    pub skipped: u64,
    pub failed: u64,
}

impl Statistics {
    pub fn from(module: &test::Module) -> Self {
        let all_specs = module
            .specs
            .iter()
            .flat_map(|spec| spec.tests.iter())
            .flat_map(|test| &test.assertions);

        let all_assertions = module
            .standalone
            .iter()
            .flat_map(|test| &test.assertions)
            .chain(all_specs);

        let (passed, failed) =
            all_assertions.fold((0u64, 0u64), |(passed, failed), &(_, result)| {
                if result {
                    (passed + 1, failed)
                } else {
                    (passed, failed + 1)
                }
            });

        Statistics {
            passed,
            failed,
            skipped: 0,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;
    use test::{Module, Spec, Test};

    fn truly() -> (String, bool) {
        (String::from(""), true)
    }

    fn falsy() -> (String, bool) {
        (String::from(""), false)
    }

    #[tokio::test]
    async fn test_statistics_from_with_standalone_tests() {
        let standalone = vec![
            Test {
                assertions: vec![falsy()],
                ..Default::default()
            },
            Test {
                assertions: vec![truly()],
                ..Default::default()
            },
            Test {
                assertions: vec![truly()],
                ..Default::default()
            },
        ];

        let stats = Statistics::from(&Module {
            standalone,
            specs: vec![],
        });

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
                    Test {
                        assertions: vec![truly(), falsy()],
                        ..Default::default()
                    },
                    Test {
                        assertions: vec![truly()],
                        ..Default::default()
                    },
                ],
            },
            Spec {
                id: "".into(),
                description: "".into(),
                tests: vec![
                    Test {
                        assertions: vec![truly(), falsy()],
                        ..Default::default()
                    },
                    Test {
                        assertions: vec![truly()],
                        ..Default::default()
                    },
                ],
            },
        ];

        let expected = Module {
            standalone: vec![],
            specs,
        };

        let stats = Statistics::from(&expected);

        assert_eq!(stats.passed, 4);
        assert_eq!(stats.failed, 2);
        assert_eq!(stats.skipped, 0);
    }

    #[tokio::test]
    async fn test_statistics_from_with_mixed_data() {
        let standalone = vec![
            Test {
                assertions: vec![truly()],
                ..Default::default()
            },
            Test {
                assertions: vec![falsy()],
                ..Default::default()
            },
        ];
        let specs = vec![Spec {
            tests: vec![Test {
                assertions: vec![truly(), falsy()],
                ..Default::default()
            }],
            ..Default::default()
        }];

        let module = Module { standalone, specs };

        let stats = Statistics::from(&module);

        assert_eq!(stats.passed, 2);
        assert_eq!(stats.failed, 2);
        assert_eq!(stats.skipped, 0);
    }
}
