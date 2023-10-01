use std::fs;
use crate::nodes;

pub fn get_decision_tree() -> nodes::Node {
    let config_data =
        fs::read_to_string("decision_tree.json").expect("Failed to read the config file");

    serde_json::from_str(&config_data).expect("Failed to parse config data")
}
