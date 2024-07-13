use std::{net::IpAddr, sync::Arc};

use crate::{configs_parser::Config, ip_geo_checker::GeoIpResponse};

use super::{GetGeoIpInfo, IpGeoProviderType, NewProvider};

#[derive(Clone)]
pub struct IpApiClient {
    api_base: String,
    client: Arc<reqwest::Client>,
}

impl NewProvider for IpApiClient {
    fn new(_: &Config) -> Self {
        Self {
            api_base: "http://ip-api.com".to_string(),
            client: Arc::new(reqwest::Client::new()),
        }
    }

    fn get_provider_type(&self) -> String {
        IpGeoProviderType::IpApi.to_string()
    }
}

impl GetGeoIpInfo for IpApiClient {
    #[allow(refining_impl_trait)]
    async fn get_geoip_info(&self, ip: IpAddr) -> Result<GeoIpResponse, reqwest::Error> {
        let ip = ip.to_string();
        let url = format!("{}/json/{}", self.api_base, ip);
        let res = self.client.get(url).send().await?;
        res.json::<GeoIpResponse>().await
    }

    #[allow(refining_impl_trait)]
    async fn batch_get_ip_info(
        &self,
        ips: &Vec<IpAddr>,
    ) -> Result<Vec<GeoIpResponse>, reqwest::Error> {
        let ips = ips.iter().map(|a| a.to_string()).collect::<Vec<String>>();
        let url = format!("{}/batch", self.api_base);
        self.client
            .post(url)
            .json(&ips)
            .send()
            .await?
            .json::<Vec<GeoIpResponse>>()
            .await
    }
}
