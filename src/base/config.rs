// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum Arch {
    #[serde(alias = "x86")]
    X86,

    #[serde(alias = "x86-64")]
    X86_64,

    #[serde(alias = "aarch64")]
    AArch64,
}

#[derive(Debug, Deserialize)]
pub struct FileSet {
    pub from: String,
    pub to: String,
    pub filter: Option<GlobPatterns>,
    pub mode: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GlobPatterns(Vec<String>);
