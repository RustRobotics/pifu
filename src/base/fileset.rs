// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::Deserialize;
use std::fs;
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
        let src_path = src.join(&self.from);
        let dest_path = dest.join(&self.to);
        let dest_dir = dest_path.parent().unwrap();
        fs::create_dir_all(dest_dir)?;
        log::info!("Copy {:?} > {:?}", src_path, dest_path);
        fs::copy(src_path, dest_path)?;
        Ok(())
    }
}

pub fn copy_filesets(files: &[FileSet], src: &Path, dest: &Path) -> Result<(), BuildError> {
    for ref file in files {
        file.copy_to(src, dest)?;
    }

    Ok(())
}
