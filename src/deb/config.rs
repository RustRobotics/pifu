// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::{Deserialize, Serialize};

use crate::base::fileset::FileSet;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DebConfig {
    #[serde(default = "default_priority")]
    pub priority: String,

    pub section: Option<String>,
    pub depends: Option<String>,
    pub recommends: Option<String>,
    pub conflicts: Option<String>,
    pub breaks: Option<String>,
    pub replaces: Option<String>,
    pub provides: Option<String>,

    pub files: Option<Vec<FileSet>>,
}

fn default_priority() -> String {
    "utility".to_string()
}

impl Default for DebConfig {
    fn default() -> Self {
        DebConfig {
            priority: default_priority(),
            section: None,
            depends: None,
            recommends: None,
            conflicts: None,
            breaks: None,
            replaces: None,
            provides: None,
            files: None,
        }
    }
}
