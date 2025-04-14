use std::fs;
use toml::Value;

pub fn load_node_config(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Value = toml::from_str(&content)?;
    Ok(config)
}
