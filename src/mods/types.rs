use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{
    interfaces::AddressType,
    providers::{aliyun::Aliyun, dynv6::Dynv6},
};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub log_level: LogLevel,
    // pub plugins: Vec<String>, // temporary disabled
    pub sites_config: Vec<SiteConfig>,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum LogLevel {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "trace")]
    Trace,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Info => "info".to_string(),
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Error => "error".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Trace => "trace".to_string(),
        }
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            log_level: LogLevel::Trace,
            // plugins: Vec::new(),
            sites_config: Vec::new(),
        }
    }

    pub async fn new_from_path<T: AsRef<Path>>(path: &T) -> Result<Config, String> {
        let config = tokio::fs::read_to_string(path).await;
        match config {
            Ok(config) => {
                let config: Config = toml::from_str(&config).unwrap();
                Ok(config)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SiteConfig {
    pub name: String, // unique name
    pub provider: DDNSProvider,
    pub interface: String,
    pub index: usize,
    pub address_version: AddressVersion,
    pub address_type: AddressType,
    // pub plugin: String,
    pub interval: u64,
    pub enabled: bool,
    pub retry_count: u32,
    pub retry_interval: u64,
    pub retry_on_failure: bool,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum AddressVersion {
    V4,
    V6,
}

impl ToString for AddressVersion {
    fn to_string(&self) -> String {
        match self {
            AddressVersion::V4 => "v4".to_string(),
            AddressVersion::V6 => "v6".to_string(),
        }
    }
}

impl SiteConfig {
    pub fn new() -> SiteConfig {
        SiteConfig {
            name: String::new(),
            provider: DDNSProvider::new(),
            interface: String::new(),
            index: 0, // 默认取第一个ip
            address_version: AddressVersion::V4,
            address_type: AddressType::Public,
            // plugin: String::new(),
            interval: 0,
            enabled: false,
            retry_count: 0,
            retry_interval: 60,
            retry_on_failure: true,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum DDNSProvider {
    Aliyun(Aliyun),
    // Namecheap, // TODO
    // Cloudflare, // TODO
    // DuckDNS, // TODO
    // NoIP, // TODO
    // Dynu, // TODO
    // DynDNS, // TODO
    Dynv6(Dynv6),
    // GoDaddy, // TODO
    // GoogleDomains, // TODO
    // HurricaneElectric, // TODO
    // Loopia, // TODO
    // NameSilo, // TODO
    // OVH, // TODO
    // Route53, // TODO
    // Strato, // TODO
    // Yandex, // TODO
    Custom,
}

impl DDNSProvider {
    pub fn new() -> DDNSProvider {
        DDNSProvider::Custom
    }
}
