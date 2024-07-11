use anyhow::Result;
use configs_parser::ConfigParser;
use ip_geo_checker::{IpGeoChecker, IpGeoCheckerTestedData};
use std::env;

mod configs_parser;
mod dns_client;
mod ip_geo_checker;

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
            "[Matched] {}, ip: {}, expected: {}, actual: {}",
            r.host, r.ip, r.expected, r.actual
        );
    });

    res.clone()
        .into_iter()
        .filter(|r: &IpGeoCheckerTestedData| r.is_err())
        .for_each(|r| {
            println!(
                "[Mismatched] {}, ip: {}, expected: {}, actual: {}, error: {:?}",
                r.host,
                r.ip,
                r.expected,
                r.actual,
                r.err()
            );
        });

    Ok(())
}
