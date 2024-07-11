#![allow(dead_code)]

use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Config {
    pub test_subnets: HashMap<String, RoutingCountryConfig>,
    pub domain: Vec<DomainConfig>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DomainConfig {
    pub host: String,
    pub geo_routing: Vec<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct RoutingCountryConfig {
    pub subnets: Vec<String>,
}

#[derive(Clone)]
pub struct ConfigParser<T: for<'a> Deserialize<'a>> {
    config: T,
}

impl<C: for<'a> Deserialize<'a>> ConfigParser<C> {
    pub fn parse(contents: String) -> C {
        toml::from_str(&contents).unwrap()
    }
}

impl ConfigParser<Config> {
    pub fn new_with_path<T: ToString>(path: T) -> ConfigParser<Config> {
        let contents =
            fs::read_to_string(&path.to_string()).expect("Should have been able to read the file");

        ConfigParser {
            config: ConfigParser::parse(contents),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    const TEMP_DIR_PATH: &'static str = "./temp";
    const TEMP_PATH: &'static str = "./temp/domain.toml";

    fn setup() {
        if fs::read_dir(TEMP_DIR_PATH).is_err() {
            fs::create_dir_all(TEMP_DIR_PATH).expect("Unable to create directory");
        }

        let test_config = r#"
            [test_subnets]
            us = { subnets = ["44.208.193.0/24"] }

            [[domain]]
            host = "google.com"
            geo_routing = ["us"]
        "#
        .trim();

        fs::write(TEMP_PATH, test_config).expect("Unable to write file");
    }

    fn teardown() {
        fs::remove_dir_all(TEMP_DIR_PATH).expect("Unable to remove directory");
    }

    #[test]
    fn test_new_with_path() {
        setup();

        let parser = ConfigParser::new_with_path(TEMP_PATH);
        let config = parser.config();
        assert_eq!(config.domain.len(), 1);
        assert_eq!(config.test_subnets.len(), 1);
        assert_eq!(config.domain.len(), 1);
        assert_eq!(config.test_subnets.len(), 1);
        assert_eq!(config.domain[0].host, "google.com");
        assert_eq!(
            config.test_subnets.get("us").unwrap().subnets[0],
            "44.208.193.0/24"
        );

        teardown();
    }

    #[test]
    fn test_parse() {
        let test_config = r#"
            [test_subnets]
            us = { subnets = ["44.208.193.0/24"] }

            [[domain]]
            host = "google.com"
            geo_routing = ["us"]
        "#;

        let config: Config = ConfigParser::parse(test_config.to_string());
        assert_eq!(config.domain.len(), 1);
        assert_eq!(config.test_subnets.len(), 1);
        assert_eq!(config.domain[0].host, "google.com");
        assert_eq!(
            config.test_subnets.get("us").unwrap().subnets[0],
            "44.208.193.0/24"
        );
    }
}
