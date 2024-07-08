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
pub struct ConfigParser {
    path: String,
}

impl ConfigParser {
    pub fn new<T: ToString>(path: T) -> ConfigParser {
        ConfigParser {
            path: path.to_string(),
        }
    }

    pub fn parse(&self) -> Config {
        let contents =
            fs::read_to_string(&self.path).expect("Should have been able to read the file");

        toml::from_str(&contents).unwrap()
    }
}
