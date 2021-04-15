use serde_derive::Deserialize;

use crate::deb::config::DebConfig;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub metadata: Metadata,

    pub deb: Option<DebConfig>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub app_id: String,
    pub description: String,
    pub authors: Vec<String>,
    pub copyright: String,
    pub version: String,
    pub license: String,
    pub license_file: Option<String>,

    pub output: String,
}
