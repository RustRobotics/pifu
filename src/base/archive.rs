// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::BuildError;

pub fn create_tar(dir: &Path, to: &Path) -> Result<(), BuildError> {
    log::info!("tar {:?} > {:?}", dir, to);
    let to_file = File::create(to)?;
    let mut builder = tar::Builder::new(to_file);
    builder.append_dir_all(".", dir)?;
    builder.finish()?;

    Ok(())
}

pub fn create_ar(dir: &Path, to: &Path) -> Result<(), BuildError> {
    log::info!("ar {:?} > {:?}", dir, to);
    let to_file = File::create(to)?;
    let mut builder = ar::Builder::new(to_file);

    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            builder.append_path(path)?;
        }
    }

    Ok(())
}

pub fn create_ar_files<P: AsRef<Path>>(files: &[&P], to: &Path) -> Result<(), BuildError> {
    let to_file = File::create(to)?;
    let mut builder = ar::Builder::new(to_file);

    for path in files {
        let path = path.as_ref();
        if path.is_file() {
            builder.append_path(path)?;
        }
    }

    Ok(())
}
