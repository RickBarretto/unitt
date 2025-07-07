use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[derive(PartialEq)]
pub struct Config {
    #[serde(default)]
    pub cache: String,
    #[serde(default)]
    pub tests: String,
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub arturo: String,
    #[serde(default)]
    pub fail_fast: bool,
}

impl Config {

    pub fn from_toml(path: &str) -> Config {
        match fs::read_to_string(&path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error parsing TOML file {}: {}", path, e);
                    Config::default()
                }
            },
            Err(e) => {
                eprintln!("Error reading TOML file {}: {}", path, e);
                Config::default()
            }
        }
    }

    pub fn from_str(toml_content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_content)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache: ".unitt".into(),
            tests: "specs".into(),
            target: "*.spec.art".into(),
            arturo: "arturo".into(),
            fail_fast: false,
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
            arturo = "custom_arturo"
        "#;

        let config: Config = Config::from_str(toml_content).unwrap();
        assert_eq!(config.cache, "custom_cache");
        assert_eq!(config.tests, "custom_tests");
        assert_eq!(config.target, "custom_target");
        assert_eq!(config.arturo, "custom_arturo");
    }

    #[test]
    fn test_from_toml_str_invalid() {
        let toml_content = r#"
            invalid_field; = "value"
        "#;

        let result = Config::from_str(toml_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_toml_str_empty() {
        let toml_content = "";

        let result = Config::from_str(toml_content).unwrap();
        assert_eq!(result, Config::default());
    }
}
