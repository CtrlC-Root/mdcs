use mdcs_node::plugin::Server;
use mdcs_node_host::make_device;

fn main() {
    let device = make_device();
    let device_name = "host".to_string(); // TODO: read local hostname
    let mut server = Server::new(device_name, device);

    server.run().unwrap();
}
