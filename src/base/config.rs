// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, PartialEq)]
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

#[derive(Debug, Deserialize)]
pub struct GlobPatterns(Vec<String>);

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
    pub src_dir: String,
}
