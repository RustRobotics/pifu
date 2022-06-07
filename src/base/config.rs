// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde::{Deserialize, Serialize};
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

#[must_use]
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
    None
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X86 => write!(f, "x86"),
            Self::X86_64 => write!(f, "x86_64"),
            Self::AArch64 => write!(f, "aarch64"),
            Self::Mips64 => write!(f, "mips64"),
        }
    }
}

impl FromStr for Arch {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "x86" | "i386" | "i686" => Ok(Self::X86),
            "x86_64" | "amd64" => Ok(Self::X86_64),
            "arm64" | "aarch64" => Ok(Self::AArch64),
            "mips64" => Ok(Self::Mips64),
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
            "deb" => Ok(Self::Deb),
            "rpm" => Ok(Self::Rpm),
            "app_image" | "appImage" | "AppImage" => Ok(Self::AppImage),
            "nsis" => Ok(Self::Nsis),
            _ => Err(()),
        }
    }
}

impl PlatformTarget {
    /// Returns extension name of generated artifcate files.
    #[must_use]
    pub const fn extension(&self) -> &'static str {
        match self {
            Self::Deb => "deb",
            Self::Rpm => "rpm",
            Self::AppImage => "AppImage",
            Self::Nsis => "exe",
        }
    }
}
