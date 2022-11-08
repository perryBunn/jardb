use std::fs;
use clap::builder::Str;
use yaml_rust as yaml;
use toml;

fn get_file_content(file: &str) -> Result<String, &str> {
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");
    Ok(contents)
}

pub fn get_config_from_yaml(config_file: &str, index: Option<i32>) -> Result<yaml::Yaml,&str> {
    let contents = get_file_content(config_file).unwrap();
    let result = yaml::YamlLoader::load_from_str(contents.as_str()).unwrap();
    Ok(result[index.unwrap_or(0) as usize].clone())
}

pub fn get_config_from_toml(config_file: &str) -> Result<String, &str>{
    let contents = get_file_content(config_file).unwrap();
    let result = toml::from_str(contents.as_str()).unwrap();
    Ok(result)
}

pub fn get_token(token_file: &str) -> String {
    return get_file_content(token_file).unwrap();
}