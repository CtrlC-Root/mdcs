use mdcs_node::node::Config;

fn main() {
    let config = match Config::from_file("config.yaml") {
        Ok(config) => config,
        Err(error) => {
            panic!("failed to parse config file: {}", error);
        }
    };

    println!("{:#?}", config);
}
