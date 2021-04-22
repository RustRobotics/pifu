// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;

use crate::base::fileset::FileSet;
use crate::base::{Arch, Metadata};
use crate::deb::DebConfig;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub metadata: Metadata,

    pub linux: Option<LinuxConfig>,
}

#[derive(Debug, Deserialize)]
pub struct LinuxConfig {
    #[serde(default = "default_arch")]
    pub arch: Vec<Arch>,

    #[serde(default = "default_linux_targets")]
    pub targets: Vec<LinuxTarget>,

    pub files: Option<Vec<FileSet>>,

    pub deb: Option<DebConfig>,
}

fn default_arch() -> Vec<Arch> {
    vec![Arch::X86_64]
}

fn default_linux_targets() -> Vec<LinuxTarget> {
    vec![LinuxTarget::Deb]
}

#[derive(Debug, Deserialize, PartialEq)]
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
