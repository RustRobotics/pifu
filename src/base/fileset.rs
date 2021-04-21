// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::path::Path;

use crate::base::GlobPatterns;
use crate::BuildError;

#[derive(Debug, Deserialize)]
pub struct FileSet {
    pub from: String,
    pub to: String,
    pub filter: Option<GlobPatterns>,
    pub mode: Option<i32>,
}

impl FileSet {
    pub fn copy_to(&self, src: &Path, dest: &Path) -> Result<(), BuildError> {
        Ok(())
    }
}

pub fn copy_filesets(files: &[FileSet], src: &Path, dest: &Path) -> Result<(), BuildError> {
    for ref file in files {
        file.copy_to(src, dest)?;
    }

    Ok(())
}
