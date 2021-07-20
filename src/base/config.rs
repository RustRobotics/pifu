// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum Arch {
    #[serde(alias = "x86")]
    X86,

    #[serde(alias = "x86-64")]
    X86_64,

    #[serde(alias = "aarch64")]
    AArch64,
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arch::X86 => write!(f, "x86"),
            Arch::X86_64 => write!(f, "x86-64"),
            Arch::AArch64 => write!(f, "aarch64"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobPatterns(Vec<String>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub name: String,
    pub product_name: String,
    pub app_id: String,
    pub description: String,
    pub homepage: String,
    pub author: String,
    pub copyright: Option<String>,
    pub company: Option<String>,
    pub version: String,
    pub build_id: String,
    pub license: String,
    pub license_file: Option<String>,

    pub workdir: String,
    pub src_dir: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum PlatformTarget {
    #[serde(alias = "deb")]
    Deb,

    #[serde(alias = "rpm")]
    Rpm,

    #[serde(alias = "app_image")]
    AppImage,

    /// For windows exe file.
    #[serde(alias = "nsis")]
    Nsis,
}

impl PlatformTarget {
    /// Returns extension name of generated artifcate files.
    pub fn extension(&self) -> &'static str {
        match self {
            PlatformTarget::Deb => "deb",
            PlatformTarget::Rpm => "rpm",
            PlatformTarget::AppImage => "AppImage",
            PlatformTarget::Nsis => "exe",
        }
    }
}
