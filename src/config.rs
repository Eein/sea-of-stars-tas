use std::error::Error;
use std::fs;
use yaml_rust2::{Yaml, YamlLoader};

pub fn load_config(filename: &str) -> Result<Vec<Yaml>, Box<dyn Error>> {
    let source = fs::read_to_string(filename)?;
    let conf = YamlLoader::load_from_str(&source)?;
    Ok(conf)
}
