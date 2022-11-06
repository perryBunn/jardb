use std::fs;
use yaml_rust as yaml;

pub fn get_config(config_file: &str, index: Option<i32>) -> Result<yaml::Yaml,&str> {
    let contents = fs::read_to_string(config_file)
        .expect("Should have been able to read the file");
    let result = yaml::YamlLoader::load_from_str(&contents);
    match result {
        Ok(y) => Ok(y[index.unwrap_or(0) as usize].clone()),
        Err(e) => return Err("Error parsing yaml: {e:?}")
    }
}

pub fn get_token(token_file: &str) -> String {
    return fs::read_to_string(token_file)
        .expect("Should have been able to read the file");
}