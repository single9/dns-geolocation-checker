use anyhow::Result;
use configs_parser::ConfigParser;
use ip_geo_checker::IpGeoChecker;
use std::env;

mod configs_parser;
mod dns_client;
mod ip_geo_checker;

#[tokio::main]
async fn main() -> Result<()> {
    let path = env::var("CONFIG_PATH").unwrap_or("./configs/domain.toml".to_string());
    let config = ConfigParser::new(path).parse();
    let _ = IpGeoChecker::new(reqwest::Client::new())
        .config(&config)
        .build()
        .check()
        .await?;

    Ok(())
}
