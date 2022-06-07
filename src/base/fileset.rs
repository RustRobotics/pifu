// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::base::GlobPatterns;
use crate::error::{Error, ErrorKind};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileSet {
    pub from: String,
    pub to: String,
    pub filter: Option<GlobPatterns>,
    pub mode: Option<i32>,
}

impl FileSet {
    pub fn copy_to(&self, src: &str, dest: &Path) -> Result<(), Error> {
        log::info!("FileSet::copy_to() src: {:?}, dest: {:?}", src, dest);
        let dest_path = dest.join(&self.to);
        let dest_dir = dest_path.parent().ok_or(Error::from_string(
            ErrorKind::IoError,
            format!("Failed to get parent dir of dest_path: {:?}", dest_path),
        ))?;
        fs::create_dir_all(dest_dir).map_err(|err| {
            Error::from_string(
                ErrorKind::IoError,
                format!(
                    "Failed to create directory `{:?}`, error: {:?}",
                    dest_dir, err
                ),
            )
        })?;
        let src_pattern = format!("{}/{}", src, &self.from);
        let entries = glob::glob(&src_pattern).map_err(|err| {
            Error::from_string(
                ErrorKind::GlobPatternError,
                format!(
                    "Failed to crate glob pattern for {:?}, error: {:?}",
                    &src_pattern, err
                ),
            )
        })?;
        let mut entry_not_match = true;
        for entry in entries {
            entry_not_match = false;
            let entry = entry?;
            let metadata = fs::metadata(&entry)?;
            if metadata.is_file() {
                fs::copy(&entry, &dest_path).map_err(|err| {
                    Error::from_string(
                        ErrorKind::IoError,
                        format!(
                            "Failed to copy file from `{:?}` to `{:?}`, err: {:?}",
                            &entry, &dest_path, err
                        ),
                    )
                })?;
            } else if metadata.is_dir() {
                // `dest_path` must be a directory.
                fs::create_dir_all(&dest_path).map_err(|err| {
                    Error::from_string(
                        ErrorKind::IoError,
                        format!(
                            "Failed to create directory `{:?}`, error: {:?}",
                            &dest_path, err
                        ),
                    )
                })?;

                let mut options = fs_extra::dir::CopyOptions::new();
                options.overwrite = true;
                options.copy_inside = true;
                fs_extra::dir::copy(&entry, &dest_path, &options).map_err(|err| {
                    Error::from_string(
                        ErrorKind::IoError,
                        format!(
                            "Failed to copy folder from `{:?}` to `{:?}`, err: {:?}",
                            &entry, &dest_path, err
                        ),
                    )
                })?;
            } else {
                return Err(Error::from_string(
                    ErrorKind::IoError,
                    format!("Unsupported file type: {:?}", entry),
                ));
            }
        }
        if entry_not_match {
            Err(Error::from_string(
                ErrorKind::GlobError,
                format!("No file is matched with pattern `{}`", src_pattern),
            ))
        } else {
            Ok(())
        }
    }
}

pub fn copy_filesets(files: &[FileSet], src: &str, dest: &Path) -> Result<(), Error> {
    log::info!(
        "copy_filesets() files: {:?}, src: {:?}, dest: {:?}",
        files,
        src,
        dest
    );
    for ref file in files {
        file.copy_to(src, dest)?;
    }

    Ok(())
}
