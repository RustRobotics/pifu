// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::{Deserialize, Serialize};

use crate::app_image::AppImageConfig;
use crate::base::fileset::FileSet;
use crate::base::{Arch, Metadata, PlatformTarget};
use crate::deb::DebConfig;
use crate::nsis::NsisConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub metadata: Metadata,

    pub windows: Option<WindowsConfig>,

    pub linux: Option<LinuxConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LinuxConfig {
    #[serde(default = "default_arch")]
    pub arch: Vec<Arch>,

    #[serde(default = "default_linux_targets")]
    pub targets: Vec<PlatformTarget>,

    pub files: Option<Vec<FileSet>>,

    /// Specific config for Deb format.
    pub deb: Option<DebConfig>,

    /// Specific config for AppImage format.
    pub app_image: Option<AppImageConfig>,
}

fn default_arch() -> Vec<Arch> {
    vec![Arch::X86_64]
}

fn default_linux_targets() -> Vec<PlatformTarget> {
    vec![PlatformTarget::Deb, PlatformTarget::AppImage]
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindowsConfig {
    #[serde(default = "default_arch")]
    pub arch: Vec<Arch>,

    #[serde(default = "default_windows_targets")]
    pub targets: Vec<PlatformTarget>,

    /// String - file to be run after install and desktop link refers to.
    /// Relative to install directory.
    pub exe_file: String,

    pub files: Option<Vec<FileSet>>,

    /// Nsis specific config.
    pub nsis: Option<NsisConfig>,
}

fn default_windows_targets() -> Vec<PlatformTarget> {
    vec![PlatformTarget::Nsis]
}
