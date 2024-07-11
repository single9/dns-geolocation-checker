use anyhow::Result;
use dns_geolocation_checker::{configs_parser::ConfigParser, ip_geo_checker::{IpGeoChecker, IpGeoCheckerTestedData}};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let path = env::var("CONFIG_PATH").unwrap_or("./configs/config.toml".to_string());
    let parser = ConfigParser::new_with_path(path);
    let config = parser.config();
    let res = IpGeoChecker::new(reqwest::Client::new())
        .config(&config)
        .build()
        .check()
        .await;

    res.clone().into_iter().filter(|r| r.is_ok()).for_each(|r| {
        println!(
            "[Matched] {}, ip: {}, subnet: {}, expected: {}, actual: {}",
            r.host, r.ip, r.subnet, r.expected, r.actual
        );
    });

    res.clone()
        .into_iter()
        .filter(|r: &IpGeoCheckerTestedData| r.is_err())
        .for_each(|r| {
            println!(
                "[Mismatched] {}, ip: {}, subnet: {}, expected: {}, actual: {}, error: {:?}",
                r.host,
                r.ip,
                r.subnet,
                r.expected,
                r.actual,
                r.err()
            );
        });

    Ok(())
}
