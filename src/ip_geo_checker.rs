#![allow(dead_code)]

use serde::Deserialize;
use std::net::IpAddr;
use Result;

use crate::configs_parser::{Config, DomainConfig};
use crate::dns_client::DnsResolver;

const IP_API_BATCH: &'static str = "http://ip-api.com/batch";

/// A struct to hold the response from the ip-api.com API
#[derive(Default, Debug, Clone, Deserialize)]
pub struct IpApiResponse {
    pub query: String,
    pub status: String,
    pub country: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    pub region: String,
    #[serde(rename = "regionName")]
    pub region_name: String,
    pub city: String,
    pub zip: String,
    pub lat: f64,
    pub lon: f64,
}

/// A struct to hold the tested data
#[derive(Debug, Clone)]
pub struct IpGeoCheckerTestedData {
    /// The host of the domain
    pub host: String,
    /// The IP address
    pub ip: IpAddr,
    /// The response from the ip-api.com API
    pub geoip: IpApiResponse,
    /// The subnet
    pub subnet: String,
    /// The expected country code
    pub expected: String,
    /// The actual country code
    pub actual: String,
    /// The error message
    pub error: Option<String>,
}

impl Default for IpGeoCheckerTestedData {
    fn default() -> Self {
        Self {
            host: "".to_string(),
            ip: "0.0.0.0".parse().unwrap(),
            geoip: IpApiResponse::default(),
            subnet: "".to_string(),
            expected: "".to_string(),
            actual: "".to_string(),
            error: None,
        }
    }
}

impl IpGeoCheckerTestedData {
    pub fn set_host(&mut self, host: &str) -> &mut Self {
        self.host = host.to_string();
        self
    }

    pub fn set_ip(&mut self, ip: IpAddr) -> &mut Self {
        self.ip = ip;
        self
    }

    pub fn set_geoip(&mut self, geoip: IpApiResponse) -> &mut Self {
        self.geoip = geoip;
        self
    }

    pub fn set_subnet<T: ToString>(&mut self, subnet: T) -> &mut Self {
        self.subnet = subnet.to_string();
        self
    }

    pub fn set_expected(&mut self, expected: &str) -> &mut Self {
        self.expected = expected.to_string().to_ascii_lowercase();
        self
    }

    pub fn set_actual(&mut self, actual: &str) -> &mut Self {
        self.actual = actual.to_string().to_ascii_lowercase();
        self
    }

    /// Check if the expected country code matches the actual country code
    pub fn test(&self) -> Self {
        if self.expected == self.actual {
            self.clone()
        } else {
            let mut err_res = self.clone();
            err_res.error = Some(format!(
                "Expected: {}, Actual: {}",
                self.expected, self.actual
            ));
            err_res.clone()
        }
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn is_ok(&self) -> bool {
        self.error.is_none()
    }

    pub fn err(&self) -> Option<String> {
        self.error.clone()
    }
}

#[derive(Default, Clone, Debug)]
pub struct IpGeoCheckerResult {
    pub domain: DomainConfig,
    pub geoip: Vec<IpApiResponse>,
    pub expected: String,
    pub actual: bool,
}

pub struct IpGeoCheckerBuilder {
    client: reqwest::Client,
    dns_resolver: DnsResolver,
    config: Config,
}

impl IpGeoCheckerBuilder {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            client,
            dns_resolver: DnsResolver::Google,
            config: Config::default(),
        }
    }

    pub fn config(&mut self, config: &Config) -> &mut Self {
        self.config = config.clone();
        self
    }

    pub fn dns_resolver(&mut self, dns_resolver: DnsResolver) -> &mut Self {
        self.dns_resolver = dns_resolver;
        self
    }

    pub fn build(&mut self) -> IpGeoChecker {
        IpGeoChecker {
            client: self.client.clone(),
            dns_resolver: self.dns_resolver.clone(),
            config: self.config.clone(),
        }
    }
}

#[derive(Clone)]
pub struct IpGeoChecker {
    client: reqwest::Client,
    dns_resolver: DnsResolver,
    config: Config,
}

impl IpGeoChecker {
    pub fn new(client: reqwest::Client) -> IpGeoCheckerBuilder {
        IpGeoCheckerBuilder::new(client)
    }

    pub async fn check(&self) -> Vec<IpGeoCheckerTestedData> {
        let resolver = self.dns_resolver.connect().await;
        let test_subnets = self.config.test_subnets.clone();
        let domains = self
            .config
            .domain
            .iter()
            .map(|d| d.clone())
            .collect::<Vec<DomainConfig>>();

        let mut tasks = vec![];
        for domain in domains.into_iter() {
            domain.geo_routing.into_iter().for_each(|geo| {
                let subnets = test_subnets.get(&geo.to_string()).unwrap().subnets.clone();
                subnets.into_iter().for_each(|subnet| {
                    let host = domain.host.clone();
                    let c_geo = geo.clone();
                    let c_resolver = resolver.clone();
                    let c_subnet = subnet.clone();

                    tasks.push(async move {
                        let ips = c_resolver
                            .resolve_with_subnet(&host.to_string(), &c_subnet.clone())
                            .await
                            .unwrap();

                        let geoip_results = self
                            .batch_get_ip_info(&ips)
                            .await
                            .unwrap()
                            .iter()
                            .map(|ip| {
                                IpGeoCheckerTestedData::default()
                                    .set_host(&host.to_string())
                                    .set_ip(ip.query.parse().unwrap())
                                    .set_geoip(ip.clone())
                                    .set_subnet(&c_subnet.clone())
                                    .set_expected(c_geo.as_str())
                                    .set_actual(ip.country_code.as_str())
                                    .test()
                            })
                            .collect::<Vec<IpGeoCheckerTestedData>>();

                        geoip_results
                    });
                });
            });
        }

        futures::future::join_all(tasks)
            .await
            .into_iter()
            .flatten()
            .collect()
    }

    async fn batch_get_ip_info(
        &self,
        ips: &Vec<std::net::IpAddr>,
    ) -> Result<Vec<IpApiResponse>, reqwest::Error> {
        let ips = ips.iter().map(|a| a.to_string()).collect::<Vec<String>>();
        self.client
            .post(IP_API_BATCH)
            .json(&ips)
            .send()
            .await?
            .json::<Vec<IpApiResponse>>()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn test_ip_geo_checker_tested_data_set_host() {
        let mut data = IpGeoCheckerTestedData::default();
        data.set_host("example.com");
        assert_eq!(data.host, String::from("example.com"));
    }

    #[test]
    fn test_ip_geo_checker_tested_data_set_ip() {
        let mut data = IpGeoCheckerTestedData::default();
        let ip: IpAddr = "192.0.2.1".parse().unwrap();
        data.set_ip(ip);
        assert_eq!(data.ip, ip);
    }

    #[test]
    fn test_ip_geo_checker_tested_data_set_geoip() {
        let mut data = IpGeoCheckerTestedData::default();
        let geoip = IpApiResponse::default();
        data.set_geoip(geoip.clone());
        assert!(data.geoip.query == geoip.query);
    }

    #[test]
    fn test_ip_geo_checker_tested_data_set_subnet() {
        let mut data = IpGeoCheckerTestedData::default();
        data.set_subnet("");
        assert_eq!(data.subnet, String::from(""));
    }

    #[test]
    fn test_ip_geo_checker_tested_data_set_expected() {
        let mut data = IpGeoCheckerTestedData::default();
        data.set_expected("US");
        assert_eq!(data.expected, String::from("us"));
    }

    #[test]
    fn test_ip_geo_checker_tested_data_set_actual() {
        let mut data = IpGeoCheckerTestedData::default();
        data.set_actual("US");
        assert_eq!(data.actual, String::from("us"));
    }

    #[test]
    fn test_ip_geo_checker_tested_data_test_success() {
        let mut data = IpGeoCheckerTestedData::default();
        data.set_expected("US").set_actual("US");
        assert!(data.test().is_ok());
    }

    #[test]
    fn test_ip_geo_checker_tested_data_test_failure() {
        let mut data = IpGeoCheckerTestedData::default();
        data.set_expected("US").set_actual("CA");
        assert!(data.test().is_err());
    }
}
