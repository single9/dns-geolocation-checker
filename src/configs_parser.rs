#![allow(dead_code)]

use serde::Deserialize;
use std::{collections::HashMap, fs};

use crate::ip_geo_client::IpGeoProviderType;

/// A struct to hold the parsed config
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Config {
    /// The IP geo provider
    #[serde(default)]
    pub ip_geo_provider: IpGeoProviderType,
    #[serde(default)]
    pub mmdb_path: Option<String>,
    /// A map of country codes to their respective subnets
    pub test_subnets: HashMap<String, RoutingCountryConfig>,
    /// A list of domains and their respective geo routing
    pub domain: Vec<DomainConfig>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DomainConfig {
    /// The host of the domain
    pub host: String,
    /// A list of country codes to route to
    pub geo_routing: Vec<String>,
}

/// A struct to hold the subnets for a country
#[derive(Default, Debug, Clone, Deserialize)]
pub struct RoutingCountryConfig {
    /// A list of subnets
    pub subnets: Vec<String>,
}

#[derive(Clone)]
pub struct ConfigParser<T: for<'a> Deserialize<'a>> {
    /// The parsed config
    config: T,
}

impl<C: for<'a> Deserialize<'a>> ConfigParser<C> {
    /// Parse the contents
    pub fn parse(contents: String) -> C {
        toml::from_str(&contents).unwrap()
    }
}

impl ConfigParser<Config> {
    /// Create a new ConfigParser with the contents of a file
    pub fn new_with_path<T: ToString>(path: T) -> ConfigParser<Config> {
        let contents =
            fs::read_to_string(&path.to_string()).expect("Should have been able to read the file");

        ConfigParser {
            config: ConfigParser::parse(contents),
        }
    }

    /// Get the parsed config
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
