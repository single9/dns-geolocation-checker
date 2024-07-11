#![allow(dead_code)]

use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::sync::Arc;

use hickory_client::client::AsyncClient;
use hickory_proto::rr::RData;
use hickory_proto::xfer::FirstAnswer;
use hickory_proto::DnsHandle;
use hickory_proto::{
    op::{Edns, Message, MessageType, OpCode, Query},
    rr::{rdata::opt::EdnsOption, DNSClass, RecordType},
    udp::UdpClientStream,
};
use hickory_resolver::Name;
use tokio::net::UdpSocket;

pub enum DnsServerAddr {
    Google,
    CloudFlare,
    Custom(SocketAddr),
}

impl DnsServerAddr {
    pub fn addr(&self) -> SocketAddr {
        match *self {
            DnsServerAddr::Google => ("8.8.8.8", 53).to_socket_addrs().unwrap().next().unwrap(),
            DnsServerAddr::CloudFlare => ("1.1.1.1", 53).to_socket_addrs().unwrap().next().unwrap(),
            DnsServerAddr::Custom(addr) => addr,
        }
    }
}

#[derive(Clone)]
pub struct DnsClient {
    client: Arc<AsyncClient>,
}

impl DnsClient {
    pub async fn new(resolver: DnsServerAddr) -> Self {
        let addr = resolver.addr();
        let stream = UdpClientStream::<UdpSocket>::new(addr);
        let client = AsyncClient::connect(stream);
        let (client, bg) = client.await.expect("client failed to connect");

        tokio::spawn(bg);

        Self {
            client: Arc::new(client),
        }
    }

    pub async fn resolve_with_subnet(
        &self,
        domain: &str,
        subnet: &str,
    ) -> anyhow::Result<Vec<IpAddr>> {
        let name = Name::from_ascii(domain)?;
        let mut edns = Edns::new();
        edns.options_mut()
            .insert(EdnsOption::Subnet(subnet.parse()?));

        let mut msg = Message::new();
        msg.add_query({
            let mut query = Query::query(name.clone(), RecordType::A);
            query.set_query_class(DNSClass::IN);
            query
        })
        .set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true)
        .set_edns(edns)
        .extensions_mut()
        .get_or_insert_with(Edns::new)
        .set_max_payload(1232)
        .set_version(0);

        let dns_res = self.client.send(msg).first_answer().await?;
        let result = dns_res
            .answers()
            .into_iter()
            .map(|record| match record.data() {
                Some(&RData::A(ref address)) => IpAddr::from(address.0),
                _ => panic!("Expected A record, got: {:?}", record.data()),
            })
            .collect::<Vec<IpAddr>>();

        Ok(result)
    }
}

#[derive(Clone)]
pub enum DnsResolver {
    Google,
    CloudFlare,
    Custom(SocketAddr),
}

impl DnsResolver {
    pub async fn connect(&self) -> DnsClient {
        match self {
            DnsResolver::Google => DnsClient::new(DnsServerAddr::Google).await,
            DnsResolver::CloudFlare => DnsClient::new(DnsServerAddr::CloudFlare).await,
            DnsResolver::Custom(addr) => DnsClient::new(DnsServerAddr::Custom(*addr)).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, SocketAddrV4};

    #[tokio::test]
    async fn test_dns_server_addr_google() {
        let google = DnsServerAddr::Google.addr();
        assert_eq!(
            google,
            ("8.8.8.8", 53).to_socket_addrs().unwrap().next().unwrap()
        );
    }

    #[tokio::test]
    async fn test_dns_server_addr_cloudflare() {
        let cloudflare = DnsServerAddr::CloudFlare.addr();
        assert_eq!(
            cloudflare,
            ("1.1.1.1", 53).to_socket_addrs().unwrap().next().unwrap()
        );
    }

    #[tokio::test]
    async fn test_dns_server_addr_custom() {
        let custom_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192, 168, 1, 1), 8080));
        let custom = DnsServerAddr::Custom(custom_addr).addr();
        assert_eq!(custom, custom_addr);
    }

    #[tokio::test]
    async fn test_resolve_with_subnet_valid() {
        let resolver = DnsResolver::Google;
        let client = resolver.connect().await;
        let result = client
            .resolve_with_subnet("example.com", "24.24.24.24/24")
            .await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_resolve_with_subnet_invalid_subnet() {
        let resolver = DnsResolver::Google;
        let client = resolver.connect().await;
        let result = client
            .resolve_with_subnet("example.com", "not_a_subnet")
            .await;
        assert!(result.is_err());
    }
}
