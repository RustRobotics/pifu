#[derive(Debug, Deserialize)]
pub struct Config {
    pub metadata: Metadata,
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
