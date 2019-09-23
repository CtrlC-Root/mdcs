use std::thread;
use std::time::Duration;

use mdcs_node::config::{PluginConfig, NodeConfig};
use mdcs_node::plugin::Plugin;

fn main() {
    let config = match NodeConfig::from_file("config.yaml") {
        Ok(config) => config,
        Err(error) => {
            panic!("failed to parse config file: {}", error);
        }
    };

    println!("{:#?}", config);

    let plugin = Plugin {
        name: String::from("host"),
        config: PluginConfig {
            description: config.plugins["host"].description.clone(),
            command: config.plugins["host"].command.clone()
        }
    };

    let mut instance = plugin.spawn().expect("Failed to spawn plugin instance");
    while instance.is_running() {
        println!(".");
        thread::sleep(Duration::from_millis(1000));
    }

    println!("{}", instance.stop().unwrap());
}
