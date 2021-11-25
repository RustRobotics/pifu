// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum Arch {
    #[serde(alias = "x86")]
    X86,

    #[serde(alias = "x86_64")]
    X86_64,

    #[serde(alias = "aarch64")]
    AArch64,

    #[serde(alias = "mips64")]
    Mips64,
}

pub const fn get_target_arch() -> Option<Arch> {
    if cfg!(target_arch = "x86") {
        return Some(Arch::X86);
    }
    if cfg!(target_arch = "x86_64") {
        return Some(Arch::X86_64);
    }
    if cfg!(target_arch = "aarch64") {
        return Some(Arch::AArch64);
    }
    if cfg!(target_arch = "mips64") {
        return Some(Arch::Mips64);
    }
    return None;
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arch::X86 => write!(f, "x86"),
            Arch::X86_64 => write!(f, "x86_64"),
            Arch::AArch64 => write!(f, "aarch64"),
            Arch::Mips64 => write!(f, "mips64"),
        }
    }
}

impl FromStr for Arch {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "x86" => Ok(Arch::X86),
            "i386" => Ok(Arch::X86),
            "i686" => Ok(Arch::X86),
            "x86_64" => Ok(Arch::X86_64),
            "amd64" => Ok(Arch::X86_64),
            "arm64" => Ok(Arch::AArch64),
            "aarch64" => Ok(Arch::AArch64),
            "mips64" => Ok(Arch::Mips64),
            _ => Err(()),
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

impl FromStr for PlatformTarget {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "deb" => Ok(PlatformTarget::Deb),
            "rpm" => Ok(PlatformTarget::Rpm),
            "app_image" => Ok(PlatformTarget::AppImage),
            "appImage" => Ok(PlatformTarget::AppImage),
            "AppImage" => Ok(PlatformTarget::AppImage),
            "nsis" => Ok(PlatformTarget::Nsis),
            _ => Err(()),
        }
    }
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
