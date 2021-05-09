// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;

use crate::base::fileset::FileSet;
use crate::base::{Arch, Metadata};
use crate::deb::DebConfig;
use crate::nsis::NsisConfig;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub metadata: Metadata,

    pub windows: Option<WindowsConfig>,

    pub linux: Option<LinuxConfig>,
}

#[derive(Debug, Deserialize)]
pub struct LinuxConfig {
    #[serde(default = "default_arch")]
    pub arch: Vec<Arch>,

    #[serde(default = "default_linux_targets")]
    pub targets: Vec<PlatformTarget>,

    pub files: Option<Vec<FileSet>>,

    pub deb: Option<DebConfig>,
}

fn default_arch() -> Vec<Arch> {
    vec![Arch::X86_64]
}

fn default_linux_targets() -> Vec<PlatformTarget> {
    vec![PlatformTarget::Deb]
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum PlatformTarget {
    #[serde(alias = "deb")]
    Deb,

    #[serde(alias = "rpm")]
    Rpm,

    #[serde(alias = "app_image")]
    AppImage,

    #[serde(alias = "tar")]
    Tar,

    #[serde(alias = "tgz")]
    TarGz,

    #[serde(alias = "nsis")]
    Nsis,
}

impl PlatformTarget {
    /// Returns extension name of generated artifcate files.
    pub fn extension(&self) -> &'static str {
        match self {
            PlatformTarget::Deb => "deb",
            PlatformTarget::Rpm => "rpm",
            PlatformTarget::AppImage => "app_image",
            PlatformTarget::Tar => "tar",
            PlatformTarget::TarGz => "tgz",
            PlatformTarget::Nsis => "exe",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WindowsConfig {
    #[serde(default = "default_arch")]
    pub arch: Vec<Arch>,

    #[serde(default = "default_windows_targets")]
    pub targets: Vec<PlatformTarget>,

    pub files: Option<Vec<FileSet>>,

    pub nsis: Option<NsisConfig>,
}

fn default_windows_targets() -> Vec<PlatformTarget> {
    vec![PlatformTarget::Nsis]
}
