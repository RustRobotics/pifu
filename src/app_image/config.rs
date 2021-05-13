// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::{Deserialize, Serialize};

use crate::base::fileset::FileSet;
use crate::base::utils::default_true;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppImageConfig {
    /// A list of elf executable files. If they are dynamically linked, dependent
    /// libraries will be copied.
    pub exe_files: Vec<String>,

    /// Boolean - whether to copy dependent libraries.
    /// This shall almost always be true.
    /// Those libraries are copied to AppDir/libs folder.
    #[serde(default = "default_true")]
    pub embed_libs: bool,

    /// File list.
    pub files: Option<Vec<FileSet>>,
    // TODO(Shaohua): Add artifact_name
}
