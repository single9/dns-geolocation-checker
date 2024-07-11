use std::net::IpAddr;

use crate::ip_geo_checker::GeoIpResponse;

use super::GetGeoIpInfo;

#[derive(Clone)]
pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl GetGeoIpInfo for ReqwestClient {
    #[allow(refining_impl_trait)]
    async fn get_geoip_info(&self, ip: IpAddr) -> Result<GeoIpResponse, reqwest::Error> {
        let ip = ip.to_string();
        let url = format!("http://ip-api.com/json/{}", ip);
        let res = self.client.get(url).send().await?;
        res.json::<GeoIpResponse>().await
    }

    #[allow(refining_impl_trait)]
    async fn batch_get_ip_info(
        &self,
        ips: &Vec<IpAddr>,
    ) -> Result<Vec<GeoIpResponse>, reqwest::Error> {
        let ips = ips.iter().map(|a| a.to_string()).collect::<Vec<String>>();
        let url = "http://ip-api.com/batch";
        self.client
            .post(url)
            .json(&ips)
            .send()
            .await?
            .json::<Vec<GeoIpResponse>>()
            .await
    }
}
