use log::error;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use serde::{Deserialize, Serialize};

pub async fn get_interfaces() -> Result<Vec<NetworkInterface>, ()> {
    let network_interfaces: Vec<NetworkInterface> = match NetworkInterface::show() {
        Ok(network_interfaces) => network_interfaces,
        Err(e) => {
            error!("Error getting interfaces: {}", e);
            return Err(());
        }
    };

    // Remove same interfaces addresses
    let network_interfaces: Vec<NetworkInterface> =
        network_interfaces
            .into_iter()
            .fold(Vec::new(), |mut acc, mut itf| {
                let mut found = false;
                for acc_itf in acc.iter_mut() {
                    if acc_itf.name == itf.name {
                        acc_itf.addr.append(&mut itf.addr);
                        found = true;
                        break;
                    }
                }
                if !found {
                    acc.push(itf);
                }
                acc
            });

    Ok(network_interfaces)
}

pub async fn get_interface_ips(
    interfaces: &Vec<NetworkInterface>,
    interface: &str,
) -> Vec<IPAddress> {
    let mut ips = Vec::new();
    for itf in interfaces.iter() {
        if itf.name == interface {
            for addr in itf.addr.iter() {
                // match addr {
                //     network_interface::Addr::V4(_address) => ips.push(IPAddress::from(addr)),
                //     network_interface::Addr::V6(_address) => ips.push(IPAddress::from(addr)),
                // }
                ips.push(IPAddress::from(addr))
            }
        }
    }
    ips
}

#[derive(Clone)]

pub enum IPAddress {
    V4(String, AddressType),
    V6(String, AddressType),
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum AddressType {
    Public,
    Private,
    Loopback,
    Multicast,
    Unspecified,
    Broadcast,
    LinkLocal,
    Other,
}

impl AddressType {
    pub fn declare_address_type(addr: &network_interface::Addr) -> Self {
        match addr {
            network_interface::Addr::V4(address) => {
                if address.ip.is_loopback() {
                    AddressType::Loopback
                } else if address.ip.is_multicast() {
                    AddressType::Multicast
                } else if address.ip.is_unspecified() {
                    AddressType::Unspecified
                } else if address.ip.is_private() {
                    AddressType::Private
                } else if address.ip.is_broadcast() {
                    AddressType::Broadcast
                } else if address.ip.is_link_local() {
                    AddressType::LinkLocal
                } else {
                    AddressType::Public
                }
            }
            network_interface::Addr::V6(address) => {
                if address.ip.is_loopback() {
                    AddressType::Loopback
                } else if address.ip.is_multicast() {
                    AddressType::Multicast
                } else if address.ip.is_unspecified() {
                    AddressType::Unspecified
                } else if address.ip.octets()[0] == 0b11111100
                    || address.ip.octets()[0] == 0b11111101
                {
                    AddressType::Private // fc00::/7
                } else if address.ip.octets()[0] >= 0b00100000
                    || address.ip.octets()[0] <= 0b00111111
                {
                    AddressType::Public // 2000::/3
                } else {
                    AddressType::Other
                }
            }
        }
    }
}

impl From<&network_interface::Addr> for IPAddress {
    fn from(ip: &network_interface::Addr) -> Self {
        match ip {
            network_interface::Addr::V4(v4_ip) => {
                IPAddress::V4(v4_ip.ip.to_string(), AddressType::declare_address_type(&ip))
            }
            network_interface::Addr::V6(v6_ip) => {
                IPAddress::V6(v6_ip.ip.to_string(), AddressType::declare_address_type(&ip))
            }
        }
    }
}
