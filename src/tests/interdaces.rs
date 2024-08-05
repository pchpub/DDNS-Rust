use network_interface::Netmask;
use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use network_interface::V6IfAddr;

use crate::mods::interfaces::AddressType;

#[tokio::test]
async fn test_interfaces() {
    let network_interfaces = NetworkInterface::show().unwrap();

    for itf in network_interfaces.iter() {
        println!("\n{:?}", itf);
    }

    let test_ipv6_private = "fe80::a00:27ff:fe4e:66c0";
    let test_ipv6_private = network_interface::Addr::V6(V6IfAddr {
        ip: test_ipv6_private.parse().unwrap(),
        broadcast: None,
        netmask: None,
    });

    let test_ipv6_private_type = AddressType::declare_address_type(&test_ipv6_private);
    println!("test_ipv6_private_type: {:?}", test_ipv6_private_type);
    assert_eq!(test_ipv6_private_type, AddressType::LinkLocal);
}
