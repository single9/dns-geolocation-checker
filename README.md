# DNS Geolocation Checker

This project is a Rust application designed to check the geolocation of DNS addresses. It utilizes various Rust crates to parse configuration files, perform DNS queries, and check IP geolocations.

## Getting Started

To get started with this project, clone the repository and ensure you have Rust and Cargo installed on your system.

### Installation

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

### Run

To run the DNS Geolocation Checker, use the following command:

```sh
cargo run
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
- [ ] Support IPv6 addresses
