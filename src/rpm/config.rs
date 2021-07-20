// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::{Deserialize, Serialize};

use crate::base::fileset::FileSet;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct RpmConfig {
    /// File list.
    pub files: Option<Vec<FileSet>>,

    /// Specify additional required packages.
    pub required_pkgs: Option<Vec<String>>,
}
