use yaml_rust2::{Yaml, YamlLoader};
use std::fs;
use std::error::Error;

pub fn load_config(filename: &str) -> Result<Vec<Yaml>, Box<dyn Error>> {
    let source = fs::read_to_string(filename)?;
    let conf = YamlLoader::load_from_str(&source)?;
    Ok(conf)
}
