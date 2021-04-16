// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;

use crate::base::{Arch, FileSet};
use crate::deb::DebConfig;

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

    pub assets: Vec<FileSet>,
    pub workdir: String,
}

#[derive(Debug, Deserialize)]
pub struct LinuxConfig {
    #[serde(default = "default_arch")]
    pub arch: Vec<Arch>,

    #[serde(default = "default_linux_targets")]
    pub targets: Vec<LinuxTarget>,

    pub deb: Option<DebConfig>,
}

fn default_arch() -> Vec<Arch> {
    vec![Arch::X86_64]
}

fn default_linux_targets() -> Vec<LinuxTarget> {
    vec![LinuxTarget::Deb]
}

#[derive(Debug, Deserialize)]
pub enum LinuxTarget {
    #[serde(alias = "deb")]
    Deb,

    #[serde(alias = "rpm")]
    Rpm,

    AppImage,

    #[serde(alias = "tar")]
    Tar,

    #[serde(alias = "tgz")]
    TarGz,
}
