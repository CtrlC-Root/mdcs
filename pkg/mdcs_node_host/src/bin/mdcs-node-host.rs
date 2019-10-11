use mdcs::device::Device;
use mdcs_node::plugin::Server;
use mdcs_node_host::platform_attributes;

fn main() {
    let mut device = Device::new();
    platform_attributes(&mut device);

    let mut server = Server::new(device);
    server.run().expect("Failed to run plugin server");
}
