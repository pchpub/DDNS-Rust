// cSpell:ignore ddns, chrono, Aliyun, Dynv6
use crate::mods::{
    interfaces::AddressType,
    providers::{aliyun::Aliyun, dynv6::Dynv6},
    types::{Config, DDNSProvider, SiteConfig},
};

#[tokio::test]
async fn config() {
    println!("config test");
    let mut config = Config::new();
    config.sites_config.push(SiteConfig::new());
    config.sites_config.push(SiteConfig {
        name: "452977c1-aef1-4b16-8299-5b840b4e31ed".to_string(),
        provider: DDNSProvider::Aliyun(Aliyun::new(
            "your_access_key_id",
            "your_access_key_secret",
            "example.com",
            "www",
            Some(600),
            "A",
        )),
        interface: "ens33".to_string(),
        index: 1,
        address_version: crate::mods::types::AddressVersion::V4,
        address_type: AddressType::Private,
        interval: 600,
        enabled: true,
        retry_count: 0,
        retry_interval: 60,
        retry_on_failure: true,
    });
    config.sites_config.push(SiteConfig {
        name: "6e771d2e-8d06-25ff-1c84-1709a4ad8cb6".to_string(),
        provider: DDNSProvider::Dynv6(Dynv6::new(11451, "YOUR_TOKEN", "www", "A")),
        interface: "ens34".to_string(),
        index: 1,
        address_version: crate::mods::types::AddressVersion::V4,
        address_type: AddressType::Public,
        interval: 600,
        enabled: true,
        retry_count: 0,
        retry_interval: 60,
        retry_on_failure: true,
    });
    config.sites_config.push(SiteConfig::new());

    // save to test.toml file
    let toml = toml::to_string(&config).unwrap();
    println!("toml: {}", toml);
}
