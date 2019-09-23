use mdcs_node::config::NodeConfig;

fn main() {
    let config = match NodeConfig::from_file("config.yaml") {
        Ok(config) => config,
        Err(error) => {
            panic!("failed to parse config file: {}", error);
        }
    };

    println!("{:#?}", config);
}
