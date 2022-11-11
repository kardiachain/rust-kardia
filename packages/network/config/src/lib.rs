use std::{fs, path::PathBuf};

pub mod config;

/// Read a fixture file from the `support/config` directory
pub fn read_fixture(name: &str) -> String {
    fs::read_to_string(PathBuf::from("./tests/support/config/").join(name)).unwrap()
}

#[test]
fn test_parse_yaml() {
    use config::Config;
    let file_string_type = read_fixture("devnet_node2.yaml");
    
    let config : Config = serde_yaml::from_str(&file_string_type).unwrap();
    
    println!("{:?}", config);
}
