// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;

use crate::deb::config::DebConfig;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub metadata: Metadata,

    pub linux: Option<LinuxConfig>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub app_id: String,
    pub description: String,
    pub author: String,
    pub copyright: Option<String>,
    pub version: String,
    pub license: String,
    pub license_file: Option<String>,

    pub workdir: String,
}

#[derive(Debug, Deserialize)]
pub struct LinuxConfig {
    pub arch: Vec<String>,
    pub targets: Vec<String>,

    pub deb: Option<DebConfig>,
}
