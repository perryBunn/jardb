use std::env;
use std::fs;
use yaml_rust as yaml;

pub fn get_config(config_file: &str) -> Vec<yaml::Yaml> {
    let contents = fs::read_to_string(config_file)
        .expect("Should have been able to read the file");
    let config = yaml::YamlLoader::load_from_str(&contents).unwrap();
    return config;
}

pub fn get_token(token_file: &str) -> String {
    return fs::read_to_string(token_file)
        .expect("Should have been able to read the file");
}