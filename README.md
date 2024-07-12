# DNS Geolocation Checker

This project is a Rust application designed to check the geolocation of DNS addresses. It utilizes various Rust crates to parse configuration files, perform DNS queries, and check IP geolocations.

## Getting Started

To get started with this project, clone the repository and ensure you have Rust and Cargo installed on your system.

### Installation

#### Cargo

Binary releases are available on [crates.io](https://crates.io/crates/dns-geolocation-checker). You can install the DNS Geolocation Checker using the following command:

```sh
cargo install dns-geolocation-checker
```

#### Manually

1. Clone the repository:

```sh
git clone https://github.com/single9/dns-geolocation-checker.git
```

2. Navigate to the project directory:

```sh
cd dns-geolocation-checker
```

3. Build the project:

```sh
cargo build
```

## Usage

### Feature Flags

The DNS Geolocation Checker supports the following feature flags:

- `ip-api`: Enables the IP Geolocation API provider.
- `mmdb`: Enables the MaxMind GeoLite2 database provider.

To enable a feature flag, use the following command:

```sh
cargo build -F ip-api
```

To enable multiple feature flags, use the following command:

```sh
cargo build -F full
```

### Configuration

You can configure the DNS Geolocation Checker by modifying the `config.toml` file. The configuration file contains the following sections:

```toml
[test_subnets]
sg = { subnets = ["175.41.192.0/18"] }
us = { subnets = ["44.208.193.0/24"] }

[[domain]]
host = "www.example.com"
geo_routing = ["sg", "us"]
```

- `test_subnets`: Defines a section for testing subnets with geographical identifiers.
  - `sg`: A key representing Singapore, containing a list with a single subnet `"175.41.192.0/18"`.
  - `us`: A key representing the United States, containing a list with a single subnet `"44.208.193.0/24"`.

- `[[domain]]`: An array of domain configurations, allowing for multiple entries.
  - `host`: Specifies the domain name, here it is `"www.example.com"`.
  - `geo_routing`: An array indicating which geographical subnet groups (`sg` and `us`) this domain is associated with for geo-routing purposes.

Put the file `config.toml` in the `configs` directory of the project. Or you can specify the path to the configuration file using the `CONFIG_PATH` environment variable when running the application.

### IP Geolocation Providers

#### MMDB

This is the default IP geolocation provider.

If you want to use the MaxMind GeoLite2 database, you need to download the database from the [MaxMind website](https://dev.maxmind.com/geoip/geoip2/geolite2/). After downloading the database, you need to specify the path to the database in the `config.toml` file:

```toml
ip_geo_provider = "mmdb"

# Set the path to the MMDB file
# Default: "./mmdb/GeoLite2-City.mmdb"
mmdb_path = "/path/to/GeoLite2-City.mmdb"
```

The default path is `./mmdb/GeoLite2-City.mmdb`.

#### IP-API

The [IP Geolocation API](https://ip-api.com/) is a free service that provides geolocation information for IP addresses.

If you want to use the IP Geolocation API service, you need to specify the provider in the `config.toml` file:

```toml
ip_geo_provider = "ip-api"
```

### Run

To run the DNS Geolocation Checker, use the following command:

```sh
cargo run --bin dns-geo-checker
```

When you run the DNS Geolocation Checker, it will query the DNS records for each domain and check the geolocation of the IP addresses returned. If the IP address falls within one of the subnets specified in the `test_subnets` section, the geolocation will be considered a match.

### Build

To build the project, use the following command:

```sh
cargo build
```

You can find the compiled binary in the `target/release` directory.

## Running the tests

To run the tests for this project, execute:

```sh
cargo test --verbose
```

## TODO

- [ ] CLI mode
- [X] Support multiple IP geolocation providers
- [ ] Support IPv6 addresses
- [ ] Map IP addresses to geographical locations

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
