use std::fs;


use serde::Deserialize;

#[derive(Deserialize)]
pub struct UnittConfig {
    pub cache: String,
    pub tests: String,
    pub target: String,
}


impl Default for UnittConfig {
    fn default() -> Self {
        Self {
            cache: ".unitt".into(), 
            tests: "specs".into(), 
            target: "*.spec.art".into() 
        }
    }
}

impl UnittConfig {
    pub fn from_toml_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: UnittConfig = toml::from_str(content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = UnittConfig::default();
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

        let config: UnittConfig = UnittConfig::from_toml_str(toml_content).unwrap();
        assert_eq!(config.cache, "custom_cache");
        assert_eq!(config.tests, "custom_tests");
        assert_eq!(config.target, "custom_target");
    }

    #[test]
    fn test_from_toml_str_invalid() {
        let toml_content = r#"
            invalid_field = "value"
        "#;

        let result = UnittConfig::from_toml_str(toml_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_toml_str_empty() {
        let toml_content = "";

        let result = UnittConfig::from_toml_str(toml_content);
        assert!(result.is_err());
    }
}


