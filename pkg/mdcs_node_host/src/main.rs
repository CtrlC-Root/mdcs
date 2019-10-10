use mdcs::device::Device;
use mdcs_node::plugin::Server;
use mdcs_node_host::platform_attributes;

fn main() {
    let mut device = Device::new();
    let device_name = "host".to_string(); // TODO: read local hostname

    platform_attributes(&mut device);

    let mut server = Server::new(device_name, device);
    server.run().unwrap();
}
