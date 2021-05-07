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

#[derive(Debug, Deserialize)]
pub struct WindowsConfig {
    #[serde(default = "default_arch")]
    pub arch: Vec<Arch>,

    #[serde(default = "default_windows_targets")]
    pub targets: Vec<WindowsTarget>,

    pub files: Option<Vec<FileSet>>,

    pub nsis: Option<NsisConfig>,
}

fn default_windows_targets() -> Vec<WindowsTarget> {
    vec![WindowsTarget::Nsis]
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum WindowsTarget {
    #[serde(alias = "nsis")]
    Nsis,
}
