use std::{error::Error, net::IpAddr};

use serde::Deserialize;

use crate::{configs_parser::Config, ip_geo_checker::GeoIpResponse};

#[cfg(feature = "ip-api")]
pub mod ip_api_client;
#[cfg(feature = "mmdb")]
pub mod mmdb_client;

pub trait NewProvider {
    /// Create a new instance of the provider
    fn new(config: &Config) -> Self;

    /// Get the provider type
    fn get_provider_type(&self) -> String;
}

pub trait GetGeoIpInfo {
    /// Get the geoip info for an IP
    fn get_geoip_info(
        &self,
        ip: IpAddr,
    ) -> impl std::future::Future<Output = Result<GeoIpResponse, impl Error>> + Send;

    /// Get the geoip info for a batch of IPs
    fn batch_get_ip_info(
        &self,
        ips: &Vec<IpAddr>,
    ) -> impl std::future::Future<Output = Result<Vec<GeoIpResponse>, impl Error>> + Send;
}

/// The type of IP geo provider
///
/// This is an enum that holds the different types of IP geo providers
///
/// # Variants
///
/// * `IpApi` - The IP API provider
/// * `MMDB` - The MMDB provider
/// * `None` - No provider
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

impl std::fmt::Display for IpGeoProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "ip-api")]
            Self::IpApi => write!(f, "IP-API"),
            #[cfg(feature = "mmdb")]
            Self::MMDB => write!(f, "MMDB"),
            Self::None => write!(f, "None"),
        }
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

    #[allow(dead_code)]
    pub fn get_provider_type(&self) -> String {
        self.0.get_provider_type()
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
    /// Create a new instance of the IP geo client with the given provider
    ///
    /// # Examples
    ///
    /// ```
    /// use dns_geolocation_checker::ip_geo_client::{IpGeoClient, IpGeoProviderType};
    /// use dns_geolocation_checker::ip_geo_client::mmdb_client::MMDBClient;
    /// use dns_geolocation_checker::ip_geo_client::ip_api_client::IpApiClient;
    /// use dns_geolocation_checker::configs_parser::Config;
    ///
    /// let config = Config::default();
    /// let mmdb_client = IpGeoClient::with_provider::<MMDBClient>(&config);
    /// let ipapi_client = IpGeoClient::with_provider::<IpApiClient>(&config);
    ///
    /// assert_eq!(mmdb_client.get_provider_type(), IpGeoProviderType::MMDB.to_string());
    /// assert_eq!(ipapi_client.get_provider_type(), IpGeoProviderType::IpApi.to_string());
    /// ```
    pub fn with_provider<T>(config: &Config) -> IpGeoProvider<T>
    where
        T: GetGeoIpInfo + NewProvider + Clone,
    {
        IpGeoProvider(T::new(config))
    }
}
