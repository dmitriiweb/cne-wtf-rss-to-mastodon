use serde_derive::Deserialize;
use std::{error::Error, fs};
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mastodon_token: String,
    pub mastodon_url: String,
    pub max_post_len: i32,
    pub rss_url: String,
    pub saved_urls_file: String,
}

impl Config {
    pub fn new(config_file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read_to_string(config_file_path)?;
        let config: Config = toml::from_str(&file_content)?;
        Ok(config)
    }
}
