use std::{env, net::IpAddr, sync::Arc};

use crate::ip_geo_checker::GeoIpResponse;

use super::{GetGeoIpInfo, NewProvider};

#[derive(Clone)]
pub struct MMDBClient {
    reader: Arc<maxminddb::Reader<Vec<u8>>>,
}

impl NewProvider for MMDBClient {
    fn new() -> Self {
        let mmdb_path = env::var("MMDB_PATH").unwrap_or("./mmdb/GeoLite2-City.mmdb".to_string());
        let reader = maxminddb::Reader::open_readfile(mmdb_path).unwrap();
        Self {
            reader: Arc::new(reader),
        }
    }
}

impl GetGeoIpInfo for MMDBClient {
    #[allow(refining_impl_trait)]
    async fn get_geoip_info(&self, ip: IpAddr) -> Result<GeoIpResponse, maxminddb::MaxMindDBError> {
        let ip = ip.to_string();
        let record: maxminddb::geoip2::City = self.reader.lookup(ip.parse().unwrap())?;
        Ok(GeoIpResponse {
            query: ip,
            country: record
                .country
                .clone()
                .unwrap()
                .iso_code
                .unwrap()
                .to_string(),
            country_code: record
                .country
                .clone()
                .unwrap()
                .iso_code
                .unwrap()
                .to_string(),
            region: record
                .subdivisions
                .clone()
                .unwrap()
                .get(0)
                .unwrap()
                .iso_code
                .unwrap()
                .to_string(),
            region_name: "".to_string(),
            city: record
                .city
                .unwrap()
                .names
                .unwrap()
                .get("en")
                .unwrap()
                .to_string(),
            lat: record.location.clone().unwrap().latitude.unwrap(),
            lon: record.location.clone().unwrap().longitude.unwrap(),
        })
    }

    #[allow(refining_impl_trait)]
    async fn batch_get_ip_info(
        &self,
        ips: &Vec<IpAddr>,
    ) -> Result<Vec<GeoIpResponse>, maxminddb::MaxMindDBError> {
        let ips = ips.iter().map(|a| a.to_string()).collect::<Vec<String>>();
        let mut results = vec![];
        for ip in ips.iter() {
            let record: maxminddb::geoip2::City = self.reader.lookup(ip.parse().unwrap())?;
            results.push(GeoIpResponse {
                query: ip.to_string(),
                country: record
                    .country
                    .clone()
                    .unwrap()
                    .iso_code
                    .unwrap()
                    .to_string(),
                country_code: record
                    .country
                    .clone()
                    .unwrap()
                    .iso_code
                    .unwrap()
                    .to_string(),
                region: "".to_string(),
                region_name: "".to_string(),
                city: record
                    .city
                    .unwrap()
                    .names
                    .unwrap()
                    .get("en")
                    .unwrap()
                    .to_string(),
                lat: record.location.clone().unwrap().latitude.unwrap(),
                lon: record.location.clone().unwrap().longitude.unwrap(),
            });
        }
        Ok(results)
    }
}
