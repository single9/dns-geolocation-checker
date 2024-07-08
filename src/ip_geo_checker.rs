#![allow(dead_code)]

use crate::configs_parser::{Config, DomainConfig};
use crate::dns_client::DnsResolver;
use serde::Deserialize;

const IP_API_BATCH: &'static str = "http://ip-api.com/batch";

#[derive(Debug, Clone, Deserialize)]
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

    pub async fn check(&self) -> anyhow::Result<bool> {
        let resolver = self.dns_resolver.connect().await;
        let test_subnets = self.config.test_subnets.clone();
        let domains = self
            .config
            .domain
            .iter()
            .map(|d| d.clone())
            .collect::<Vec<DomainConfig>>();

        let mut tasks = vec![];
        for domain in domains.iter() {
            domain.geo_routing.iter().for_each(|geo| {
                tasks.push(async {
                    let ips = resolver
                        .resolve_with_subnet(
                            domain.host.as_str(),
                            test_subnets.get(&geo.to_string()).unwrap().subnets[0].as_str(),
                        )
                        .await
                        .unwrap();

                    let ip_info = self.batch_get_ip_info(&ips).await.unwrap();
                    let geo_not_eq = ip_info.iter().any(|ip_info| {
                        ip_info.country_code.to_ascii_lowercase() != geo.to_ascii_lowercase()
                    });

                    if geo_not_eq {
                        Err(domain.clone())
                    } else {
                        Ok(())
                    }
                });
            });
        }

        for task in tasks {
            let ip_info = task.await;
            if let Err(domain) = ip_info {
                eprintln!("Country mismatch for domain: {}", domain.host);
                return Ok(false);
            }
        }

        Ok(true)
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
