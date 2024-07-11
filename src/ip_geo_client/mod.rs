use std::{error::Error, net::IpAddr};

use crate::ip_geo_checker::GeoIpResponse;

#[cfg(feature = "mmdb")]
pub mod mmdb_client;
#[cfg(feature = "reqwest")]
pub mod reqwest_client;

pub trait GetGeoIpInfo {
    fn get_geoip_info(
        &self,
        ip: IpAddr,
    ) -> impl std::future::Future<Output = Result<GeoIpResponse, impl Error>> + Send;
    fn batch_get_ip_info(
        &self,
        ips: &Vec<IpAddr>,
    ) -> impl std::future::Future<Output = Result<Vec<GeoIpResponse>, impl Error>> + Send;
}

pub struct IpGeoClient;

impl IpGeoClient {
    pub fn new() -> impl GetGeoIpInfo + Clone {
        #[cfg(not(any(feature = "reqwest", feature = "mmdb")))]
        compile_error!("At least one of the features 'reqwest' or 'mmdb' must be enabled");
        #[cfg(feature = "reqwest")]
        #[cfg(not(feature = "mmdb"))]
        let client = reqwest_client::ReqwestClient::new();
        #[cfg(feature = "mmdb")]
        let client = mmdb_client::MMDBClient::new();

        client
    }
}
