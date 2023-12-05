use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;

#[tokio::test]
async fn test_interfaces() {
    let network_interfaces = NetworkInterface::show().unwrap();

    for itf in network_interfaces.iter() {
        println!("{:?}", itf);
    }
}
