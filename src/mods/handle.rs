use std::time::Duration;

use log::{error, trace};
use tokio::time::sleep;

use crate::mods::{providers::types::ProvidersErrorType, types::AddressVersion};

use super::{
    interfaces::{get_interface_ips, get_interfaces},
    providers::DDNSProviderTrait,
    statics::CONFIG,
};

pub async fn spawn_tasks() -> Result<(), String> {
    let config = CONFIG.lock().await.clone();
    let sites_config = config.sites_config;
    for single_site in sites_config {
        tokio::spawn(async move {
            let mut interval_duration = Duration::from_secs(0);
            let site = single_site.clone();
            let mut failures: u32 = 0;
            loop {
                sleep(interval_duration).await;

                let interfaces = match get_interfaces().await {
                    Ok(interfaces) => interfaces,
                    Err(_e) => {
                        if site.retry_on_failure {
                            if site.retry_count == 0 {
                                interval_duration = Duration::from_secs(site.retry_interval);
                                continue;
                            } else if failures < site.retry_count {
                                failures += 1;
                                interval_duration = Duration::from_secs(site.retry_interval);
                                continue;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                };

                trace!("interfaces: {:?}", interfaces);

                let ips = get_interface_ips(&interfaces, &site.interface)
                    .await
                    .into_iter()
                    .filter(|ip| match ip {
                        super::interfaces::IPAddress::V4(_ip, address_type) => {
                            if site.address_version == AddressVersion::V4
                                && *address_type == site.address_type
                            {
                                true
                            } else {
                                false
                            }
                        }
                        super::interfaces::IPAddress::V6(_ip, address_type) => {
                            if site.address_version == AddressVersion::V6
                                && *address_type == site.address_type
                            {
                                true
                            } else {
                                false
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                trace!("ips: {:?}", ips);
                if ips.len() < site.index + 1 {
                    if site.retry_on_failure {
                        trace!("retrying");
                        if site.retry_count == 0 {
                            interval_duration = Duration::from_secs(site.retry_interval);
                            continue;
                        } else if failures < site.retry_count {
                            failures += 1;
                            interval_duration = Duration::from_secs(site.retry_interval);
                            continue;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                let needed_ip = match ips[site.index].clone() {
                    super::interfaces::IPAddress::V4(ip, _address_type) => ip,
                    super::interfaces::IPAddress::V6(ip, _address_type) => ip,
                };
                let mut provier: Box<dyn DDNSProviderTrait> =
                    Box::new(match site.provider.clone() {
                        super::types::DDNSProvider::Aliyun(value) => value,
                        super::types::DDNSProvider::Custom => {
                            error!("Custom provider not implemented");
                            break;
                        }
                    });
                let cloud_ip = match provier.get_ip_address().await {
                    Ok(cloud_ip) => cloud_ip,
                    Err(e) => match e {
                        ProvidersErrorType::NoRecordFound => {
                            trace!("No record found, should create new record");
                            String::new()
                        }
                        _ => {
                            error!("Failed to get cloud IP address: {}", e);
                            if site.retry_on_failure {
                                if site.retry_count == 0 {
                                    interval_duration = Duration::from_secs(site.retry_interval);
                                    continue;
                                } else if failures < site.retry_count {
                                    failures += 1;
                                    interval_duration = Duration::from_secs(site.retry_interval);
                                    continue;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    },
                };
                if cloud_ip == needed_ip {
                    interval_duration = Duration::from_secs(site.interval);
                    failures = 0;
                    continue;
                } else {
                    match provier.update(&needed_ip).await {
                        Ok(_) => {
                            failures = 0;
                        }
                        Err(e) => {
                            error!("Failed to update IP address: {}", e);

                            if site.retry_on_failure {
                                if site.retry_count == 0 {
                                    interval_duration = Duration::from_secs(site.retry_interval);
                                    continue;
                                } else if failures < site.retry_count {
                                    failures += 1;
                                    interval_duration = Duration::from_secs(site.retry_interval);
                                    continue;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                interval_duration = Duration::from_secs(site.interval);
            }
            error!("Exit task: {}", site.name);
        });
    }
    Ok(())
}
