use mdcs::device::Device;
use mdcs_node::plugin::PluginServer;

use sensors;

fn main() {
    let device = Device::new();
    println!("{:#?}", device);

    let server = PluginServer::new(device);
    server.run();
}
