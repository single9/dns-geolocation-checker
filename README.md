# dns-geolocation-checker

## Settings

```toml
[test_subnets]
sg = { subnets = ["175.41.192.0/18"] }
us = { subnets = ["44.208.193.0/24"] }

[[domain]]
host = "www.example.com"
geo_routing = ["sg", "us"]
```
