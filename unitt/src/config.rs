use serde::Deserialize;

#[derive(Deserialize)]
struct Proxy {
    pub cache: Option<String>,
    pub tests: Option<String>,
    pub target: Option<String>,
} 


#[derive(PartialEq, Debug)]
pub struct Config {
    pub cache: String,
    pub tests: String,
    pub target: String,
}

impl Config {
    pub fn from_toml(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let proxy: Proxy = toml::from_str(content)?;
        let default = Config::default();

        Ok(Config {
            cache: proxy.cache.unwrap_or(default.cache),
            tests: proxy.tests.unwrap_or(default.tests),
            target: proxy.target.unwrap_or(default.target),
        })
    }

    pub fn with_cache(mut self, cache: &str) -> Self {
        self.cache = cache.into();
        self
    }

    pub fn with_tests(mut self, tests: &str) -> Self {
        self.tests = tests.into();
        self
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target = target.into();
        self
    }

}

impl Default for Config {
    fn default() -> Self {
        Self { 
            cache: ".unitt".into(),
            tests: "specs".into(),
            target: "*.spec.art".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.cache, ".unitt");
        assert_eq!(config.tests, "specs");
        assert_eq!(config.target, "*.spec.art");
    }

    #[test]
    fn test_from_toml_str_valid() {
        let toml_content = r#"
            cache = "custom_cache"
            tests = "custom_tests"
            target = "custom_target"
        "#;

        let config: Config = Config::from_toml(toml_content).unwrap();
        assert_eq!(config.cache, "custom_cache");
        assert_eq!(config.tests, "custom_tests");
        assert_eq!(config.target, "custom_target");
    }

    #[test]
    fn test_from_toml_str_invalid() {
        let toml_content = r#"
            invalid_field; = "value"
        "#;

        let result = Config::from_toml(toml_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_toml_str_empty() {
        let toml_content = "";

        let result = Config::from_toml(toml_content).unwrap();
        assert_eq!(result, Config::default());
    }

    #[test]
    fn test_with_cache() {
        let config = Config::default().with_cache("new_cache");
        assert_eq!(config.cache, "new_cache");
        assert_eq!(config.tests, "specs");
        assert_eq!(config.target, "*.spec.art");
    }

    #[test]
    fn test_with_tests() {
        let config = Config::default().with_tests("new_tests");
        assert_eq!(config.cache, ".unitt");
        assert_eq!(config.tests, "new_tests");
        assert_eq!(config.target, "*.spec.art");
    }

    #[test]
    fn test_with_target() {
        let config = Config::default().with_target("new_target");
        assert_eq!(config.cache, ".unitt");
        assert_eq!(config.tests, "specs");
        assert_eq!(config.target, "new_target");
    }

    #[test]
    fn test_with_methods_combined() {
        let config = Config::default()
            .with_cache("combined_cache")
            .with_tests("combined_tests")
            .with_target("combined_target");
        assert_eq!(config.cache, "combined_cache");
        assert_eq!(config.tests, "combined_tests");
        assert_eq!(config.target, "combined_target");
    }

}


