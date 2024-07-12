use anyhow::Result;
use dns_geolocation_checker::{
    configs_parser::ConfigParser,
    ip_geo_checker::{IpGeoChecker, IpGeoCheckerTestedData},
    ip_geo_client::IpGeoProviderType,
};
use std::env;

#[cfg(feature = "ip-api")]
use dns_geolocation_checker::ip_geo_client::ip_api_client::IpApiClient;
#[cfg(feature = "mmdb")]
use dns_geolocation_checker::ip_geo_client::mmdb_client::MMDBClient;

fn print_tested_data(data: Vec<IpGeoCheckerTestedData>) {
    data.clone()
        .into_iter()
        .filter(|r| r.is_ok())
        .for_each(|r| {
            println!(
                "[Matched] {}, ip: {}, subnet: {}, expected: {}, actual: {}",
                r.host, r.ip, r.subnet, r.expected, r.actual
            );
        });

    data.clone()
        .into_iter()
        .filter(|r: &IpGeoCheckerTestedData| r.is_err())
        .for_each(|r| {
            eprintln!(
                "[Mismatched] {}, ip: {}, subnet: {}, expected: {}, actual: {}, error: {:?}",
                r.host,
                r.ip,
                r.subnet,
                r.expected,
                r.actual,
                r.err()
            );
        });
}

#[tokio::main]
async fn main() -> Result<()> {
    let path = env::var("CONFIG_PATH").unwrap_or("./configs/config.toml".to_string());
    let parser = ConfigParser::new_with_path(path);
    let config = parser.config();
    let geo_ip_provider = config.ip_geo_provider.clone();
    let data = match geo_ip_provider {
        #[cfg(feature = "ip-api")]
        IpGeoProviderType::IpApi => {
            IpGeoChecker::<IpApiClient>::new()
                .config(&config)
                .with_ip_api_client()
                .check()
                .await
        }
        #[cfg(feature = "mmdb")]
        IpGeoProviderType::MMDB => {
            IpGeoChecker::<MMDBClient>::new()
                .config(&config)
                .with_mmdb_client()
                .check()
                .await
        }
        _ => panic!("Invalid IP Geo Provider"),
    };

    print_tested_data(data);

    Ok(())
}
