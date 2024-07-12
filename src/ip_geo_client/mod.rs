use std::{error::Error, net::IpAddr};

use serde::Deserialize;

use crate::{configs_parser::Config, ip_geo_checker::GeoIpResponse};

#[cfg(feature = "ip-api")]
pub mod ip_api_client;
#[cfg(feature = "mmdb")]
pub mod mmdb_client;

pub trait NewProvider {
    fn new(config: &Config) -> Self;
}

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

#[derive(Clone, Debug, Deserialize)]
pub enum IpGeoProviderType {
    #[cfg(feature = "ip-api")]
    #[serde(alias = "ip-api")]
    IpApi,
    #[cfg(feature = "mmdb")]
    #[serde(alias = "mmdb")]
    MMDB,
    None,
}

impl Default for IpGeoProviderType {
    fn default() -> Self {
        Self::MMDB
    }
}

#[derive(Clone, Default, Debug)]
pub struct IpGeoProvider<T>(pub T);

impl<T> IpGeoProvider<T>
where
    T: GetGeoIpInfo + NewProvider + Clone,
{
    pub fn new(provider: T) -> Self {
        Self(provider)
    }
}

impl<T: GetGeoIpInfo> GetGeoIpInfo for IpGeoProvider<T> {
    fn get_geoip_info(
        &self,
        ip: IpAddr,
    ) -> impl std::future::Future<Output = Result<GeoIpResponse, impl Error>> + Send {
        self.0.get_geoip_info(ip)
    }

    fn batch_get_ip_info(
        &self,
        ips: &Vec<IpAddr>,
    ) -> impl std::future::Future<Output = Result<Vec<GeoIpResponse>, impl Error>> + Send {
        self.0.batch_get_ip_info(ips)
    }
}

#[derive(Default, Clone)]
pub struct IpGeoClient;

impl IpGeoClient {
    pub fn with_provider<T>(config: &Config) -> IpGeoProvider<T>
    where
        T: GetGeoIpInfo + NewProvider + Clone,
    {
        IpGeoProvider(T::new(config))
    }
}
