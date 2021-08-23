// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::app_image::AppImageConfig;
use crate::base::fileset::FileSet;
use crate::base::{Arch, Metadata, PlatformTarget};
use crate::deb::DebConfig;
use crate::error::{Error, ErrorKind};
use crate::nsis::NsisConfig;
use crate::rpm::RpmConfig;

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

    /// Specific config for AppImage format.
    #[serde(default = "AppImageConfig::default")]
    pub app_image: AppImageConfig,

    /// Specific config for Deb format.
    #[serde(default = "DebConfig::default")]
    pub deb: DebConfig,

    /// Specific config for Rpm format.
    #[serde(default = "RpmConfig::default")]
    pub rpm: RpmConfig,
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

pub fn get_project_dir() -> Result<ProjectDirs, Error> {
    match ProjectDirs::from("org", "biofan", "pifu") {
        Some(dir) => Ok(dir),
        None => Err(Error::new(ErrorKind::HomeDirError, "Invalid $HOME")),
    }
}

pub fn get_binary_dir() -> Result<PathBuf, Error> {
    let project_dir = get_project_dir()?;
    let conf_dir = project_dir.config_dir();
    Ok(conf_dir.join("bin"))
}
