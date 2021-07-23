// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::base::GlobPatterns;
use crate::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileSet {
    pub from: String,
    pub to: String,
    pub filter: Option<GlobPatterns>,
    pub mode: Option<i32>,
}

impl FileSet {
    pub fn copy_to(&self, src: &str, dest: &Path) -> Result<(), Error> {
        let dest_path = dest.join(&self.to);
        let dest_dir = dest_path.parent().unwrap();
        fs::create_dir_all(dest_dir)?;
        let src_pattern = format!("{}/{}", src, &self.from);
        for entry in glob::glob(&src_pattern)? {
            let entry = entry?;
            fs::copy(&entry, &dest_path)?;
        }
        Ok(())
    }
}

pub fn copy_filesets(files: &[FileSet], src: &str, dest: &Path) -> Result<(), Error> {
    for ref file in files {
        file.copy_to(src, dest)?;
    }

    Ok(())
}
