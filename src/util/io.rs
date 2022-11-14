use std::fs;
use toml;
use crate::Config;

fn get_file_content(file: &str) -> Result<String, &str> {
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");
    Ok(contents)
}

pub fn get_config_from_toml(config_file: &str,) -> Result<Config, &str>{
    let contents = get_file_content(config_file).unwrap();
    let result: Config = toml::from_str(contents.as_str()).unwrap();
    Ok(result)
}

pub fn get_token(token_file: &str) -> String {
    return get_file_content(token_file).unwrap();
}